//! Simple logging module for the crate.

/// Logs an info message.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        #[cfg(feature = "std")]
        {
            eprintln!("[INFO] {}", format!($($arg)*));
        }
        #[cfg(not(feature = "std"))]
        {
            // In no_std, we can't print, but we can still compile
            let _ = format!($($arg)*);
        }
    }
}

/// Logs a debug message.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "std")]
        {
            eprintln!("[DEBUG] {}", format!($($arg)*));
        }
        #[cfg(not(feature = "std"))]
        {
            let _ = format!($($arg)*);
        }
    }
}

/// Logs a warning message.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        #[cfg(feature = "std")]
        {
            eprintln!("[WARN] {}", format!($($arg)*));
        }
        #[cfg(not(feature = "std"))]
        {
            let _ = format!($($arg)*);
        }
    }
}

/// Logs an error message.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature = "std")]
        {
            eprintln!("[ERROR] {}", format!($($arg)*));
        }
        #[cfg(not(feature = "std"))]
        {
            let _ = format!($($arg)*);
        }
    }
}