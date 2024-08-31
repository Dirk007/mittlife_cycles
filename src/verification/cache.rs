use std::collections::HashMap;

use anyhow::Result;

use super::public_key::PublicKey;

const DEFAULT_CACHE_MAX_AGE: std::time::Duration = std::time::Duration::from_secs(60 * 60 * 24 * 30);

#[async_trait::async_trait]
pub trait Cache<K: PublicKey + Clone> {
    /// Returns an Option containing the public key for the given serial, or None if not found in the cache
    async fn get(&self, serial: &str) -> Option<K>;
    /// Sets the public key for the given serial in the cache, overwriting any existing value
    async fn set(&mut self, serial: String, value: K) -> Result<()>;
    /// Removes all keys from the cache that have exceeded their maximum age
    async fn retire_keys(&mut self);
}

/// A simple in-memory cache that stores keys and their associated public keys
pub struct MemoryCache<K: PublicKey + Clone + Send + Sync> {
    keys: HashMap<String, K>,
    max_age: std::time::Duration,
}

impl<K: PublicKey + Clone + Send + Sync> Default for MemoryCache<K> {
    fn default() -> Self {
        MemoryCache {
            keys: HashMap::new(),
            max_age: DEFAULT_CACHE_MAX_AGE,
        }
    }
}

#[async_trait::async_trait]
impl<K: PublicKey + Clone + Send + Sync> Cache<K> for MemoryCache<K> {
    async fn get(&self, serial: &str) -> Option<K> {
        log::debug!("Looking for key {}", serial);
        if let Some(found) = self.keys.get(serial) {
            log::debug!("Found key {} in cache", serial);
            return Some(found.clone());
        }
        log::debug!("Key {} not found in cache", serial);
        None
    }

    async fn set(&mut self, serial: String, value: K) -> Result<()> {
        log::debug!("Setting new content for key {}", serial);
        self.keys.insert(serial, value);
        Ok(())
    }

    async fn retire_keys(&mut self) {
        log::debug!("Retiring keys older than {} seconds", self.max_age.as_secs());
        let old = self.keys.len();
        self.keys.retain(|_, key| key.age() < self.max_age);
        log::debug!("Retired {} keys", old - self.keys.len());
    }
}
