pub mod http;
pub mod sqlite;

#[derive(Debug, Clone, PartialEq)]
pub enum Database {
    Sqlite(Box<sqlite::Database>),
    Http(Box<http::Database>),
}
