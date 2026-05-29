#[cfg(feature = "time")]
pub use chrono::Local;
pub use colored::*;

pub enum Level {
    Info,
    Warn,
    Error,
    Running,
    Success,
}

/// A pretty, colorful logger macro.
/// 
/// # Usage
/// 
/// ```rust
/// use lagos_logger::logger;
/// 
/// // Defaults to Level::Info:
/// logger!("Hello, world!");
/// logger!("Doing a task: {}", 42);
/// 
/// // Or explicitly provide a Level:
/// use lagos_logger::Level;
/// logger!(Level::Warn, "This is a warning!");
/// ```
#[cfg(feature = "time")]
#[macro_export]
macro_rules! logger {
    ($fmt:literal, $($arg:tt)*) => {{
        $crate::logger!($crate::Level::Info, $fmt, $($arg)*)
    }};
    ($fmt:literal) => {{
        $crate::logger!($crate::Level::Info, $fmt)
    }};
    ($level:expr, $($arg:tt)*) => {{
        use $crate::Colorize;
        let time = $crate::Local::now().format("%Y-%m-%d %H:%M:%S");

        match $level {
            $crate::Level::Info => println!("{} {} {}", "Info".bright_green().bold(), time, format_args!($($arg)*)),
            $crate::Level::Warn => println!("{} {} {}", "Warn".bright_yellow().bold(), time, format_args!($($arg)*)),
            $crate::Level::Error => eprintln!("{} {} {}", "Error".bright_red().bold(), time, format_args!($($arg)*)),
            $crate::Level::Running => println!("{} {} {}", "Running".bright_green().bold(), time, format_args!($($arg)*)),
            $crate::Level::Success => println!("{} {} {}", "Success".bright_green().bold(), time, format_args!($($arg)*)),
        }
    }};
}

/// A pretty, colorful logger macro.
/// 
/// # Usage
/// 
/// ```rust
/// use lagos_logger::logger;
/// 
/// // Defaults to Level::Info:
/// logger!("Hello, world!");
/// logger!("Doing a task: {}", 42);
/// 
/// // Or explicitly provide a Level:
/// use lagos_logger::Level;
/// logger!(Level::Warn, "This is a warning!");
/// ```
#[cfg(not(feature = "time"))]
#[macro_export]
macro_rules! logger {
    ($fmt:literal, $($arg:tt)*) => {{
        $crate::logger!($crate::Level::Info, $fmt, $($arg)*)
    }};
    ($fmt:literal) => {{
        $crate::logger!($crate::Level::Info, $fmt)
    }};
    ($level:expr, $($arg:tt)*) => {{
        use $crate::Colorize;
        match $level {
            $crate::Level::Info => println!("{:>12} {}", "Info".bright_green().bold(), format_args!($($arg)*)),
            $crate::Level::Warn => println!("{:>12} {}", "Warn".bright_yellow().bold(), format_args!($($arg)*)),
            $crate::Level::Error => eprintln!("{:>12} {}", "Error".bright_red().bold(), format_args!($($arg)*)),
            $crate::Level::Running => println!("{:>12} {}", "Running".bright_green().bold(), format_args!($($arg)*)),
            $crate::Level::Success => println!("{:>12} {}", "Success".bright_green().bold(), format_args!($($arg)*)),
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::{logger, Level};

    #[test]
    fn test_logger() {
        logger!("This is a test info message");
        logger!("This is a test info message with arg: {}", "hello");
        logger!(Level::Info, "This is an explicit info message");
        logger!(Level::Warn, "This is a warn message: {}", 42);
    }
}
