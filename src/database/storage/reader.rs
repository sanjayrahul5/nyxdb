use crate::database::core::datatype::DataType;
use crate::database::error::FileReadError;
use std::collections::HashMap;

/// Public interface of table reader
pub trait TableReader {
    fn read(
        &self,
        file_path: String,
        select_columns: Vec<String>,
    ) -> Result<(HashMap<String, DataType>, HashMap<String, Vec<DataType>>), FileReadError>;
}
