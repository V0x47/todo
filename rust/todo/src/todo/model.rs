/// A simple struct representing a Todo list.
///
/// This struct holds two vectors:
/// - `tasks`: a list of task descriptions (as Strings)
/// - `completed`: a list of booleans representing whether each corresponding task is completed
///
/// Both fields use `pub(super)` visibility to allow access from sibling modules within the same crate
/// (e.g., from `actions.rs`) while still keeping them private from external code.
pub struct Todo {
    /// A list of task descriptions.
    ///
    /// Each entry in this vector corresponds to a single task the user has added.
    /// This is kept in sync with the `completed` vector by position/index.
    pub(super) tasks: Vec<String>,

    /// A list of task completion statuses.
    ///
    /// Each boolean in this vector indicates whether the task at the corresponding index in `tasks`
    /// has been completed (`true`) or not (`false`).
    pub(super) completed: Vec<bool>,
}
