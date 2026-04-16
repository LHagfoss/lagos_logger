pub use chrono::Local;
pub use colored::*;

/// Pretty print macro that prints a info message with a green "INFO" prefix.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        let current_time = $crate::Local::now().format("%H:%M:%S");

        println!("{} {:>5} {}", format!("{}", current_time).dimmed(), format!("INFO").green().bold(), format!($($arg)*))
    }};
}

/// Pretty print macro that prints a warning message with a yellow "WARN" prefix.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        let current_time = $crate::Local::now().format("%H:%M:%S");

        println!("{} {:>5} {}", format!("{}", current_time).dimmed(), format!("WARN").yellow().bold(), format!($($arg)*))
    }};
}

/// Pretty print macro that prints an error message with a red "ERROR" prefix.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let current_time = $crate::Local::now().format("%H:%M:%S");

        eprintln!("{} {:>5} {}", format!("{}", current_time).dimmed(), format!("ERROR").red().bold(), format!($($arg)*))
    }};
}
