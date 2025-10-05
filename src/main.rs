use log::{error, info};
use nyxdb::database::core::command::{Command, CommandResult, SelectSemantics};
use nyxdb::database::core::logger::logger_init;
use nyxdb::database::core::virtual_machine::VirtualMachine;

/// Testing purpose - to be removed
fn main() {
    logger_init();

    let select_sample = SelectSemantics {
        table: "sales_receipts".to_string(),
        columns: vec!["balances".to_string()],
    };

    let cmd = vec![Command::Select(select_sample)];

    let vm = VirtualMachine::new();
    let result = vm.execute(cmd);

    match result {
        CommandResult::SelectSuccess(table) => info!("Select success!!!"),
        CommandResult::Error(e) => error!("Select failed: {}", e),
    }
}
