use ncube_cache::GuardedCache;
use ncube_errors::HostError;
use std::fmt::Debug;
use tracing::instrument;

pub mod errors;
pub mod http;
pub mod migrations;
pub mod sqlite;

pub use self::errors::DatabaseError;

#[derive(Debug, Clone, PartialEq)]
pub enum Database {
    Sqlite(Box<sqlite::Database>),
    Http(Box<http::Database>),
}

impl Database {
    #[instrument]
    pub async fn login(&mut self) -> Result<(), HostError> {
        match self {
            Database::Http(inner_db) => {
                inner_db.ensure_login().await.map_err(|e| {
                    let msg = format!("login failed: {:?}", e.to_string());
                    HostError::AuthError(msg)
                })?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
/// Cache database connectors.
///
/// # Example
///
/// ```
/// use ncube_db::{Database, DatabaseCache, sqlite};
///
/// let cache = DatabaseCache::new();
///
/// let url = "sqlite://:memory:";
/// let db = sqlite::Database::from_str(url, 1).unwrap();
///
/// assert!(cache.get(url).is_none());
///
/// cache.put(url, Database::Sqlite(Box::new(db)));
///
/// let db1 = cache.get(url).unwrap();
/// let db2 = cache.get(url).unwrap();
///
/// assert_eq!(db1, db2);
/// ```
pub type DatabaseCache = GuardedCache<Database>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_cache_for_sqlite_database_types() {
        let url1 = "sqlite://:memory:";
        let url2 = "sqlite://testdb";
        let db1 = sqlite::Database::from_str(&url1, 2).unwrap();
        let db2 = sqlite::Database::from_str(&url2, 2).unwrap();
        let cache = DatabaseCache::new();

        assert!(cache.get(url1).is_none());
        assert!(cache.get(url2).is_none());

        cache.put(url1, Database::Sqlite(Box::new(db1)));
        cache.put(url2, Database::Sqlite(Box::new(db2)));

        let db3 = cache.get(url1).unwrap();
        let db4 = cache.get(url1).unwrap();

        assert_eq!(db3, db4);
    }

    #[test]
    fn database_cache_for_sqlite_database_tests_existence() {
        let url = "sqlite://:memory:";
        let db = sqlite::Database::from_str(&url, 2).unwrap();
        let cache = DatabaseCache::new();

        assert_eq!(cache.has(url), false);

        cache.put(url, Database::Sqlite(Box::new(db)));

        assert_eq!(cache.has(url), true);
    }

    #[test]
    fn database_cache_resets_the_cache() {
        let url1 = "sqlite://test.db";
        let url2 = "sqlite://test2.db";
        let db1 = sqlite::Database::from_str(&url1, 1).unwrap();
        let db2 = sqlite::Database::from_str(&url2, 2).unwrap();

        let cache = DatabaseCache::new();

        cache.put(url1, Database::Sqlite(Box::new(db1)));

        let db3 = cache.get(url1).unwrap();

        cache.reset(url1, Database::Sqlite(Box::new(db2)));

        let db4 = cache.get(url1).unwrap();

        assert_ne!(db3, db4);
    }
}
