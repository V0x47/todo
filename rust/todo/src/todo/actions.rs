use super::model::Todo;
use crate::print::*;
use crate::terminal::clear;
use std::io::{self, Read, Write};

impl Todo {
    /// Creates a new Todo instance by trying to load existing tasks from file.
    ///
    /// This method attempts to load the tasks and their completion status from a file.
    /// If the file is not found or an error occurs, it initializes an empty Todo list.
    pub fn new() -> Self {
        match Self::load_tasks() {
            Ok((tasks, completed)) => Self { tasks, completed },
            Err(_) => Self {
                tasks: Vec::new(),
                completed: Vec::new(),
            },
        }
    }

    /// Adds a new task to the list.
    ///
    /// This method prompts the user to enter a new task and then adds it to the list of tasks.
    /// The task is truncated if it exceeds the `MAX_TASK_LEN` length. The task is marked as uncompleted.
    pub fn add_task(&mut self) {
        clear();

        let task = Todo::enter_task();

        // Add task to the list, and mark it as uncompleted
        self.tasks.push(task);
        self.completed.push(false);

        print_success("Task added");
    }

    /// Displays the list of tasks.
    ///
    /// This method prints all tasks in the list, along with their completion status.
    /// If there are no tasks, a message is displayed to inform the user.
    pub fn list_tasks(&mut self) {
        clear();

        if self.tasks.is_empty() {
            print_message("No tasks available!");
            return;
        }

        print_info("Tasks: ");
        self.print_tasks(); // Assumes this method is implemented to print tasks

        print_continue("Press any key to continue... ");
        io::stdout().flush().unwrap();

        let mut buffer = [0u8];
        io::stdin().read_exact(&mut buffer).unwrap();
    }

    /// Edits an existing task.
    ///
    /// This method allows the user to modify a task in the list. If the task is completed,
    /// it prompts the user to confirm if they still want to edit it. After editing, it may
    /// prompt to mark the task as uncompleted.
    pub fn edit_task(&mut self) {
        clear();
        let index = match self.get_index("edit") {
            Ok(i) => i,
            Err(_) => return, // Aborted by user
        };

        // If completed, confirm whether to continue editing
        if self.completed[index]
            && !Todo::get_answer("This task is completed. Do you still want to edit it? [y or n] ")
        {
            return;
        }

        clear();

        let task = Todo::enter_task();

        // Update the task
        self.tasks[index] = task.to_string();
        print_success("Task edited successfully.");

        // Ask whether to mark the edited task as uncompleted
        if self.completed[index]
            && Todo::get_answer("Do you want to set this edited task as uncompleted? [y or n] ")
        {
            self.completed[index] = false;
        }
    }

    /// Marks a task as completed.
    ///
    /// This method marks the task at the specified index as completed.
    /// If the task is already completed, it won't do anything.
    pub fn complete_task(&mut self) {
        clear();
        let index = match self.get_index("complete") {
            Ok(i) => i,
            Err(_) => return, // Aborted by user
        };

        // If the task is not yet completed, mark it as completed
        if !self.completed[index] {
            self.completed[index] = true;

            print_success("Task completed!");
        }
    }

    /// Removes a task from the list.
    ///
    /// This method allows the user to remove a task from the list. It prompts the user for confirmation
    /// if the task is not completed. The task is removed from both the `tasks` and `completed` vectors.
    pub fn remove_task(&mut self) {
        clear();
        let index = match self.get_index("remove") {
            Ok(i) => i,
            Err(_) => return, // Aborted by user
        };

        // If task is not completed, confirm removal
        if !self.completed[index]
            && !Todo::get_answer(
                "Task is unfinished. Are you sure you want to remove it? [y or n] ",
            )
        {
            return;
        }

        // Remove task and completion status
        self.tasks.remove(index);
        self.completed.remove(index);

        print_success("Task removed!");
    }

    /// Saves tasks and exits the program.
    ///
    /// This method saves the current tasks and their statuses to the file before exiting the program.
    pub fn save_and_exit(&mut self) {
        if let Err(err) = self.save_tasks() {
            let msg = format!("Error saving tasks: {}", err);
            print_error(&msg);
        } else {
            print_message("Exiting... ");
        }

        clear();
        std::process::exit(0);
    }
}
