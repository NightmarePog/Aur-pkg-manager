use std::{io::IsTerminal, iter::once};

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use owo_colors::OwoColorize;

use crate::ui::{INDENT, UiError, step};

pub struct Loading(ProgressBar);

pub fn loading(msg: &str) -> Result<Loading, UiError> {
    Ok(Loading(if is_interactive() {
        ProgressBar::with_draw_target(None, ProgressDrawTarget::stdout())
            .with_style(spinner_style()?)
            .with_message(msg.to_owned())
    } else {
        step(msg);
        ProgressBar::hidden()
    }))
}

impl Loading {
    pub fn set_message(&self, message: String) {
        self.0.set_message(message);
    }
}

impl Drop for Loading {
    fn drop(&mut self) {
        self.0.finish_and_clear();
    }
}

pub struct Progress {
    progress: ProgressBar,
    interactive: bool,
}

impl Progress {
    pub fn new() -> Result<Self, UiError> {
        Ok(Self {
            progress: if is_interactive() {
                ProgressBar::new(100).with_style(progress_style()?)
            } else {
                ProgressBar::hidden()
            },
            interactive: is_interactive(),
        })
    }

    pub fn update(
        &mut self,
        label: &str,
        status: &str,
        activity: impl IntoIterator<Item = impl AsRef<str>>,
    ) {
        let Self {
            progress,
            interactive,
        } = self;

        if *interactive {
            progress.set_prefix(label.bold().to_string());
            progress.set_position(u64::from(stage_completion(status)));
            progress.set_message(
                once(phase(status).dimmed().to_string())
                    .chain(activity.into_iter().map(|line| {
                        format!(
                            "{INDENT}{INDENT}{} {}",
                            "│".dimmed(),
                            line.as_ref().dimmed()
                        )
                    }))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        } else {
            println!("{INDENT}{label} {}", phase(status));
        }
    }

    pub fn finish(&mut self) {
        self.progress.finish();
    }
}

fn phase(status: &str) -> &str {
    status.is_empty().then_some("starting").unwrap_or(status)
}

pub fn is_interactive() -> bool {
    std::io::stdout().is_terminal()
}

fn spinner_style() -> Result<ProgressStyle, UiError> {
    Ok(ProgressStyle::default_spinner().template("{spinner:.yellow} {msg}")?)
}

fn progress_style() -> Result<ProgressStyle, UiError> {
    Ok(ProgressStyle::default_bar()
        .template("{spinner:.yellow} {prefix:.bold} [{bar:10.cyan/blue}] {msg}")?)
}

fn stage_completion(status: &str) -> u8 {
    match status {
        "" => 0,
        "validating" => 20,
        "extracting" => 30,
        "resolving version" => 40,
        "preparing" => 50,
        "building" => 65,
        "testing" => 75,
        "packaging" => 85,
        "tidying" => 90,
        "compressing" => 95,
        "generating metadata" => 97,
        "done" => 100,
        _ => 10,
    }
}
