#[cfg(feature = "time")]
pub use chrono::Local;
pub use colored::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Info,
    Warn,
    Error,
    Running,
    Success,
    User,
    Account,
    Card,
    Transaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubLevel {
    None,
    User,
    Account,
    Card,
    Transaction,
}

/// The core logging implementation that formats and pads the message dynamically.
/// It behaves exactly like standard Axum/Tower logging by outputting single long lines per log entry,
/// while still elegantly aligning subsequent lines if the message contains explicit newlines (\n).
pub fn log_message(level: Level, sub_level: SubLevel, msg: &str) {
    use colored::Colorize;

    let level_str = match level {
        Level::Info => "Info",
        Level::Warn => "Warn",
        Level::Error => "Error",
        Level::Running => "Running",
        Level::Success => "Success",
        Level::User => "User",
        Level::Account => "Account",
        Level::Card => "Card",
        Level::Transaction => "Tx",
    };

    let level_colored = match level {
        Level::Info | Level::Running | Level::Success => level_str.bright_green().bold(),
        Level::Warn => level_str.bright_yellow().bold(),
        Level::Error => level_str.bright_red().bold(),
        Level::User => level_str.bright_blue().bold(),
        Level::Account => level_str.bright_cyan().bold(),
        Level::Card => level_str.bright_magenta().bold(),
        Level::Transaction => level_str.bright_purple().bold(),
    };


    #[cfg(feature = "sublevel")]
    let sub_level_colored = match sub_level {
        SubLevel::None => "".clear(),
        SubLevel::User => "User".bright_blue().bold(),
        SubLevel::Account => "Account".bright_blue().bold(),
        SubLevel::Card => "Card".bright_blue().bold(),
        SubLevel::Transaction => "Tx".bright_blue().bold(),
    };

    #[cfg(not(feature = "sublevel"))]
    let _ = sub_level;

    #[cfg(feature = "time")]
    let time_str = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Calculate prefix length for dynamic wrapping alignment
    #[cfg(all(feature = "time", feature = "sublevel"))]
    let prefix_len = time_str.len() + 1 + 17 + 1;

    #[cfg(all(feature = "time", not(feature = "sublevel")))]
    let prefix_len = time_str.len() + 1 + 8 + 1;

    #[cfg(all(not(feature = "time"), feature = "sublevel"))]
    let prefix_len = 22;

    #[cfg(all(not(feature = "time"), not(feature = "sublevel")))]
    let prefix_len = 13;

    let mut lines = msg.lines();
    if let Some(first) = lines.next() {
        #[cfg(all(feature = "time", feature = "sublevel"))]
        let mut output = format!("{} {:>8} {:<8} {}", time_str, level_colored, sub_level_colored, first);

        #[cfg(all(feature = "time", not(feature = "sublevel")))]
        let mut output = format!("{} {:>8} {}", time_str, level_colored, first);

        #[cfg(all(not(feature = "time"), feature = "sublevel"))]
        let mut output = format!("{:>12} {:<8} {}", level_colored, sub_level_colored, first);

        #[cfg(all(not(feature = "time"), not(feature = "sublevel")))]
        let mut output = format!("{:>12} {}", level_colored, first);

        let indent = " ".repeat(prefix_len);
        for line in lines {
            output.push_str("\n");
            output.push_str(&indent);
            output.push_str(line);
        }

        if level == Level::Error {
            eprintln!("{}", output);
        } else {
            println!("{}", output);
        }
    }
}

/// A pretty, colorful logger macro with optional sub-levels.
#[macro_export]
macro_rules! logger {
    // Pattern 1: logger!(Level, SubLevel, fmt, args...)
    ($level:expr, $sub_level:expr, $fmt:literal, $($arg:tt)*) => {{
        $crate::log_message($level, $sub_level, &format!($fmt, $($arg)*))
    }};
    ($level:expr, $sub_level:expr, $fmt:literal) => {{
        $crate::log_message($level, $sub_level, $fmt)
    }};

    // Pattern 2: logger!(Level, fmt, args...)
    ($level:expr, $fmt:literal, $($arg:tt)*) => {{
        $crate::log_message($level, $crate::SubLevel::None, &format!($fmt, $($arg)*))
    }};
    ($level:expr, $fmt:literal) => {{
        $crate::log_message($level, $crate::SubLevel::None, $fmt)
    }};

    // Pattern 3: logger!(fmt, args...)
    ($fmt:literal, $($arg:tt)*) => {{
        $crate::log_message($crate::Level::Info, $crate::SubLevel::None, &format!($fmt, $($arg)*))
    }};
    ($fmt:literal) => {{
        $crate::log_message($crate::Level::Info, $crate::SubLevel::None, $fmt)
    }};
}

#[cfg(test)]
mod tests {
    use crate::{Level, SubLevel};

    #[test]
    fn test_logger() {
        logger!("This is a test info message");
        logger!(Level::Info, SubLevel::Account, "This is an explicit info message with Account sublevel");
        logger!(Level::Error, SubLevel::User, "This is an error message with User sublevel");
    }
}
