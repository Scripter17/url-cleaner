//! Caching to allow for only expanding redirects the first time you encounter them.

#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::sync::Mutex;
use std::time::Duration;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
#[expect(unused_imports, reason = "Used in docs.")]
use diesel::query_builder::SqlQuery;
use rand::TryRngCore;

#[expect(unused_imports, reason = "Used in docs.")]
use crate::types::*;

pub mod path;
pub use path::*;
pub mod inner;
pub use inner::*;
pub mod outer;
pub use outer::*;
pub mod glue;
pub use glue::*;

/// A wrapper around a [`Cache`] for optional security features like anti-cache detection artificial delays.
/// # Examples
/// ```
/// use url_cleaner_engine::glue::*;
/// use std::time::Duration;
///
/// let cache = CacheHandle {
///     cache: &Default::default(),
///     config: Default::default()
/// };
///
/// assert_eq!(cache.read(CacheEntryKeys { category: "category", key: "key" }).unwrap().map(|entry| entry.value), None);
/// cache.write(NewCacheEntry { category: "category", key: "key", value: None, duration: Default::default() }).unwrap();
/// assert_eq!(cache.read(CacheEntryKeys { category: "category", key: "key" }).unwrap().map(|entry| entry.value), Some(None));
/// cache.write(NewCacheEntry { category: "category", key: "key", value: Some("value"), duration: Default::default() }).unwrap();
/// assert_eq!(cache.read(CacheEntryKeys { category: "category", key: "key" }).unwrap().map(|entry| entry.value), Some(Some("value".into())));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CacheHandle<'a> {
    /// The [`Cache`].
    pub cache: &'a Cache,
    /// The [`CacheHandleConfig`].
    pub config: CacheHandleConfig
}

/// Configuration for how a [`CacheHandle`] should behave.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CacheHandleConfig {
    /// If [`true`], delay cache reads by about as long as the inital computation took.
    ///
    /// This reduces the ability for websites to tell if you have a URL cached.
    pub delay: bool
}

impl CacheHandle<'_> {
    /// Reads from the cache.
    /// # Errors
    /// If the call to [`InnerCache::read`] returns an error, that error is returned.
    /// # Panics
    /// If, somehow, [`rand::rngs::OsRng`] doesn't work, this panics when [`Self::config`]'s [`CacheHandleConfig::delay`] is [`true`].
    pub fn read(&self, keys: CacheEntryKeys) -> Result<Option<CacheEntryValues>, ReadFromCacheError> {
        let ret = self.cache.read(keys)?;
        if self.config.delay && let Some(CacheEntryValues {duration, ..}) = ret {
            let between_neg_1_and_1 = rand::rngs::OsRng.try_next_u32().expect("Os RNG to be available") as f32 / f32::MAX * 2.0 - 1.0;
            std::thread::sleep(duration.mul_f32(1.0 + between_neg_1_and_1 / 8.0));
        }
        Ok(ret)
    }

    /// Writes to the cache.
    ///
    /// If an entry for the `category` and `key` already exists, overwrites it.
    /// # Errors
    /// If the call to [`InnerCache::write`] returns an error, that error is returned.
    pub fn write(&self, entry: NewCacheEntry) -> Result<(), WriteToCacheError> {
        self.cache.write(entry)
    }
}

diesel::table! {
    /// The table containing cache entries.
    cache (category, key) {
        /// The category of the entry.
        category -> Text,
        /// The key of the entry.
        key -> Text,
        /// The value of the entry.
        value -> Nullable<Text>,
        /// The time the original computation took.
        duration -> Float
    }
}

/// The Sqlite command to initialize the cache database.
pub const DB_INIT_COMMAND: &str = r#"CREATE TABLE cache (
    category TEXT NOT NULL,
    "key" TEXT NOT NULL,
    value TEXT,
    duration FLOAT NOT NULL,
    UNIQUE(category, "key") ON CONFLICT REPLACE
)"#;

/// A new entry for the cache database.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Insertable)]
#[diesel(table_name = cache)]
pub struct NewCacheEntry<'a> {
    /// The category of the new entry.
    pub category: &'a str,
    /// The key of the new entry.
    pub key: &'a str,
    /// The value of the new entry.
    pub value: Option<&'a str>,
    /// The time the original computation took.
    #[diesel(serialize_as = DurationGlue)]
    pub duration: Duration
}

/// The keys of a cache entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = cache)]
pub struct CacheEntryKeys<'a> {
    /// The category of the entry.
    pub category: &'a str,
    /// The key of the entry.
    pub key: &'a str,
}

/// The values of a cache entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = cache)]
pub struct CacheEntryValues {
    /// The value of the entry.
    pub value: Option<String>,
    /// The time the original computation took.
    #[diesel(deserialize_as = DurationGlue)]
    pub duration: Duration
}

/// The enum of errors [`Cache::read`] and [`InnerCache::read`] can return.
#[derive(Debug, Error)]
pub enum ReadFromCacheError {
    /// Returned when a call to [`Mutex::lock`] returns an error.
    #[error("{0}")]
    MutexPoisonError(String),
    /// Returned when a [`diesel::result::Error`] is encountered.
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),
    /// Returned when a [`ConnectCacheError`] is encountered.
    #[error(transparent)]
    ConnectCacheError(#[from] ConnectCacheError)
}

/// The enum of errors [`Cache::read`] and [`InnerCache::read`] can return.
#[derive(Debug, Error)]
pub enum WriteToCacheError {
    /// Returned when a call to [`Mutex::lock`] returns an error.
    #[error("{0}")]
    MutexPoisonError(String),
    /// Returned when a [`diesel::result::Error`] is encountered.
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),
    /// Returned when a [`ConnectCacheError`] is encountered.
    #[error(transparent)]
    ConnectCacheError(#[from] ConnectCacheError)
}

/// The enum of errors that [`InnerCache::connect`] can return.
#[derive(Debug, Error)]
pub enum ConnectCacheError {
    /// Returned when a [`diesel::ConnectionError`] is encountered.
    #[error(transparent)]
    ConnectionError(#[from] diesel::ConnectionError),
    /// Returned when a [`std::io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Returned when a [`diesel::result::Error`] is encountered.
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error)
}
