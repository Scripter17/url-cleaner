//! The home of [`Cache`].

use std::cell::RefCell;
use parking_lot::ReentrantMutex;

use super::*;

/// A shareable [`InnerCache`].
/// # Examples
/// ```
/// use url_cleaner_engine::glue::prelude::*;
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
pub struct Cache(pub ReentrantMutex<RefCell<InnerCache>>);

impl Cache {
    /// Create a new unconnected [`Self`].
    #[allow(dead_code, reason = "Public API.")]
    pub fn new(path: CachePath) -> Self {
        path.into()
    }
}

impl From<InnerCache> for Cache {
    fn from(value: InnerCache) -> Self {
        Self(ReentrantMutex::new(RefCell::new(value)))
    }
}

impl From<CachePath> for Cache {
    fn from(value: CachePath) -> Self {
        Cache::from(InnerCache::from(value))
    }
}

impl Cache {
    /// Reads from the cache.
    /// # Errors
    /// If the call to [`InnerCache::read`] returns an error, that error is returned.
    pub fn read(&self, keys: CacheEntryKeys) -> Result<Option<CacheEntryValues>, ReadFromCacheError> {
        self.0.lock().borrow_mut().read(keys)
    }

    /// Writes to the cache.
    ///
    /// If an entry for the `subject` and `key` already exists, overwrites it.
    /// # Errors
    /// If the call to [`InnerCache::write`] returns an error, that error is returned.
    pub fn write(&self, entry: NewCacheEntry) -> Result<(), WriteToCacheError> {
        self.0.lock().borrow_mut().write(entry)
    }
}
