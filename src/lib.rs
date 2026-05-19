#[cfg(feature = "time")]
pub use chrono::Local;
pub use colored::*;

// =========================================
// MACROS WITH time (feature enabled)
// =========================================
#[cfg(feature = "time")]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        let current_time = $crate::Local::now().format("%H:%M:%S");
        println!("{} {:>5} {}", format!("{}", current_time).dimmed(), format!("INFO").green().bold(), format!($($arg)*))
    }};
}

#[cfg(feature = "time")]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        let current_time = $crate::Local::now().format("%H:%M:%S");
        println!("{} {:>5} {}", format!("{}", current_time).dimmed(), format!("WARN").yellow().bold(), format!($($arg)*))
    }};
}

#[cfg(feature = "time")]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let current_time = $crate::Local::now().format("%H:%M:%S");
        eprintln!("{} {:>5} {}", format!("{}", current_time).dimmed(), format!("ERROR").red().bold(), format!($($arg)*))
    }};
}

// MACROS WITHOUT TIME (feature disabled)
#[cfg(not(feature = "time"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        println!("{:>5} {}", format!("INFO").green().bold(), format!($($arg)*))
    }};
}

#[cfg(not(feature = "time"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        println!("{:>5} {}", format!("WARN").yellow().bold(), format!($($arg)*))
    }};
}

#[cfg(not(feature = "time"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        eprintln!("{:>5} {}", format!("ERROR").red().bold(), format!($($arg)*))
    }};
}
