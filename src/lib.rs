mod postgres;
mod reader;

pub enum DbProto {
    Postgres,
    MySql,
    MsSql,
}

pub enum TableType {
    Table(String),
    Query(String),
}

pub struct ReaderOptions {
    protocol: DbProto,
    conn_string: String,
    /// ignored if table is a query
    limit: Option<usize>,
    filter: (),
    projection: Option<Vec<String>>,
    table: TableType,
    batch_size: usize,
}

pub enum WriteMode {
    Insert,
    CreateIfNotExist,
    DropAndCreate,
}
pub struct WriterOptions {
    protocol: DbProto,
    write_mode: WriteMode,
    conn_string: String,
}
