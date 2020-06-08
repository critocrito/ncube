use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Mutex, RwLock};
use tracing::{instrument, trace};

use crate::db::Database;

/// Cache database connectors.
///
/// # Example
///
/// ```
/// use ncubed::cache;
/// use ncubed::{db, db::sqlite};
///
/// let cache = cache::DatabaseCache::new();
///
/// let url = "sqlite://:memory:";
/// let db = sqlite::Database::from_str(url, 1).unwrap();
///
/// assert!(cache.get(url).is_none());
///
/// cache.put(url, db::Database::Sqlite(Box::new(db)));
///
/// let db1 = cache.get(url).unwrap();
/// let db2 = cache.get(url).unwrap();
///
/// assert_eq!(db1, db2);
/// ```
#[derive(Debug)]
pub struct DatabaseCache(RwLock<HashMap<String, Mutex<Database>>>);

impl Default for DatabaseCache {
    fn default() -> Self {
        Self(RwLock::new(HashMap::new()))
    }
}

impl DatabaseCache {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    #[instrument]
    pub fn get(&self, key: &str) -> Option<Database> {
        let trimmed = key.trim().to_string();
        let cache = self.0.read().expect("RwLock poisoned");
        if let Some(elem) = cache.get(&trimmed) {
            let elem = elem.lock().expect("Mutex poisoned");
            let db = elem.clone();
            trace!("database served from cache");
            return Some(db);
        }
        trace!("database not in cache");
        None
    }

    #[instrument]
    pub fn put(&self, key: &str, db: Database) {
        let trimmed = key.trim().to_string();
        let mut cache = self.0.write().expect("RwLock poisoned");
        cache.entry(trimmed).or_insert_with(|| {
            trace!("new database inserted into cache");
            Mutex::new(db)
        });
    }

    #[instrument]
    pub fn has(&self, key: &str) -> bool {
        let trimmed = key.trim().to_string();
        let cache = self.0.read().expect("RwLock poisoned");
        match cache.get(&trimmed) {
            Some(_) => {
                trace!("database found in cache");
                true
            }
            _ => {
                trace!("database not found in cache");
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::sqlite;

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
}
