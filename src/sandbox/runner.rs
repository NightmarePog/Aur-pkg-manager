use std::{
    collections::VecDeque,
    ffi::OsString,
    io::{BufRead, BufReader, Read},
    process::{Child, Command, ExitStatus, Stdio},
    sync::mpsc::{self, RecvTimeoutError, Sender},
    time::Duration,
};

use crate::sandbox::SpawnError;

use crate::ui;

const MAX_ACTIVITY_LINES: usize = 4;

pub struct Runner(Child);

impl Runner {
    pub fn spawn(args: Vec<OsString>) -> Result<Self, SpawnError> {
        Command::new("bwrap")
            .args(args)
            .spawn()
            .map(Self)
            .map_err(SpawnError::Io)
    }

    pub fn spawn_quiet(args: Vec<OsString>) -> Result<Self, SpawnError> {
        Command::new("bwrap")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map(Self)
            .map_err(SpawnError::Io)
    }

    pub fn wait(&mut self) -> Result<ExitStatus, SpawnError> {
        self.0.wait().map_err(SpawnError::Io)
    }

    pub fn wait_with_progress(mut self, label: &str) -> Result<(ExitStatus, String), SpawnError> {
        let stdout = self.0.stdout.take().unwrap();
        let stderr = self.0.stderr.take().unwrap();
        let (stderr_tx, stderr_rx) = mpsc::channel();
        let stdout_reader = spawn_line_reader(stdout, stderr_tx.clone());
        let stderr_reader = spawn_line_reader(stderr, stderr_tx);
        let mut full_output = String::new();
        let mut current_status = String::new();
        let mut activity = VecDeque::new();
        let mut progress = ui::Progress::new()?;

        progress.update(label, &current_status, &activity);

        loop {
            match stderr_rx.recv_timeout(Duration::from_millis(120)) {
                Ok(line) => {
                    full_output.push_str(&line);
                    full_output.push('\n');

                    let is_stage = if let Some(status) = parse_makepkg_stage(&line) {
                        current_status = status.to_string();
                        if !is_progress_stage(status) {
                            append_activity(&mut activity, status);
                        }
                        true
                    } else if ui::is_interactive() {
                        if let Some(detail) = summarize_build_line(&line) {
                            append_activity(&mut activity, &detail);
                        }
                        false
                    } else {
                        false
                    };

                    if ui::is_interactive() || is_stage {
                        progress.update(label, &current_status, &activity);
                    }
                }
                Err(RecvTimeoutError::Timeout) if ui::is_interactive() => {
                    progress.update(label, &current_status, &activity);
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }

        stdout_reader.join().ok();
        stderr_reader.join().ok();
        let status = self.0.wait()?;

        progress.finish();

        let diagnostics = if status.success() {
            String::new()
        } else {
            full_output
        };

        Ok((status, diagnostics))
    }
}

fn append_activity(activity: &mut VecDeque<String>, line: &str) {
    if activity.back().is_some_and(|last| last == line) {
        return;
    }

    if activity.len() == MAX_ACTIVITY_LINES {
        activity.pop_front();
    }

    activity.push_back(line.to_owned());
}

fn is_progress_stage(status: &str) -> bool {
    matches!(
        status,
        "validating"
            | "extracting"
            | "resolving version"
            | "preparing"
            | "building"
            | "testing"
            | "packaging"
            | "tidying"
            | "compressing"
            | "generating metadata"
            | "done"
    )
}

fn spawn_line_reader<R>(reader: R, sender: Sender<String>) -> std::thread::JoinHandle<()>
where
    R: Read + Send + 'static,
{
    std::thread::spawn(move || {
        for line in BufReader::new(reader).lines() {
            let Ok(line) = line else {
                break;
            };

            if sender.send(line).is_err() {
                break;
            }
        }
    })
}

fn summarize_build_line(line: &str) -> Option<String> {
    let line = line.trim();

    if line.is_empty() || line.starts_with("==>") {
        return None;
    }

    let mut summary = line.to_owned();
    if summary.chars().count() > 64 {
        summary = summary.chars().take(61).collect();
        summary.push_str("...");
    }

    Some(summary)
}

fn parse_makepkg_stage(line: &str) -> Option<&str> {
    if let Some(rest) = line.strip_prefix("==> Retrieving sources...") {
        if !rest.is_empty() {
            return Some(rest.trim_start());
        }
    }
    if line.starts_with("  -> Downloading ") {
        let file = line.strip_prefix("  -> Downloading ").unwrap_or(line);
        return Some(file);
    }
    if line.starts_with("  -> Found ") {
        let file = line.strip_prefix("  -> Found ").unwrap_or(line);
        return Some(file);
    }
    if line.starts_with("==> Validating source") {
        return Some("validating");
    }
    if line.starts_with("==> Extracting sources") {
        return Some("extracting");
    }
    if line.starts_with("==> Starting pkgver") {
        return Some("resolving version");
    }
    if line.starts_with("==> Starting prepare") {
        return Some("preparing");
    }
    if line.starts_with("==> Starting build") {
        return Some("building");
    }
    if line.starts_with("==> Starting check") {
        return Some("testing");
    }
    if line.starts_with("==> Entering fakeroot") {
        return Some("packaging");
    }
    if line.starts_with("==> Starting package") {
        return Some("packaging");
    }
    if line.starts_with("==> Tidying install") {
        return Some("tidying");
    }
    if line.starts_with("==> Creating package") {
        let pkg = line.strip_prefix("==> Creating package \"").unwrap_or("");
        let pkg = pkg.strip_suffix('"').unwrap_or(pkg);
        return Some(pkg);
    }
    if line.starts_with("==> Compressing package") {
        return Some("compressing");
    }
    if line.starts_with("==> Generating .PKGINFO") {
        return Some("generating metadata");
    }
    if line.starts_with("==> Generating .BUILDINFO") {
        return Some("generating metadata");
    }
    if line.starts_with("==> Generating .MTREE") {
        return Some("generating metadata");
    }
    if line.starts_with("==> Finished making") {
        return Some("done");
    }
    if line.starts_with("==> Making package") {
        return None;
    }

    None
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::sandbox::runner::{append_activity, parse_makepkg_stage, summarize_build_line};

    #[test]
    fn keeps_a_rolling_log_of_recent_build_actions() {
        let mut activity = VecDeque::new();

        for line in [
            "Preparing source tree",
            "Updating crates.io index",
            "Downloading serde",
            "Compiling paru",
            "Finished release profile",
        ] {
            append_activity(&mut activity, line);
        }

        assert_eq!(
            activity.into_iter().collect::<Vec<_>>(),
            vec![
                "Updating crates.io index",
                "Downloading serde",
                "Compiling paru",
                "Finished release profile",
            ]
        );
    }

    #[test]
    fn parses_user_facing_build_stages() {
        let cases = [
            ("==> Validating source files...", Some("validating")),
            ("==> Starting build()...", Some("building")),
            ("==> Starting check()...", Some("testing")),
            ("==> Compressing package...", Some("compressing")),
            ("==> Making package: demo 1.0-1 (Mon Jan 1)", None),
        ];

        for (line, expected) in cases {
            assert_eq!(parse_makepkg_stage(line), expected);
        }
    }

    #[test]
    fn summarizes_compiler_activity_without_makepkg_headers() {
        assert_eq!(
            summarize_build_line("  cc -O2 src/main.c"),
            Some("cc -O2 src/main.c".into())
        );
        assert_eq!(summarize_build_line("==> Starting build()..."), None);
    }
}
