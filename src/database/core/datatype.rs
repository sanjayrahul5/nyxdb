use chrono::{NaiveDate, NaiveDateTime};
use polars::datatypes::DataType as PolarsDataType;
use polars::prelude::*;

/// Defines the underlying datatypes of the stored data
pub enum DataType {
    String(String),
    Integer(i64), // TODO: Implement unsigned datatypes
    Float(f64),
    Boolean(bool),
    Date(NaiveDate),
    Timestamp(NaiveDateTime),
    Null,
}

impl DataType {
    pub fn from_polars_dtype(data_type: &PolarsDataType) -> Self {
        match data_type {
            PolarsDataType::String => DataType::String(String::new()),
            PolarsDataType::Int8
            | PolarsDataType::Int16
            | PolarsDataType::Int32
            | PolarsDataType::Int64 => DataType::Integer(0),
            PolarsDataType::Float32 | PolarsDataType::Float64 => DataType::Float(0.0),
            PolarsDataType::Boolean => DataType::Boolean(false),
            PolarsDataType::Date => DataType::Date(NaiveDate::default()),
            PolarsDataType::Datetime(_, _) => DataType::Timestamp(NaiveDateTime::default()),
            _ => DataType::Null, // TODO: Implement missed polars datatypes
        }
    }

    pub fn from_polars_value(val: &AnyValue) -> Self {
        match val {
            AnyValue::String(s) => DataType::String(s.to_string()),
            AnyValue::Int8(i) => DataType::Integer(*i as i64),
            AnyValue::Int16(i) => DataType::Integer(*i as i64),
            AnyValue::Int32(i) => DataType::Integer(*i as i64),
            AnyValue::Int64(i) => DataType::Integer(*i as i64),
            AnyValue::Float32(f) => DataType::Float(*f as f64),
            AnyValue::Float64(f) => DataType::Float(*f as f64),
            AnyValue::Boolean(b) => DataType::Boolean(*b),
            AnyValue::Date(d) => {
                let dt = NaiveDate::from_ymd(1970, 1, 1) + chrono::Duration::days(*d as i64);
                DataType::Date(dt)
            }
            AnyValue::Datetime(ts, time_unit, _) => {
                let milliseconds = match time_unit {
                    TimeUnit::Nanoseconds => *ts / 1_000_000,
                    TimeUnit::Microseconds => *ts / 1_000,
                    TimeUnit::Milliseconds => *ts,
                };

                let dt = NaiveDateTime::from_timestamp(0, 0)
                    + chrono::Duration::milliseconds(milliseconds);
                DataType::Timestamp(dt)
            }
            _ => DataType::Null, // TODO: Implement missed polars datatypes
        }
    }
}
