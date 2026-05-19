#[cfg(feature = "time")]
pub use chrono::Local;
pub use colored::*;

/// Pretty print macro that prints a info message with a green "INFO" prefix.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        #[cfg(feature = "time")]
        {
            let current_time = $crate::Local::now().format("%H:%M:%S");
            println!("{} {:>5} {}", format!("{}", current_time).dimmed(), format!("INFO").green().bold(), format!($($arg)*))
        }
        #[cfg(not(feature = "time"))]
        {
            println!("{:>5} {}", format!("INFO").green().bold(), format!($($arg)*))
        }
    }};
}

/// Pretty print macro that prints a warning message with a yellow "WARN" prefix.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        #[cfg(feature = "time")]
        {
            let current_time = $crate::Local::now().format("%H:%M:%S");
            println!("{} {:>5} {}", format!("{}", current_time).dimmed(), format!("WARN").yellow().bold(), format!($($arg)*))
        }
        #[cfg(not(feature = "time"))]
        {
            println!("{:>5} {}", format!("WARN").yellow().bold(), format!($($arg)*))
        }
    }};
}

/// Pretty print macro that prints an error message with a red "ERROR" prefix.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        #[cfg(feature = "time")]
        {
            let current_time = $crate::Local::now().format("%H:%M:%S");
            eprintln!("{} {:>5} {}", format!("{}", current_time).dimmed(), format!("ERROR").red().bold(), format!($($arg)*))
        }
        #[cfg(not(feature = "time"))]
        {
            eprintln!("{:>5} {}", format!("ERROR").red().bold(), format!($($arg)*))
        }
    }};
}
