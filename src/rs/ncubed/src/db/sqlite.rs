//! [Sqlite](https://www.sqlite.org/index.html) is one of the supported
//! databases of Ncube.
//!
//! # Example
//!
//! ```no_run
//! use ncubed::db::sqlite;
//! # #[tokio::main]
//! # async fn main() {
//! let connection_str = "sqlite://:memory:";
//! let db = sqlite::Database::from_str(&connection_str, 10).unwrap();
//!
//! let conn = db.connection().await.unwrap();
//! # }
//! ```
//!
//! Sqlite databases can be created in memory or as a file on-disk. The
//! connection string for in-memory databases is `sqlite://:memory:` and the
//! connection string for file based databases if
//! `sqlite://path/to/file.db`.
use async_trait::async_trait;
use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use thiserror::Error;
use tracing::{debug, instrument};

#[derive(Error, Debug)]
pub struct SqliteConfigError;

impl Display for SqliteConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SqliteConfigError")
    }
}

struct UrlParser;

impl UrlParser {
    fn parse(s: &str) -> Result<Option<Config>, SqliteConfigError> {
        let s = match Self::remove_url_prefix(s) {
            Some(s) => s,
            None => return Ok(None),
        };

        if s == ":memory:" {
            return Ok(Some(Config {
                source: Source::Memory,
            }));
        }

        Ok(Some(Config {
            source: Source::File(PathBuf::from(s)),
        }))
    }

    fn remove_url_prefix(s: &str) -> Option<&str> {
        let prefix = "sqlite://";

        if s.starts_with(prefix) {
            return Some(&s[prefix.len()..]);
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Source {
    File(PathBuf),
    Memory,
}

/// The Sqlite database configuration object. Right now Sqlite databases
/// only require a connection string. The `Config` object can easily be
/// parsed from that.
///
/// ```no_run
/// # use ncubed::db::sqlite;
/// let url = "sqlite://:memory:";
/// let config = url.parse::<sqlite::Config>().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub(crate) source: Source,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            source: Source::Memory,
        }
    }
}

impl FromStr for Config {
    type Err = SqliteConfigError;

    fn from_str(s: &str) -> Result<Self, SqliteConfigError> {
        match UrlParser::parse(s)? {
            Some(config) => Ok(config),
            None => Err(SqliteConfigError),
        }
    }
}

/// A pooled connection to a single Sqlite database. The database can
/// on-disk or in-memory. The pool size is set at point of creation by
/// providing the capacity to the database constructor.
#[derive(Clone)]
pub struct Database {
    config: Config,
    pool: Pool,
}

impl PartialEq for Database {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
    }
}

impl Debug for Database {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "sqlite::Database({:?})", self.config)
    }
}

impl Database {
    /// Construct a pooled Sqlite database. The `capacity` sets the number
    /// of pooled connections.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ncubed::db::sqlite;
    /// # #[tokio::main]
    /// # async fn main () {
    /// let config = "sqlite://:memory:";
    /// let db = sqlite::Database::from_str(&config, 10).unwrap();
    /// let connection = db.connection().await.unwrap();
    /// // Run a query on the connection object.
    /// # }
    /// ```
    pub fn from_str(connection_string: &str, capacity: usize) -> Result<Self, SqliteConfigError> {
        let config: Config = connection_string.parse::<Config>()?;
        let mgr = Manager::new(config.clone());
        let pool = Pool::new(mgr, capacity);
        Ok(Self { pool, config })
    }

    /// Get a single database connection from the pool. The database
    /// connection is a [`rusqlite`](https://crates.io/crates/rusqlite)
    /// connection.
    #[instrument]
    pub async fn connection(
        &self,
    ) -> Result<
        deadpool::managed::Object<ClientWrapper, rusqlite::Error>,
        deadpool::managed::PoolError<rusqlite::Error>,
    > {
        debug!(
            "Fetching a new Sqlite connection from pool: {:?}",
            self.pool.status()
        );
        let conn = self.pool.get().await?;
        Ok(conn)
    }
}

#[derive(Debug)]
pub struct ClientWrapper {
    client: rusqlite::Connection,
}

impl ClientWrapper {
    pub(crate) fn new(client: rusqlite::Connection) -> Self {
        Self { client }
    }
}

impl Deref for ClientWrapper {
    type Target = rusqlite::Connection;
    fn deref(&self) -> &rusqlite::Connection {
        &self.client
    }
}

impl DerefMut for ClientWrapper {
    fn deref_mut(&mut self) -> &mut rusqlite::Connection {
        &mut self.client
    }
}

#[derive(Debug)]
struct Manager {
    config: Config,
}

impl Manager {
    fn new(cfg: Config) -> Self {
        Self { config: cfg }
    }

    fn file<P: AsRef<Path>>(&self, path: P) -> Result<rusqlite::Connection, rusqlite::Error> {
        rusqlite::Connection::open(path)
    }

    fn memory(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        rusqlite::Connection::open_in_memory()
    }
}

type Pool = deadpool::managed::Pool<ClientWrapper, rusqlite::Error>;

#[async_trait]
impl deadpool::managed::Manager<ClientWrapper, rusqlite::Error> for Manager {
    async fn create(&self) -> Result<ClientWrapper, rusqlite::Error> {
        match self.config.source {
            Source::File(ref path) => self.file(path),
            Source::Memory => self.memory(),
        }
        .and_then(|c| Ok(ClientWrapper::new(c)))
    }

    async fn recycle(
        &self,
        conn: &mut ClientWrapper,
    ) -> deadpool::managed::RecycleResult<rusqlite::Error> {
        conn.execute_batch("").map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_sqlite_url_prefix() {
        let url1 = "sqlite:///path/to/db";
        let url2 = "/path/to/db";
        let expected = "/path/to/db";

        assert_eq!(UrlParser::remove_url_prefix(url1), Some(expected));
        assert_eq!(UrlParser::remove_url_prefix(url2), None);
    }

    #[test]
    fn parse_sqlite_config_from_url_string() {
        let url1 = "sqlite:///path/to/db";
        let url2 = "sqlite://:memory:";
        let cfg1 = url1.parse::<Config>().unwrap();
        let cfg2 = url2.parse::<Config>().unwrap();
        assert_eq!(cfg1.source, Source::File("/path/to/db".into()));
        assert_eq!(cfg2.source, Source::Memory);
    }
}
