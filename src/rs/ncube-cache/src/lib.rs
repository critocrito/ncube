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
}
