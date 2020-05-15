pub mod sqlite {
    //! [Sqlite](https://www.sqlite.org/index.html) is one of the supported
    //! databases of Ncube.
    //!
    //! # Example
    //!
    //! ```no_run
    //! use ncubed::db::sqlite;
    //! # #[tokio::main]
    //! # async fn main() {
    //! let db_path = "sqlite://:memory:";
    //! let config = db_path.parse::<sqlite::Config>().unwrap();
    //! let db = sqlite::Database::new(config, 10);
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
    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Error, Formatter};
    use std::ops::{Deref, DerefMut};
    use std::path::{Path, PathBuf};
    use std::str::FromStr;
    use std::sync::{Mutex, RwLock};
    use tracing::{debug, instrument};

    struct UrlParser;

    #[derive(thiserror::Error, Debug)]
    pub struct ConfigError;

    impl Display for ConfigError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ConfigError()")
        }
    }

    impl UrlParser {
        fn parse(s: &str) -> Result<Option<Config>, ConfigError> {
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
        type Err = ConfigError;

        fn from_str(s: &str) -> Result<Self, ConfigError> {
            match UrlParser::parse(s)? {
                Some(config) => Ok(config),
                None => Err(ConfigError),
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
        /// let config = "sqlite://:memory:".parse::<sqlite::Config>().unwrap();
        /// let db = sqlite::Database::new(config, 10);
        /// let connection = db.connection().await.unwrap();
        /// // Run a query on the connection object.
        /// # }
        /// ```
        pub fn new(config: Config, capacity: usize) -> Self {
            let mgr = Manager::new(config.clone());
            let pool = Pool::new(mgr, capacity);
            Self { pool, config }
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

    /// Cache Sqlite database pools.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ncubed::db::sqlite;
    /// let cache = sqlite::DatabaseCache::new();
    ///
    /// let url = "sqlite://:memory:";
    /// let cfg = url.parse::<sqlite::Config>().unwrap();
    /// let db = sqlite::Database::new(cfg, 1);
    ///
    /// assert!(cache.get(url).is_none());
    ///
    /// cache.put(url, db);
    ///
    /// let db1 = cache.get(url).unwrap();
    /// let db2 = cache.get(url).unwrap();
    ///
    /// assert_eq!(db1, db2);
    /// ```
    #[derive(Debug)]
    pub struct DatabaseCache(RwLock<HashMap<String, Mutex<Database>>>);

    impl DatabaseCache {
        pub fn new() -> Self
        where
            Self: Sized,
        {
            DatabaseCache(RwLock::new(HashMap::new()))
        }

        pub fn get(&self, key: &str) -> Option<Database> {
            let trimmed = key.trim().to_string();
            let cache = self.0.read().expect("RwLock poisoned");
            if let Some(elem) = cache.get(&trimmed) {
                let elem = elem.lock().expect("Mutex poisoned");
                let db = elem.clone();
                return Some(db);
            }
            None
        }

        pub fn put(&self, key: &str, db: Database) {
            let trimmed = key.trim().to_string();
            let mut cache = self.0.write().expect("RwLock poisoned");
            cache.entry(trimmed).or_insert_with(|| Mutex::new(db));
        }

        pub fn has(&self, key: &str) -> bool {
            let trimmed = key.trim().to_string();
            let cache = self.0.read().expect("RwLock poisoned");
            match cache.get(&trimmed) {
                Some(_) => true,
                _ => false,
            }
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

        fn database_cache_for_sqlite_database_types() {
            let url1 = "sqlite://:memory:";
            let url2 = "sqlite://testdb";
            let cfg1 = url1.parse::<Config>().unwrap();
            let cfg2 = url2.parse::<Config>().unwrap();
            let db1 = Database::new(cfg1, 2);
            let db2 = Database::new(cfg2, 2);
            let cache = DatabaseCache::new();

            assert!(cache.get(url1).is_none());
            assert!(cache.get(url2).is_none());

            cache.put(url1, db1);
            cache.put(url2, db2);

            let db3 = cache.get(url1).unwrap();
            let db4 = cache.get(url1).unwrap();

            assert_eq!(db3, db4);
        }

        #[test]
        fn database_cache_for_sqlite_database_tests_existence() {
            let url = "sqlite://:memory:";
            let cfg = url.parse::<Config>().unwrap();
            let db = Database::new(cfg, 2);
            let cache = DatabaseCache::new();

            assert_eq!(cache.has(url), false);

            cache.put(url, db);

            assert_eq!(cache.has(url), true);
        }
    }
}
