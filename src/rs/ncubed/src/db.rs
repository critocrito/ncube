#[derive(Debug, Clone, PartialEq)]
pub enum Database {
    Sqlite(sqlite::Database),
    Http(http::Database),
}

pub mod http;
pub mod sqlite;
