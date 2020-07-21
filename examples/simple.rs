
// use crate::reader::DatabaseReader;
use std::time::Instant;

use arrow::ipc::writer::StreamWriter;
use rust_arrow_sql::{DbProto, ReaderOptions, TableType};
use rust_arrow_sql::postgres::reader::PostgresReader;
use rust_arrow_sql::reader::DatabaseReader;

fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    let options = ReaderOptions{
        batch_size: 10000,
        limit: None,
        projection: None,
        protocol: DbProto::Postgres, 
        conn_string: String::from("host=localhost port=5432 dbname=postgres user=postgres password=admin connect_timeout=10"),
        table: TableType::Table(String::from("nut_data"))
    };
    let reader = PostgresReader{options: options};
    let before = Instant::now();
    let zeme =  reader.read();
    // let zeme =  *reader.read();
    // let mut vec = Vec::new();
    // let mut writer = StreamWriter::try_new(vec, &schema)?;
    for item in zeme {
        println!("sucess")
    }

    // zeme.for_each(|x| println!("sucess"))

    // }

    // let schema = record.schema();
    println!("Elapsed time: {:.2?}", before.elapsed());

    // let mut vec = Vec::new();
    // let mut writer = StreamWriter::try_new(vec, &schema)?;

    // let mut writer = StreamWriter::try_new(io::stdout(), &schema)?;

    // let mut stream_writer = StreamWriter::try_new(file, &schema).unwrap();
    //     stream_writer.write(&batch).unwrap();
    //     stream_writer.finish().unwrap();

        
}