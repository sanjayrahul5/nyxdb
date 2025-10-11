use crate::database::config::DEFAULT_STORAGE_FORMAT;
use crate::database::core::table::Table;
use crate::database::query::command::{Command, CommandResult};

pub struct QueryEngine;

impl QueryEngine {
    pub fn new() -> Self {
        QueryEngine
    }

    /// Main entry point, executes a list of command in order
    pub fn execute(&self, commands: Vec<Command>) -> CommandResult {
        for command in commands {
            match command {
                Command::Select(sel) => {
                    let result = Table::new().load(
                        sel.table,
                        sel.columns,
                        DEFAULT_STORAGE_FORMAT.to_string(),
                    );

                    match result {
                        Ok(table) => return CommandResult::SelectSuccess(table),
                        Err(e) => return CommandResult::Error(format!("{}", e)),
                    };
                }
            }
        }

        CommandResult::Error("No commands to execute".to_string())
    }
}
