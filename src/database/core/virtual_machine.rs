use crate::database::core::command::{Command, CommandResult};
use crate::database::core::config::DEFAULT_FILE_FORMAT;
use crate::database::core::table::Table;

pub struct VirtualMachine;

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine
    }

    /// Main entry point, executes a list of command in order
    pub fn execute(&self, commands: Vec<Command>) -> CommandResult {
        for command in commands {
            match command {
                Command::Select(sel) => {
                    let result =
                        Table::new().load(sel.table, sel.columns, DEFAULT_FILE_FORMAT.to_string());

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
