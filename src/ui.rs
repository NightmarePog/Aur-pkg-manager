use std::{io::{self, Write}, time::{SystemTime, UNIX_EPOCH}};

use comfy_table::{
    presets::UTF8_FULL,
    Cell, Color, Table,
};
use owo_colors::OwoColorize;

use crate::dependency::{
    AurMeta,
    InstallPlan,
    PackageSource,
};

const INDENT: &str = "    ";

pub fn info(msg: impl AsRef<str>) {
    println!("{} {}", format!("{INDENT}[INFO]").blue(), msg.as_ref());
}

pub fn success(msg: impl AsRef<str>) {
    println!("{} {}", format!("{INDENT}[OK]").green(), msg.as_ref());
}

pub fn warn(msg: impl AsRef<str>) {
    println!("{} {}", format!("{INDENT}[WARN]").yellow(), msg.as_ref());
}

pub fn error(msg: impl std::fmt::Display) {
    eprintln!("{} {}", format!("{INDENT}[ERROR]").red(), msg);
}

pub fn step(msg: impl AsRef<str>) {
    println!("{} {}", "==>".cyan(), msg.as_ref());
}

pub fn debug(msg: impl AsRef<str>) {
    println!("{} {}", format!("{INDENT}[DEBUG]").dimmed(), msg.as_ref());
}

pub fn header(msg: impl AsRef<str>) {
    println!("\n{}", msg.as_ref().bold());
}

pub fn install_plan(plan: &InstallPlan) {
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .set_header(vec![
            "Package",
            "Version",
            "Source",
            "Maintainer",
            "Votes",
            "Download",
            "Size",
        ]);

    for package in &plan.packages {
        let mut source_cell = Cell::new(match package.source {
            PackageSource::Repo => "repo",
            PackageSource::Aur => "aur",
            PackageSource::Installed => "installed",
        });

        match package.source {
            PackageSource::Repo => {
                source_cell = source_cell.fg(Color::Green);
            }
            PackageSource::Aur => {
                source_cell = source_cell.fg(Color::DarkBlue);
            }
            PackageSource::Installed => {
                source_cell = source_cell.fg(Color::DarkGrey);
            }
        }

        let version = package
            .version
            .as_deref()
            .unwrap_or("?")
            .to_string();

        let size = package
            .size
            .map(format_size)
            .unwrap_or_else(|| "-".into());

        let download_size = package
            .download_size
            .map(format_size)
            .unwrap_or_else(|| "-".into());

        table.add_row(vec![
            Cell::new(&package.name),
            Cell::new(version),
            source_cell,
            maintainer_cell(package.aur.as_ref()),
            Cell::new(
                package
                    .aur
                    .as_ref()
                    .map(|aur| aur.votes.to_string())
                    .unwrap_or_else(|| "-".into()),
            ),
            Cell::new(download_size),
            Cell::new(size),
        ]);
    }

    println!("{table}");
}

fn maintainer_cell(aur: Option<&AurMeta>) -> Cell {
    match aur {
        None => Cell::new("-"),
        Some(aur) => match &aur.maintainer {
            Some(maintainer) => Cell::new(maintainer),
            None => Cell::new("orphan").fg(Color::Red),
        },
    }
}

pub fn aur_details(plan: &InstallPlan) {
    let packages = plan
        .packages
        .iter()
        .filter_map(|package| {
            package.aur.as_ref().map(|aur| (&package.name, aur))
        });

    for (name, aur) in packages {
        println!("\n  {}", name.bold());

        if let Some(description) = &aur.description {
            println!("{INDENT}{INDENT}{description}");
        }

        detail("base", &aur.base);
        detail(
            "maintainer",
            aur.maintainer.as_deref().unwrap_or("orphan"),
        );

        if let Some(submitter) = &aur.submitter {
            detail("submitter", submitter);
        }

        detail(
            "votes",
            format!("{} (popularity {:.2})", aur.votes, aur.popularity),
        );
        detail("updated", relative_time(aur.last_modified));

        if let Some(flagged) = aur.out_of_date {
            detail(
                "flagged",
                format!("out of date {}", relative_time(flagged))
                    .red()
                    .to_string(),
            );
        }

        if let Some(url) = &aur.url {
            detail("url", url);
        }
    }

    println!();
}

fn detail(label: &str, value: impl AsRef<str>) {
    println!(
        "{INDENT}{INDENT}{:<12} {}",
        label.dimmed(),
        value.as_ref()
    );
}

pub fn relative_time(timestamp: i64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs() as i64)
        .unwrap_or_default();

    let seconds = now - timestamp;

    if seconds < 0 {
        return "just now".into();
    }

    const MINUTE: i64 = 60;
    const HOUR: i64 = MINUTE * 60;
    const DAY: i64 = HOUR * 24;
    const MONTH: i64 = DAY * 30;
    const YEAR: i64 = DAY * 365;

    let (value, unit) = match seconds {
        ..MINUTE => return "just now".into(),
        ..HOUR => (seconds / MINUTE, "minute"),
        ..DAY => (seconds / HOUR, "hour"),
        ..MONTH => (seconds / DAY, "day"),
        ..YEAR => (seconds / MONTH, "month"),
        _ => (seconds / YEAR, "year"),
    };

    let plural = if value == 1 { "" } else { "s" };

    format!("{value} {unit}{plural} ago")
}

fn format_size(size: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let size = size as f64;

    if size >= GB {
        format!("{:.1} GiB", size / GB)
    } else if size >= MB {
        format!("{:.1} MiB", size / MB)
    } else if size >= KB {
        format!("{:.1} KiB", size / KB)
    } else {
        format!("{} B", size)
    }
}

pub fn prompt() -> String {
    print!("{} ", "==>".cyan());

    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
