//! Postgres Reader

use crate::postgres::data_types::row_to_schema;
use crate::reader::DatabaseReader;
use crate::{ReaderOptions, TableType};

// use arrow::array::*;
use arrow::array::*;
use arrow::datatypes::*;
use arrow::record_batch::RecordBatch;
use chrono::Timelike;
use postgres::{Client, NoTls, Row};

pub struct PostgresReader {
    pub options: ReaderOptions,
}

impl PostgresReader {
    /// limit overrides the internal limit by wrapping a query in a limit
    fn get_query(&self, limit: Option<usize>) -> String {
        match &self.options.table {
            TableType::Query(query) => match limit {
                Some(limit) => format!("SELECT * FROM ({}) LIMIT {}", query, limit),
                None => query.to_owned(),
            },
            // TODO take the lower limit
            TableType::Table(ref table) => match limit {
                Some(limit) => format!("SELECT * FROM {} LIMIT {}", table, limit),
                None => format!("SELECT * FROM {}", table),
            },
        }
    }

    fn connect(&self) -> Result<Client, ()> {
        Client::connect(&self.options.conn_string, NoTls).map_err(|_e| ())
    }
}

impl DatabaseReader for PostgresReader {
    fn read(&self) -> Box<Iterator<Item = Result<RecordBatch, ()>>> {
        let mut client = self.connect().unwrap();
        // TODO `query_iter` would work better, but haven't found a way of collecting chunks of rows
        // I tried chunking an iterator, then struggled to make itertools work with fallible_iterator
        let results = client.query(self.get_query(None).as_str(), &[]).unwrap();

        // TODO change signature so we can return an error on failure/empty results

        let schema = row_to_schema(results.get(0).unwrap()).unwrap();
        let field_len = schema.fields().len();
        let mut builder = StructBuilder::from_fields(schema.fields().to_vec(), self.options.batch_size);
        let chunks = results.chunks(self.options.batch_size);
        let mut batches = vec![];
        chunks.for_each(|chunk: &[Row]| {
            for j in 0..field_len {
                match schema.field(j).data_type() {
                    DataType::Int32 => {
                        let field_builder = builder.field_builder::<Int32Builder>(j).unwrap();
                        for i in 0..chunk.len() {
                            let row: &Row = chunk.get(i).unwrap();
                            match row.try_get(j) {
                                Ok(value) => field_builder.append_value(value).unwrap(),
                                Err(_) => field_builder.append_null().unwrap(),
                            };
                        }
                    }
                    DataType::Int64 => {
                        let field_builder = builder.field_builder::<Int64Builder>(j).unwrap();
                        for i in 0..chunk.len() {
                            let row: &Row = chunk.get(i).unwrap();
                            match row.try_get(j) {
                                Ok(value) => field_builder.append_value(value).unwrap(),
                                Err(_) => field_builder.append_null().unwrap(),
                            };
                        }
                    }
                    DataType::Float64 => {
                        let field_builder = builder.field_builder::<Float64Builder>(j).unwrap();
                        for i in 0..chunk.len() {
                            let row: &Row = chunk.get(i).unwrap();
                            match row.try_get(j) {
                                Ok(value) => field_builder.append_value(value).unwrap(),
                                Err(_) => field_builder.append_null().unwrap(),
                            };
                        }
                    }
                    DataType::Timestamp(TimeUnit::Millisecond, None) => {
                        let field_builder = builder
                            .field_builder::<TimestampMillisecondBuilder>(j)
                            .unwrap();
                        for i in 0..chunk.len() {
                            let row: &Row = chunk.get(i).unwrap();
                            let timestamp: chrono::NaiveDateTime = row.get(j);
                            field_builder
                                .append_value(timestamp.timestamp_millis())
                                .unwrap();
                        }
                    }
                    DataType::Time64(TimeUnit::Microsecond) => {
                        let field_builder = builder
                            .field_builder::<Time64MicrosecondBuilder>(j)
                            .unwrap();
                        for i in 0..chunk.len() {
                            let row: &Row = chunk.get(i).unwrap();
                            let time: chrono::NaiveTime = row.get(j);
                            field_builder
                                .append_value(
                                    time.num_seconds_from_midnight() as i64 * 1000000
                                        + time.nanosecond() as i64 / 1000,
                                )
                                .unwrap();
                        }
                    }
                    DataType::Boolean => {
                        let field_builder = builder.field_builder::<BooleanBuilder>(j).unwrap();
                        for i in 0..chunk.len() {
                            let row: &Row = chunk.get(i).unwrap();
                            field_builder.append_value(row.get(j)).unwrap();
                        }
                    }
                    DataType::Utf8 => {
                        let field_builder = builder.field_builder::<StringBuilder>(j).unwrap();
                        for i in 0..chunk.len() {
                            let row: &Row = chunk.get(i).unwrap();
                            match row.try_get(j) {
                                Ok(value) => field_builder.append_value(value).unwrap(),
                                Err(_) => field_builder.append_null().unwrap(),
                            }
                        }
                    }
                    t @ _ => panic!("Field builder for {:?} not yet supported", t),
                }
            }
            // TODO perhaps change the order of processing so we can do this earlier
            for _i in 0..chunk.len() {
                builder.append(true).unwrap();
            }
            let batch: RecordBatch = RecordBatch::from(&builder.finish());
            batches.push(batch);
        });
        Box::new(batches.into_iter().map(|batch| Ok(batch)))
    }

    fn schema(&self) -> Result<Schema, ()> {
        let mut client = self.connect().unwrap();
        let results = client.query(self.get_query(Some(1)).as_str(), &[]).unwrap();
        if results.is_empty() {
            // we currently can't read the schema of an empty table
            return Err(());
        }
        let schema = row_to_schema(results.get(0).unwrap());
        // TODO apply any schema projection
        schema
    }
}
