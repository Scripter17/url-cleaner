//! The home of [`Cache`].

use std::sync::{Mutex, LockResult, MutexGuard};

#[expect(unused_imports, reason = "Used in docs.")]
use diesel::query_builder::SqlQuery;

use super::*;

/// A [`Mutex`]ed [`InnerCache`].
/// # Examples
/// ```
/// use url_cleaner_engine::glue::*;
///
/// // Note the immutability.
/// let cache = Cache::new(CachePath::Memory);
///
/// assert_eq!(cache.read(CacheEntryKeys { subject: "subject", key: "key" }).unwrap().map(|entry| entry.value), None);
/// cache.write(NewCacheEntry { subject: "subject", key: "key", value: None, duration: Default::default() }).unwrap();
/// assert_eq!(cache.read(CacheEntryKeys { subject: "subject", key: "key" }).unwrap().map(|entry| entry.value), Some(None));
/// cache.write(NewCacheEntry { subject: "subject", key: "key", value: Some("value"), duration: Default::default() }).unwrap();
/// assert_eq!(cache.read(CacheEntryKeys { subject: "subject", key: "key" }).unwrap().map(|entry| entry.value), Some(Some("value".into())));
/// ```
#[derive(Debug, Default)]
pub struct Cache(pub Mutex<InnerCache>);

impl Cache {
    /// Create a new unconnected [`Self`].
    #[allow(dead_code, reason = "Public API.")]
    pub fn new(path: CachePath) -> Self {
        path.into()
    }
}

impl From<InnerCache> for Cache {
    fn from(value: InnerCache) -> Self {
        Self(Mutex::new(value))
    }
}

impl From<CachePath> for Cache {
    fn from(value: CachePath) -> Self {
        Cache::from(InnerCache::from(value))
    }
}

impl Cache {
    /// Get the contained [`InnerCache`].
    /// # Errors
    /// If the call to [`Mutex::lock`] returns an error, that error is returned.
    pub fn get_inner(&self) -> LockResult<MutexGuard<'_, InnerCache>> {
        self.0.lock()
    }
    
    /// Reads from the cache.
    /// # Errors
    /// If the call to [`InnerCache::read`] returns an error, that error is returned.
    pub fn read(&self, keys: CacheEntryKeys) -> Result<Option<CacheEntryValues>, ReadFromCacheError> {
        self.0.lock().map_err(|e| ReadFromCacheError::MutexPoisonError(e.to_string()))?.read(keys)
    }

    /// Writes to the cache.
    ///
    /// If an entry for the `subject` and `key` already exists, overwrites it.
    /// # Errors
    /// If the call to [`InnerCache::write`] returns an error, that error is returned.
    pub fn write(&self, entry: NewCacheEntry) -> Result<(), WriteToCacheError> {
        self.0.lock().map_err(|e| WriteToCacheError::MutexPoisonError(e.to_string()))?.write(entry)
    }
}
