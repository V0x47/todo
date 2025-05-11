use super::model::Todo;
use crate::print::*;
use std::io::{self, Write};

// Maximum allowed length for a task
const MAX_TASK_LEN: usize = 100;

impl Todo {
    /// Prints the list of tasks with their completion status.
    ///
    /// This function iterates through the `tasks` and `completed` vectors, printing each task with
    /// a checkbox ("✓" for completed and a space for incomplete tasks).
    pub(super) fn print_tasks(&mut self) {
        println!("\n");
        for (i, task) in self.tasks.iter().enumerate() {
            // Determine whether the task is completed or not
            let status = if self.completed[i] { "✓" } else { " " };
            println!("  {}: [{}] {}", i + 1, status, task)
        }
        println!();
    }

    /// Prompts the user to select a valid task index for various operations (e.g., edit, remove).
    ///
    /// The function repeatedly asks the user for a task index. If the index is invalid or out of range,
    /// it will show an error message and prompt the user to try again. If there is only one task, it confirms
    /// whether the user wants to perform the operation on it.
    pub(super) fn get_index(&mut self, message: &str) -> Result<usize, ()> {
        loop {
            // If there are no tasks, show a message and return an error
            if self.tasks.is_empty() {
                print_message(&format!("No tasks available to {}!", message));
                return Err(());
            }

            // If there's only one task, ask the user if they want to proceed with the operation
            if self.tasks.len() == 1 {
                println!("There is only one Task: {}", self.tasks[0]);
                if Todo::get_answer(&format!("Do you want to {}? [y or n] ", message)) {
                    return Ok(1); // Task index 1 (the only task) is selected
                } else {
                    return Err(()); // Action aborted by user
                }
            }

            // Display the list of tasks to the user
            print_info("Available tasks: ");
            self.print_tasks();

            // Prompt the user to enter a task index for the operation
            print_info(&format!(
                "Enter the index of the task to {} (0 to abort): ",
                message
            ));

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            // Try to parse the input as a valid task index
            match input.trim().parse::<usize>() {
                // If the user enters 0, the operation is aborted
                Ok(0) => {
                    print_message("Action aborted.");
                    return Err(()); // Explicit abort
                }
                // If the index is valid (within bounds), return the index adjusted for 0-based indexing
                Ok(index) if index <= self.tasks.len() => return Ok(index - 1),
                // If the index is invalid (out of range), ask the user to try again
                Ok(_) => {
                    print_error("Invalid index! Please enter a valid number.");
                    // retry
                }
                // If the input is not a valid number, prompt the user again
                Err(_) => {
                    print_error("Invalid input! Please enter a number.");
                    // retry
                }
            }
        }
    }

    /// Prompts the user with a yes/no question and returns true if they answer "y" or "Y"
    ///
    /// This function loops until the user provides a valid response ("y", "Y", "n", or "N"). If the user
    /// answers with "y" or "Y", it returns true; if they answer with "n" or "N", it returns false. For any
    /// other input, it asks the user again.
    pub(super) fn get_answer(message: &str) -> bool {
        loop {
            print_info(message);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let input = input.trim_end();

            // Return true if user answers "y" or "Y", and false for "n" or "N"
            match input {
                "y" | "Y" => return true,
                "n" | "N" => return false,
                // For any other input, show an error and ask again
                _ => {
                    print_error("Invalid input. Please enter 'y' or 'n'.");
                }
            }
        }
    }

    /// This function handles reading a task from the user and checks if it's too long.
    /// If the task exceeds the maximum length, it keeps asking until a valid task is entered.
    pub(super) fn enter_task() -> String {
        loop {
            // Prompt the user for input
            print!("Enter new task: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            // Trim the input to remove trailing newlines or spaces
            let input = input.trim_end();

            // Check if the task is too long
            if input.len() > MAX_TASK_LEN {
                print_error(&format!(
                    "Task is too long! Please keep it under {} characters.",
                    MAX_TASK_LEN
                ));
                continue; // Ask for input again if task is too long
            }

            return input.to_string(); // Return the valid task
        }
    }
}
