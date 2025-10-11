use log::{error, info};
use nyxdb::database::query::command::{Command, CommandResult, SelectSemantics};
use nyxdb::database::query::engine::QueryEngine;
use nyxdb::database::utils::logger::logger_init;

/// Testing purpose - to be removed
fn main() {
    logger_init();

    let select_sample = SelectSemantics {
        table: "sales_receipts".to_string(),
        columns: vec!["balance".to_string()],
    };

    let cmd = vec![Command::Select(select_sample)];

    let vm = QueryEngine::new();
    let result = vm.execute(cmd);

    match result {
        CommandResult::SelectSuccess(table) => info!("Select success!!!"),
        CommandResult::Error(e) => error!("Select failed: {}", e),
    }
}
