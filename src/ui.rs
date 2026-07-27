use owo_colors::OwoColorize;

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

pub fn error(msg: impl AsRef<str>) {
    eprintln!("{} {}", format!("{INDENT}[ERROR]").red(), msg.as_ref());
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
