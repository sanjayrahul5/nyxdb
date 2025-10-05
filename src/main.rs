use log::{error, info};
use nyxdb::database::core::cache_table::CacheTable;
use nyxdb::database::core::logger::logger_init;

fn main() {
    logger_init();

    let table_name = "sales_receipts".to_string();
    let select_columns = vec!["balance".to_string()];
    let file_format = "parquet".to_string();

    let result = CacheTable::new().load(table_name, select_columns, file_format);
    match result {
        Ok(_) => {
            info!("success")
        }
        Err(e) => {
            error!("{}", e)
        }
    }
}
