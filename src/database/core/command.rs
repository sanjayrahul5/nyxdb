use crate::database::core::table::Table;

/// Logical structure of a SELECT query
pub struct SelectSemantics {
    /// Table to select from
    pub table: String,
    /// Columns to select
    pub columns: Vec<String>,
}

/// Defines all known commands
pub enum Command {
    /// Select command
    Select(SelectSemantics),
}

/// Result of a command execution
pub enum CommandResult {
    /// Successful SELECT with resulting table
    SelectSuccess(Table),
    /// Command failed with error message
    Error(String),
}
