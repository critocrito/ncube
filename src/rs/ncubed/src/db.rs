pub(crate) mod sqlite {
    use async_trait::async_trait;
    use deadpool;
    use rusqlite;
    use std::fmt::{Debug, Error, Formatter};
    use std::ops::{Deref, DerefMut};
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    struct UrlParser;

    #[derive(Debug)]
    pub(crate) struct ConfigError;

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

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct Config {
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

    #[derive(Clone)]
    pub(crate) struct Database {
        pub(crate) config: Config,
        pub(crate) pool: Pool,
    }

    impl PartialEq for Database {
        fn eq(&self, other: &Self) -> bool {
            self.config == other.config
        }
    }

    impl Debug for Database {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            write!(f, "sqlite::Database()")
        }
    }

    impl Database {
        pub(crate) fn new(config: Config, pool_size: usize) -> Self {
            let mgr = Manager::new(config.clone());
            let pool = Pool::new(mgr, pool_size);
            Self { pool, config }
        }

        pub(crate) async fn connection(
            &self,
        ) -> Result<
            deadpool::managed::Object<ClientWrapper, rusqlite::Error>,
            deadpool::managed::PoolError<rusqlite::Error>,
        > {
            let conn = self.pool.get().await?;
            Ok(conn)
        }
    }

    #[derive(Debug)]
    pub(crate) struct ClientWrapper {
        pub(crate) client: rusqlite::Connection,
    }

    impl ClientWrapper {
        pub fn new(client: rusqlite::Connection) -> Self {
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
    pub(crate) struct Manager {
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

    pub(crate) type Pool = deadpool::managed::Pool<ClientWrapper, rusqlite::Error>;

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
