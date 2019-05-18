//! Interface for reading from databases

use arrow::datatypes::Schema;
use arrow::record_batch::RecordBatch;
// use fallible_iterator::FallibleIterator;

pub trait DatabaseReader {
    /// read data from database, returned as an iterator of record batches
    // Box is a temporary measure for now (replace with impl Iterator?)
    fn read(&self) -> Box<Iterator<Item = Result<RecordBatch, ()>>>;

    /// inspect database schema
    fn schema(&self) -> Result<Schema, ()>;
}
