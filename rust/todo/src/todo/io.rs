use super::model::Todo;
use csv::{ReaderBuilder, WriterBuilder};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

impl Todo {
    /// Loads tasks and their status from `tasks.csv`.
    ///
    /// This function reads the `tasks.csv` file, processes each line as a task, and stores them in
    /// two vectors: one for task strings and another for their completion statuses.
    pub(super) fn load_tasks() -> Result<(Vec<String>, Vec<bool>), io::Error> {
        // Define the path to the tasks file
        let path = Path::new("tasks.csv");

        // If the file doesn't exist, return a NotFound error
        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }

        // Open the file for reading
        let file = File::open(path)?;

        // Create a CSV reader with no headers, wrapping the file in a buffered reader
        let mut rdr = ReaderBuilder::new()
            .has_headers(false) // The CSV file has no headers
            .from_reader(BufReader::new(file));

        // Initialize two vectors:
        // - one to store task strings
        // - one to store whether each task is completed
        let mut tasks = Vec::new();
        let mut completed = Vec::new();

        // Read each record (row) from the CSV file
        for result in rdr.records() {
            // Unwrap the result or return an error
            let record = result?;

            // Only process rows with at least two fields: task and status
            if record.len() >= 2 {
                // Add the task (first field) to the tasks list
                tasks.push(record[0].to_string());

                // Convert the completion string to a boolean and store it
                completed.push(&record[1] == "true");
            }
        }

        // Return the populated vectors wrapped in Ok
        Ok((tasks, completed))
    }

    /// Saves current tasks and their statuses to `tasks.csv`.
    ///
    /// This function writes the tasks and their completion statuses back to the `tasks.csv` file.
    /// If the file already exists, it will be overwritten (truncated).
    pub(super) fn save_tasks(&self) -> Result<(), io::Error> {
        // Open or create the file for writing (truncates if it already exists)
        let file = File::create("tasks.csv")?;

        // Create a CSV writer with no headers
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);

        // Iterate over the tasks and their corresponding completion status
        for (task, completed) in self.tasks.iter().zip(self.completed.iter()) {
            // Write each task and its status as a CSV row
            wtr.write_record([task, &completed.to_string()])?;
        }

        // Flush the writer to ensure all data is written to the file
        wtr.flush()?;

        // Return success
        Ok(())
    }
}
