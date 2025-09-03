use std::fmt;

use console::style;

pub fn log(message: impl fmt::Display) {
    println!("{}", message);
}

pub fn ilog(message: impl fmt::Display) {
    println!("{} {}", style(">").cyan().bold(), message);
}

pub fn wlog(message: impl fmt::Display) {
    println!("{} {}", style("!").yellow().bold(), message);
}

pub fn elog(message: impl fmt::Display) {
    println!("{} {}", style("🞪").red().bold(), message);
}

pub fn slog(message: impl fmt::Display) {
    println!("{} {}", style("✓").green().bold(), message);
}
