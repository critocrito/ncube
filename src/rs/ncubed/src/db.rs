#[derive(Debug, Clone, PartialEq)]
pub enum Database {
    Sqlite(sqlite::Database),
    Http(http::Database),
}

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
    use std::fmt::{Debug, Display, Error, Formatter};
    use std::ops::{Deref, DerefMut};
    use std::path::{Path, PathBuf};
    use std::str::FromStr;
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
}

pub mod http {
    //! When connecting to remote ncube installation all requests are done using
    //! HTTP. Internally the HTTP endpoint is treated like a database.
    //!
    //! # Example
    //!
    //! ```no_run
    //! use ncubed::db::http;
    //! use hyper::{Body, Method, Request};
    //! # #[tokio::main]
    //! # async fn main() {
    //! let endpoint = "https://example.org";
    //! let cfg = endpoint.parse::<http::Config>().unwrap();
    //! let client = http::Database::new(cfg);
    //! // client.get("workspaces")
    //! // client.put("workspaces/1", ..)
    //! // ..
    //! # }
    //! ```
    use bytes::{buf::BufExt as _, Buf};
    use hyper::{client::HttpConnector, Body, Client, Method, Request, Uri};
    use std::fmt::{self, Debug, Formatter};
    use std::ops::{Deref, DerefMut};
    use std::str::FromStr;
    use url::Url;

    use crate::http::SuccessResponse;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AuthToken(String);

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("Config Error")]
        Config,
        #[error("{0}")]
        Http(#[from] hyper::error::Error),
        #[error(transparent)]
        Resp(#[from] serde_json::error::Error),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Config {
        pub(crate) endpoint: Url,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                endpoint: Url::parse("http://127.0.0.1:40666").unwrap(),
            }
        }
    }

    impl FromStr for Config {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Error> {
            let endpoint = Url::parse(s).map_err(|_| Error::Config)?;

            Ok(Config { endpoint })
        }
    }

    #[derive(Clone)]
    pub struct Database {
        config: Config,
        client: ClientWrapper,
    }

    impl PartialEq for Database {
        fn eq(&self, other: &Self) -> bool {
            self.config == other.config
        }
    }

    impl Debug for Database {
        fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
            write!(f, "Http::Database({:?})", self.config)
        }
    }

    impl Database {
        /// Construct a HTTP client.
        ///
        /// # Example
        ///
        /// ```no_run
        /// # use ncubed::db::http;
        /// # #[tokio::main]
        /// # async fn main () {
        /// let config = "http://example.org".parse::<http::Config>().unwrap();
        /// let db = http::Database::new(config);
        /// // Run a query on the connection object.
        /// # }
        /// ```
        pub fn new(config: Config) -> Self {
            let client = Client::new();

            Self {
                client: ClientWrapper::new(client),
                config,
            }
        }

        async fn execute(&self, req: Request<Body>) -> Result<impl Buf, Error> {
            let res = self.client.request(req).await?;
            let body = hyper::body::aggregate(res).await?;

            Ok(body)
        }

        fn url(&self, path: &str) -> Uri {
            let mut uri = self.config.endpoint.clone();
            uri.set_path(path);
            Uri::from_str(uri.as_str()).unwrap()
        }

        #[allow(dead_code)]
        pub(crate) async fn get<T>(&self, path: &str) -> Result<SuccessResponse<T>, Error>
        where
            T: serde::de::DeserializeOwned,
        {
            let url = self.url(&path);
            let req = Request::builder()
                .method(Method::GET)
                .uri(url)
                .header("content-type", "application/json")
                .body(Default::default())
                .unwrap();
            let body = self.execute(req).await?;
            let data = serde_json::from_reader(body.reader())?;

            Ok(data)
        }

        #[allow(dead_code)]
        pub(crate) async fn post<T, S>(
            &self,
            path: &str,
            payload: S,
        ) -> Result<SuccessResponse<T>, Error>
        where
            T: serde::de::DeserializeOwned,
            S: serde::Serialize,
        {
            let url = self.url(&path);
            let payload_json = serde_json::to_string(&payload).unwrap().into_bytes();
            let req = Request::post(url)
                .header("content-type", "application/json")
                .body(Body::from(payload_json))
                .unwrap();
            let body = self.execute(req).await?;
            let data = serde_json::from_reader(body.reader())?;

            Ok(data)
        }

        #[allow(dead_code)]
        pub(crate) async fn put<T, S>(
            &self,
            path: &str,
            payload: S,
        ) -> Result<SuccessResponse<T>, Error>
        where
            T: serde::de::DeserializeOwned,
            S: serde::Serialize,
        {
            let url = self.url(&path);
            let payload_json = serde_json::to_string(&payload).unwrap().into_bytes();
            let req = Request::put(url)
                .header("content-type", "application/json")
                .body(Body::from(payload_json))
                .unwrap();
            let body = self.execute(req).await?;
            let data = serde_json::from_reader(body.reader())?;

            Ok(data)
        }

        #[allow(dead_code)]
        pub(crate) async fn delete<T, S>(&self, path: &str) -> Result<SuccessResponse<T>, Error>
        where
            T: serde::de::DeserializeOwned,
        {
            let url = self.url(&path);
            let req = Request::delete(url)
                .header("content-type", "application/json")
                .body(Default::default())
                .unwrap();

            let body = self.execute(req).await?;
            let data = serde_json::from_reader(body.reader())?;

            Ok(data)
        }
    }

    #[derive(Debug, Clone)]
    pub struct ClientWrapper {
        client: Client<HttpConnector, Body>,
    }

    impl ClientWrapper {
        pub(crate) fn new(client: Client<HttpConnector, Body>) -> Self {
            Self { client }
        }
    }

    impl Deref for ClientWrapper {
        type Target = Client<HttpConnector, Body>;
        fn deref(&self) -> &Client<HttpConnector, Body> {
            &self.client
        }
    }

    impl DerefMut for ClientWrapper {
        fn deref_mut(&mut self) -> &mut Client<HttpConnector, Body> {
            &mut self.client
        }
    }
}
