pub mod postgres;
pub mod reader;

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
    pub protocol: DbProto,
    pub conn_string: String,
    /// ignored if table is a query
    pub limit: Option<usize>,
    pub projection: Option<Vec<String>>,
    pub table: TableType,
    pub batch_size: usize,
}

pub enum WriteMode {
    Insert,
    CreateIfNotExist,
    DropAndCreate,
}
