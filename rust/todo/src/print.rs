// ===========================
// Imports & Constants
// ===========================

use crate::terminal::*;

/// Delay time in milliseconds used after printing messages.
const TIME: u64 = 1000;

// ===========================
// Styled Print Functions
// ===========================

/// Print a success message in green, then clear the terminal after a delay.
///
/// # Arguments
/// * `message` - The success message to display.
pub fn print_success(message: &str) {
    clear();
    println!("\x1b[1;32m{}\x1b[0m", message); // Bright green text
    delay(TIME);
    clear();
}

/// Print a general informational message in blue, then delay.
///
/// # Arguments
/// * `message` - The informational message to display.
pub fn print_message(message: &str) {
    clear();
    println!("\x1b[1;34m{}\x1b[0m", message); // Bright blue text
    delay(TIME);
}

/// Print an inline informational prompt in blue (no newline, no delay).
///
/// # Arguments
/// * `message` - The prompt message to display.
pub fn print_info(message: &str) {
    print!("\x1b[1;34m{}\x1b[0m", message); // Bright blue text
}

/// Print an inline continue-style message in green (no newline, no delay).
///
/// # Arguments
/// * `message` - The continue prompt to display.
pub fn print_continue(message: &str) {
    print!("\x1b[1;32m{}\x1b[0m", message); // Bright green text
}

/// Print an error message in red, then clear the terminal after a delay.
///
/// # Arguments
/// * `message` - The error message to display.
pub fn print_error(message: &str) {
    clear();
    println!("\x1b[1;31m{}\x1b[0m", message); // Bright red text
    delay(TIME);
    clear();
}
