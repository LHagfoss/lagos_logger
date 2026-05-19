#[cfg(feature = "time")]
pub use chrono::Local;
pub use colored::*;

// MACRO WITH TIME (Feature enabled)
#[cfg(feature = "time")]
#[macro_export]
macro_rules! logger {
    (Info, $($arg:tt)*) => {{
        let time = $crate::Local::now().format("%H:%M:%S");
        println!("{} {:>7} {}", format!("{}", time).dimmed(), "INFO".green().bold(), format!($($arg)*))
    }};
    (Warn, $($arg:tt)*) => {{
        let time = $crate::Local::now().format("%H:%M:%S");
        println!("{} {:>7} {}", format!("{}", time).dimmed(), "WARN".yellow().bold(), format!($($arg)*))
    }};
    (Error, $($arg:tt)*) => {{
        let time = $crate::Local::now().format("%H:%M:%S");
        eprintln!("{} {:>7} {}", format!("{}", time).dimmed(), "ERROR".red().bold(), format!($($arg)*))
    }};
    (Running, $($arg:tt)*) => {{
        let time = $crate::Local::now().format("%H:%M:%S");
        println!("{} {:>7} {}", format!("{}", time).dimmed(), "RUNNING".green().bold(), format!($($arg)*))
    }};
    (Success, $($arg:tt)*) => {{
        let time = $crate::Local::now().format("%H:%M:%S");
        println!("{} {:>7} {}", format!("{}", time).dimmed(), "SUCCESS".green().bold(), format!($($arg)*))
    }};
}

// MACRO WITHOUT TIME (Feature disabled)
#[cfg(not(feature = "time"))]
#[macro_export]
macro_rules! logger {
    (Info, $($arg:tt)*) => {{
        println!("{:>7} {}", "INFO".green().bold(), format!($($arg)*))
    }};
    (Warn, $($arg:tt)*) => {{
        println!("{:>7} {}", "WARN".yellow().bold(), format!($($arg)*))
    }};
    (Error, $($arg:tt)*) => {{
        eprintln!("{:>7} {}", "ERROR".red().bold(), format!($($arg)*))
    }};
    (Running, $($arg:tt)*) => {{
        println!("{:>7} {}", "RUNNING".green().bold(), format!($($arg)*))
    }};
    (Success, $($arg:tt)*) => {{
        println!("{:>7} {}", "SUCCESS".green().bold(), format!($($arg)*))
    }};
}
