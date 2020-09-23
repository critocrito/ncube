use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Mutex, RwLock};
use tracing::trace;

#[derive(Debug)]
pub struct GuardedCache<T>(RwLock<HashMap<String, Mutex<T>>>)
where
    T: Debug + Clone;

impl<T> Default for GuardedCache<T>
where
    T: Debug + Clone,
{
    fn default() -> Self {
        Self(RwLock::new(HashMap::new()))
    }
}

impl<T> GuardedCache<T>
where
    T: Debug + Clone,
{
    pub fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<T> {
        let trimmed = key.trim().to_string();
        let cache = self.0.read().expect("RwLock poisoned");
        if let Some(elem) = cache.get(&trimmed) {
            let elem = elem.lock().expect("Mutex poisoned");
            let entry = elem.clone();
            trace!("Element {} served from cache", key);
            return Some(entry);
        }
        trace!("element {} not in cache", key);
        None
    }

    pub fn put(&self, key: &str, entry: T) {
        let trimmed = key.trim().to_string();
        let mut cache = self.0.write().expect("RwLock poisoned");
        cache.entry(trimmed).or_insert_with(|| {
            trace!("new element {} inserted into cache", key);
            Mutex::new(entry)
        });
    }

    pub fn delete(&mut self, key: &str) {
        let trimmed = key.trim().to_string();
        let cache = self.0.get_mut().expect("RwLock poisoned");

        match cache.remove(&trimmed) {
            None => trace!("element {} not in cache", key),
            Some(_) => trace!("removed element {} from cache", key),
        };
    }

    pub fn has(&self, key: &str) -> bool {
        let trimmed = key.trim().to_string();
        let cache = self.0.read().expect("RwLock poisoned");
        match cache.get(&trimmed) {
            Some(_) => {
                trace!("element {} found in cache", key);
                true
            }
            _ => {
                trace!("element {} not found in cache", key);
                false
            }
        }
    }

    pub fn reset(&self, key: &str, entry: T) {
        let trimmed = key.trim().to_string();
        let mut cache = self.0.write().expect("RwLock poisoned");
        cache.insert(trimmed, Mutex::new(entry));
    }

    pub fn all(&self) -> Vec<(String, T)> {
        let cache = self.0.write().expect("RwLock poisoned");
        cache
            .iter()
            .map(|(k, v)| {
                let elem = v.lock().expect("Mutex poisoned");
                let entry = elem.clone();

                (k.to_string(), entry)
            })
            .collect()
    }
}

// FIXME: I don't test any concurrent access to the cache.
#[cfg(test)]
mod cache_tests {
    use super::*;

    #[test]
    fn maybe_return_elements() {
        let cache: GuardedCache<i32> = GuardedCache::new();

        assert_eq!(cache.get("one"), None);
    }

    #[test]
    fn elements_can_be_put_only_once() {
        let cache: GuardedCache<i32> = GuardedCache::new();

        cache.put("one", 1);
        cache.put("one", 23);

        assert_eq!(cache.get("one"), Some(1));
    }

    #[test]
    fn elements_can_be_updated() {
        let cache: GuardedCache<i32> = GuardedCache::new();

        cache.put("one", 1);

        assert_eq!(cache.get("one"), Some(1));

        cache.reset("one", 23);

        assert_eq!(cache.get("one"), Some(23));
    }

    #[test]
    fn test_if_elements_exist_in_cache() {
        let cache: GuardedCache<i32> = GuardedCache::new();

        assert_eq!(cache.has("one"), false);

        cache.put("one", 1);

        assert_eq!(cache.has("one"), true);
    }

    #[test]
    fn list_all_elements_of_the_cache() {
        let cache: GuardedCache<i32> = GuardedCache::new();

        cache.put("one", 1);

        assert_eq!(cache.all(), vec![("one".to_string(), 1)]);
    }

    #[test]
    fn remove_element_by_key() {
        let mut cache: GuardedCache<i32> = GuardedCache::new();

        assert_eq!(cache.has("one"), false);

        cache.put("one", 1);

        assert!(cache.has("one"));

        cache.delete("one");

        assert_eq!(cache.has("one"), false);
    }
}
