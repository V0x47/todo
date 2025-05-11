// ===========================
// Imports and Module Declarations
// ===========================

use std::io::{self, Write}; // For user input and flushing stdout

// Local modules for system utilities, printing helpers, and task management
mod print;
mod terminal;
mod todo;

use print::{print_error, print_message};
use terminal::clear;
use todo::Todo;

// ===========================
// Menu Function
// ===========================

/// Displays the menu and gets the user's choice as a number (1-6)
fn menu() -> u8 {
    loop {
        clear();

        // Show menu options
        println!("What would you like to do?");
        println!("  1. Add a task");
        println!("  2. List all tasks");
        println!("  3. Edit a task");
        println!("  4. Complete a task");
        println!("  5. Remove a task");
        println!("  6. Exit");
        print!("Enter your choice: ");

        io::stdout().flush().unwrap(); // Make sure prompt is shown

        // Read and parse user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(choice) = input.trim().parse::<u8>() {
            return choice;
        } else {
            print_error("Invalid input! Please enter a number.");
        }
    }
}

// ===========================
// Main Function
// ===========================

fn main() {
    print_message("\n\tWelcome to the To-Do List CLI!");

    let mut todo = Todo::new(); // Load or create task list

    loop {
        match menu() {
            1 => todo.add_task(),
            2 => todo.list_tasks(),
            3 => todo.edit_task(),
            4 => todo.complete_task(),
            5 => todo.remove_task(),
            6 => todo.save_and_exit(),
            _ => {
                // Should not occur unless menu returns invalid input
                print_error("Invalid choice! Please select a valid option.");
            }
        }
    }
}
