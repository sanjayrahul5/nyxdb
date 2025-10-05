use crate::database::core::config::DATA_DIR;
use crate::database::core::datatype::DataType;
use crate::database::core::errors::{FileReadError, TableError};
use crate::database::core::file_format::FileFormat;
use crate::database::core::file_io::parquet::parquet_reader::ParquetReader;
use crate::database::core::file_io::reader::TableReader;
use log::{error, info};
use std::collections::HashMap;
use std::str::FromStr;

/// Defines how the data will be stored cached in-memory by Nyx DB
pub struct Table {
    /// Name of the table
    pub name: String,
    /// Schema of the table
    pub schema: HashMap<String, DataType>,
    /// Actual data in columnar format
    pub columns: HashMap<String, Vec<DataType>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            name: String::new(),
            schema: HashMap::<String, DataType>::new(),
            columns: HashMap::<String, Vec<DataType>>::new(),
        }
    }

    pub fn load(
        &self,
        table_name: String,
        select_columns: Vec<String>,
        file_format: String,
    ) -> Result<Table, TableError> {
        let format = FileFormat::from_str(file_format.as_str())
            .map_err(|e| TableError::UnsupportedFileFormat(file_format))?;

        let path = format!("{}/{}.{}", DATA_DIR, table_name, format.extension());

        info!("Loading table from path: {}", path);

        let reader: Box<dyn TableReader>;
        match format {
            FileFormat::Parquet => reader = ParquetReader::new(),
        }

        let (schema, columns) = reader.read(path, select_columns.clone()).map_err(|e| {
            error!("Error reading file: {}", e);
            match e {
                FileReadError::FileNotFound => TableError::TableNotFound,
                FileReadError::FieldNotFound(c) => TableError::ColumnNotFound(c),
                _ => TableError::ReadError(format!("{}", e)),
            }
        })?;

        let table = Table {
            name: table_name,
            schema,
            columns,
        };

        Ok(table)
    }
}
