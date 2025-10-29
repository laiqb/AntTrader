
/// Trait providing an explicit cleanup method that may be invoked prior to `Drop`.
pub trait CleanDrop {
    /// Perform custom cleanup, releasing external resources and breaking strong reference cycles.
    fn clean_drop(&mut self);
}
