
// use crate::reader::DatabaseReader;
use std::time::Instant;

use arrow::ipc::writer::StreamWriter;
use rust_arrow_sql::{DbProto, ReaderOptions, TableType};
use rust_arrow_sql::postgres::reader::PostgresReader;
use rust_arrow_sql::reader::DatabaseReader;

fn main() {
    let options = ReaderOptions{
        batch_size: 100000,
        limit: None,
        projection: None,
        protocol: DbProto::Postgres, 
        conn_string: String::from("host=localhost port=5432 dbname=postgres user=postgres  connect_timeout=10"),
        table: TableType::Table(String::from("city"))
    };
    let reader = PostgresReader{options: options};
    let before = Instant::now();

    let mut batches =  reader.read();
    let first_batch = batches.next().unwrap().unwrap();

    let schema = first_batch.schema();
    let vec = Vec::new();
    let mut writer = StreamWriter::try_new(vec, &schema).unwrap();

    writer.write(&first_batch);
    while let Some(batch) = batches.next() {
        writer.write(&batch.unwrap());
    }
    println!("Elapsed time: {:.2?}", before.elapsed());
}