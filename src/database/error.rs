#[derive(thiserror::Error, Debug)]
pub enum FileReadError {
    #[error("File not found")]
    FileNotFound,
    #[error("Field(s) not found: {0}")]
    FieldNotFound(String),
    #[error("Parquet read error: {0}")]
    ParquetError(String),
}

#[derive(thiserror::Error, Debug)]
pub enum TableError {
    #[error("File format not supported: {0}")]
    UnsupportedFileFormat(String),
    #[error("Table not found")]
    TableNotFound,
    #[error("Column(s) not found: {0}")]
    ColumnNotFound(String),
    #[error("Table read error: {0}")]
    ReadError(String),
}
