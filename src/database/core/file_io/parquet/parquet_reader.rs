use crate::database::core::datatype::DataType;
use crate::database::core::errors::FileReadError;
use crate::database::core::file_io::reader::TableReader;
use polars::prelude::*;
use std::collections::HashMap;
use std::path::Path;

/// Reader for parquet file format
pub struct ParquetReader;

impl ParquetReader {
    pub fn new() -> Box<ParquetReader> {
        Box::new(ParquetReader)
    }
}

impl TableReader for ParquetReader {
    fn read(
        &self,
        file_path: String,
        select_columns: Vec<String>,
    ) -> Result<(HashMap<String, DataType>, HashMap<String, Vec<DataType>>), FileReadError> {
        // Check if file exists
        if !Path::new(&file_path).exists() {
            return Err(FileReadError::FileNotFound);
        }

        let mut lf = LazyFrame::scan_parquet(PlPath::new(&file_path), ScanArgsParquet::default())
            .map_err(|e| FileReadError::ParquetError(e.to_string()))?;

        if !select_columns.is_empty() {
            // Check if all requested columns exist in the parquet file.
            // Polars would normally throw an error if a selected column is not found,
            // but we want to propagate a cleaner error message to the caller.
            let temp_df = lf
                .clone()
                .limit(1)
                .collect()
                .map_err(|e| FileReadError::ParquetError(e.to_string()))?;

            let available_columns = temp_df.get_column_names_str();

            let missing_columns: Vec<String> = select_columns
                .iter()
                .filter(|c| !available_columns.contains(&c.as_str()))
                .cloned()
                .collect();

            if !missing_columns.is_empty() {
                return Err(FileReadError::FieldNotFound(missing_columns.join(", ")));
            }

            lf = lf.select(select_columns.iter().map(|c| col(c)).collect::<Vec<_>>())
        };

        let df = lf
            .collect()
            .map_err(|e| FileReadError::ParquetError(e.to_string()))?;

        // Build schema
        let schema = df
            .schema()
            .iter()
            .map(|(name, dtype)| (name.to_string(), DataType::from_polars_dtype(dtype)))
            .collect();

        // Build column data
        let data = df
            .get_columns()
            .iter()
            .map(|col| {
                let values = (0..col.len())
                    .map(|i| {
                        col.get(i)
                            .map_or(DataType::Null, |v| DataType::from_polars_value(&v))
                    })
                    .collect();
                (col.name().to_string(), values)
            })
            .collect();

        Ok((schema, data))
    }
}
