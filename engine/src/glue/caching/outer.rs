//! The home of [`Cache`].

use std::sync::Mutex;
use std::time::Duration;

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
/// assert_eq!(cache.read("category", "key", false).unwrap(), None);
/// cache.write("category", "key", None, Default::default()).unwrap();
/// assert_eq!(cache.read("category", "key", false).unwrap(), Some(None));
/// cache.write("category", "key", Some("value"), Default::default()).unwrap();
/// assert_eq!(cache.read("category", "key", false).unwrap(), Some(Some("value".into())));
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
    /// Reads from the cache.
    /// # Errors
    /// If the call to [`InnerCache::read`] returns an error, that error is returned.
    pub fn read(&self, category: &str, key: &str, delay: bool) -> Result<Option<Option<String>>, ReadFromCacheError> {
        self.0.lock().map_err(|e| ReadFromCacheError::MutexPoisonError(e.to_string()))?.read(category, key, delay)
    }

    /// Writes to the cache.
    ///
    /// If an entry for the `category` and `key` already exists, overwrites it.
    /// # Errors
    /// If the call to [`InnerCache::write`] returns an error, that error is returned.
    pub fn write(&self, category: &str, key: &str, value: Option<&str>, duration: Duration) -> Result<(), WriteToCacheError> {
        self.0.lock().map_err(|e| WriteToCacheError::MutexPoisonError(e.to_string()))?.write(category, key, value, duration)
    }
}
