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

#[cfg(feature = "time")]
#[macro_export]
macro_rules! logger {
    ($level:expr, $($arg:tt)*) => {{
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

#[cfg(not(feature = "time"))]
#[macro_export]
macro_rules! logger {
    ($level:expr, $($arg:tt)*) => {{
        match $level {
            $crate::Level::Info => println!("{:>12} {}", "Info".bright_green().bold(), format_args!($($arg)*)),
            $crate::Level::Warn => println!("{:>12} {}", "Warn".bright_yellow().bold(), format_args!($($arg)*)),
            $crate::Level::Error => eprintln!("{:>12} {}", "Error".bright_red().bold(), format_args!($($arg)*)),
            $crate::Level::Running => println!("{:>12} {}", "Running".bright_green().bold(), format_args!($($arg)*)),
            $crate::Level::Success => println!("{:>12} {}", "Success".bright_green().bold(), format_args!($($arg)*)),
        }
    }};
}
