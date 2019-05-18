//! Conversion between Postgres and Arrow datatypes

use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use postgres::types::*;
use postgres::Row;

/// Convert Postgres Type to Arrow DataType
///
/// Not all types are covered, but can be easily added
pub fn pg_to_arrow_type(dt: &Type) -> Option<DataType> {
    match dt {
        &Type::BOOL => Some(DataType::Boolean),
        &Type::BYTEA | &Type::CHAR | &Type::NAME | &Type::TEXT | &Type::VARCHAR => {
            Some(DataType::Utf8)
        }
        &Type::INT2 => Some(DataType::Int16),
        &Type::INT4 => Some(DataType::Int32),
        &Type::INT8 => Some(DataType::Int64),
        &Type::NUMERIC => Some(DataType::Float64),
        //        &OID => None,
        //        &JSON => None,
        &Type::FLOAT4 => Some(DataType::Float32),
        &Type::FLOAT8 => Some(DataType::Float64),
        //        &ABSTIME => None,
        //        &RELTIME => None,
        //        &TINTERVAL => None,
        //        &MONEY => None,
        &Type::BOOL_ARRAY => Some(DataType::List(Box::new(DataType::Boolean))),
        &Type::BYTEA_ARRAY | &Type::CHAR_ARRAY | &Type::NAME_ARRAY => {
            Some(DataType::List(Box::new(DataType::Utf8)))
        }
        &Type::INT2_ARRAY => Some(DataType::List(Box::new(DataType::Int16))),
        //        &INT2_VECTOR => None,
        //        &INT2_VECTOR_ARRAY => None,
        &Type::INT4_ARRAY => Some(DataType::List(Box::new(DataType::Int32))),
        //        &TEXT_ARRAY => None,
        &Type::INT8_ARRAY => Some(DataType::List(Box::new(DataType::Int64))),
        &Type::FLOAT4_ARRAY => Some(DataType::List(Box::new(DataType::Float32))),
        &Type::FLOAT8_ARRAY => Some(DataType::List(Box::new(DataType::Float64))),
        //        &ABSTIME_ARRAY => None,
        //        &RELTIME_ARRAY => None,
        //        &TINTERVAL_ARRAY => None,
        //        &DATE => None,
        &Type::TIME => Some(DataType::Time64(TimeUnit::Microsecond)),
        &Type::TIMESTAMP => Some(DataType::Timestamp(TimeUnit::Millisecond)),
        &Type::TIMESTAMP_ARRAY => Some(DataType::List(Box::new(DataType::Timestamp(
            TimeUnit::Millisecond,
        )))),
        //        &DATE_ARRAY => None,
        &Type::TIME_ARRAY => Some(DataType::List(Box::new(DataType::Time64(
            TimeUnit::Millisecond,
        )))),
        //        &TIMESTAMPTZ => None,
        //        &TIMESTAMPTZ_ARRAY => None,
        //        &INTERVAL => None,
        //        &INTERVAL_ARRAY => None,
        //        &NUMERIC_ARRAY => None,
        //        &TIMETZ => None,
        //        &BIT => None,
        //        &BIT_ARRAY => None,
        //        &VARBIT => None,
        //        &NUMERIC => None,
        //        &UUID => None,
        t @ _ => panic!("Postgres type {:?} not supported", t),
    }
}

/// Generate Arrow schema from a row
pub fn row_to_schema(row: &Row) -> Result<Schema, ()> {
    let fields = row
        .columns()
        .iter()
        .map(|col: &postgres::Column| {
            Field::new(col.name(), pg_to_arrow_type(col.type_()).unwrap(), true)
        })
        .collect();
    Ok(Schema::new(fields))
}
