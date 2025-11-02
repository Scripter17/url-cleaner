//! Glue for [`rusqlite`].

use std::time::Duration;

use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::prelude::*;

pub mod location;
pub mod config;
pub mod inner;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::location::*;
    pub use super::config::*;
    pub use super::inner::*;

    pub use super::{
        Cache,
        NewCacheEntry, CacheEntryKeys, CacheEntryValues,
        LockCacheError, ReadFromCacheError, WriteToCacheError
    };
}
use prelude::*;

/// An [`InnerCache`] and a [`CacheConfig`].
///
/// I promise I tried making it possible to write `cache: &Default::default()`.
/// URL Cleaner Site needing [`CacheConfig::delay`] to be configurable per-[`Job`] just made it mess.
/// # Examples
/// ```
/// use std::time::{Instant, Duration};
/// use url_cleaner_engine::prelude::*;
///
/// let inner_cache = Default::default();
/// 
/// let cache = Cache {
///     config: Default::default(),
///     inner: &inner_cache
/// };
///
/// assert_eq!(
///     cache.read(CacheEntryKeys {
///         subject: "a",
///         key: "b"
///     }).unwrap(),
///     None
/// );
///
/// cache.write(NewCacheEntry {
///     subject: "a",
///     key: "b",
///     value: Some("c"),
///     duration: Duration::from_secs(1)
/// }).unwrap();
///
/// assert_eq!(
///     cache.read(CacheEntryKeys {
///         subject: "a",
///         key: "b"
///     }).unwrap(),
///     Some(CacheEntryValues {
///         value: Some("c".to_string()),
///         duration: Duration::from_secs(1)
///     })
/// );
///
/// let cache = Cache {
///     config: CacheConfig {
///         delay: true,
///         ..Default::default()
///     },
///     inner: &inner_cache
/// };
///
/// let start = Instant::now();
/// cache.read(CacheEntryKeys {
///     subject: "a",
///     key: "b"
/// }).unwrap();
/// let elapsed = start.elapsed();
/// // The delay is a random value between 7/8 abd 9/8 times the original duration.
/// assert!( 0.875 < elapsed.as_secs_f64() && elapsed.as_secs_f64() < 1.3, "{elapsed:?}");
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Cache<'a> {
    /// The [`CacheConfig`].
    pub config: CacheConfig,
    /// The [`InnerCache`].
    pub inner: &'a InnerCache
}

impl Cache<'_> {
    /// Read an entry from the cache.
    /// # Errors
    #[doc = edoc!(callerr(InnerCache::read))]
    pub fn read(&self, keys: CacheEntryKeys<'_>) -> Result<Option<CacheEntryValues>, ReadFromCacheError> {
        debug!(Cache::read, self, &keys);

        Ok(if self.config.read {
            let start = std::time::Instant::now();
            let ret = self.inner.read(keys)?;
            let elapsed = start.elapsed();
            // No I do not care that using the duration as a source of randomness is Bad.
            // Okay I care a little but there is so much noise and websites can only get one datapoint before the value is definitely in the cache.
            // I really doubt you can get cache detection out of that.
            if self.config.delay && let Some(CacheEntryValues {ref duration, ..}) = ret && let Some(remaining) = duration.mul_f64(elapsed.as_nanos() as i8 as f64 / 128.0 / 8.0 + 1.0).checked_sub(elapsed) {
                std::thread::sleep(remaining);
            }
            ret
        } else {
            None
        })
    }

    /// Write an entry to the cache.
    /// # Errors
    #[doc = edoc!(callerr(InnerCache::write))]
    pub fn write(&self, entry: NewCacheEntry<'_>) -> Result<(), WriteToCacheError> {
        debug!(Cache::write, self, &entry);

        if self.config.write {
            self.inner.write(entry)?;
        }

        Ok(())
    }
}

/// A new entry for the cache database.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NewCacheEntry<'a> {
    /// The subject of the new entry.
    pub subject: &'a str,
    /// The key of the new entry.
    pub key: &'a str,
    /// The value of the new entry.
    pub value: Option<&'a str>,
    /// The time the original computation took.
    pub duration: Duration
}

/// The keys of a cache entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CacheEntryKeys<'a> {
    /// The subject of the entry.
    pub subject: &'a str,
    /// The key of the entry.
    pub key: &'a str,
}

/// The values of a cache entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheEntryValues {
    /// The value of the entry.
    pub value: Option<String>,
    /// The time the original computation took.
    pub duration: Duration
}

/// The enum of errors [`Cache::read`] and [`InnerCache::read`] can return.
#[derive(Debug, Error)]
pub enum ReadFromCacheError {
    /// Returned when a [`rusqlite::Error`] is encountered.
    #[error(transparent)]
    RusqliteError(#[from] rusqlite::Error),
    /// Returned when a [`LockCacheError`] is encountered.
    #[error(transparent)]
    LockCacheError(#[from] LockCacheError)
}

/// The enum of errors [`Cache::read`] and [`InnerCache::read`] can return.
#[derive(Debug, Error)]
pub enum WriteToCacheError {
    /// Returned when a [`rusqlite::Error`] is encountered.
    #[error(transparent)]
    RusqliteError(#[from] rusqlite::Error),
    /// Returned when a [`LockCacheError`] is encountered.
    #[error(transparent)]
    LockCacheError(#[from] LockCacheError)
}

/// The enum of errors that [`InnerCache::lock`] can return.
#[derive(Debug, Error)]
pub enum LockCacheError {
    /// Returned when a [`rusqlite::Error`] is encountered.
    #[error(transparent)]
    RusqliteError(#[from] rusqlite::Error),
    /// Returned when a [`std::io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
