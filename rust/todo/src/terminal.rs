// ===========================
// Imports
// ===========================

use std::{thread, time};

// ===========================
// Delay Utility
// ===========================

/// Sleeps the current thread for the given number of milliseconds.
///
/// # Arguments
/// * `time` - The amount of time to sleep in milliseconds.
pub fn delay(time: u64) {
    thread::sleep(time::Duration::from_millis(time));
}

// ===========================
// Terminal Clearing Utility
// ===========================

/// Clears the terminal screen based on the current platform (Windows or Unix-like).
pub fn clear() {
    #[cfg(windows)]
    {
        // On Windows, use 'cls' command
        std::process::Command::new("cls").status().unwrap();
    }

    #[cfg(not(windows))]
    {
        // On Unix-based systems (Linux, macOS), use 'clear' command
        std::process::Command::new("clear").status().unwrap();
    }

    // Flush stdout to make sure the terminal is actually cleared immediately
    use std::io::{Write, stdout};
    stdout().flush().unwrap();
}
