use std::str::FromStr;

pub enum FileFormat {
    Parquet,
}

impl FileFormat {
    pub fn extension(&self) -> &str {
        match self {
            FileFormat::Parquet => "parquet",
        }
    }
}

impl FromStr for FileFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "parquet" => Ok(FileFormat::Parquet),
            other => Err(format!("Unsupported file format: {other}",)),
        }
    }
}
