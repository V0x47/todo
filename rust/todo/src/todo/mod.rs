// Declare the modules that this crate uses
pub mod actions; // This will handle user actions like adding tasks, completing tasks, etc.
pub mod io; // This module will handle input/output operations, like reading from and writing to the terminal.
pub mod model; // This module contains the core data structures, like `Todo`, task status, etc.
pub mod utils; // This module may contain utility functions, like the ones used for interacting with the user.

/// Publicly expose the `Todo` struct from the `model` module so other parts of the program can use it.
pub use model::Todo;
