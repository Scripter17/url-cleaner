//! Caching to allow for only expanding redirects the first time you encounter them.

#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::sync::Mutex;
use std::time::Duration;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
#[expect(unused_imports, reason = "Used in docs.")]
use diesel::query_builder::SqlQuery;

#[expect(unused_imports, reason = "Used in docs.")]
use crate::types::*;

pub mod path;
pub use path::*;
pub mod inner;
pub use inner::*;
pub mod outer;
pub use outer::*;

/// A per-[`Job`] handle of a possibly multi-[`Job`] [`Cache`].
/// # Examples
/// ```
/// use url_cleaner_engine::glue::*;
/// use std::time::Duration;
///
/// let cache = CacheHandle {
///     cache: &Default::default(),
///     delay: false
/// };
///
/// assert_eq!(cache.read("category", "key").unwrap(), None);
/// cache.write("category", "key", None, Default::default()).unwrap();
/// assert_eq!(cache.read("category", "key").unwrap(), Some(None));
/// cache.write("category", "key", Some("value"), Default::default()).unwrap();
/// assert_eq!(cache.read("category", "key").unwrap(), Some(Some("value".into())));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CacheHandle<'a> {
    /// The [`Cache`].
    pub cache: &'a Cache,
    /// If [`true`], delay cache reads by about as long as the inital computation took.
    ///
    /// This reduces the ability for websites to tell if you have a URL cached.
    pub delay: bool
}

impl CacheHandle<'_> {
    /// Reads from the cache.
    /// # Errors
    /// If the call to [`InnerCache::read`] returns an error, that error is returned.
    pub fn read(&self, category: &str, key: &str) -> Result<Option<Option<String>>, ReadFromCacheError> {
        self.cache.read(category, key, self.delay)
    }

    /// Writes to the cache.
    ///
    /// If an entry for the `category` and `key` already exists, overwrites it.
    /// # Errors
    /// If the call to [`InnerCache::write`] returns an error, that error is returned.
    pub fn write(&self, category: &str, key: &str, value: Option<&str>, duration: Duration) -> Result<(), WriteToCacheError> {
        self.cache.write(category, key, value, duration)
    }
}

diesel::table! {
    /// The table containing cache entries.
    cache (category, key) {
        /// The "category" of the entry.
        category -> Text,
        /// The "key" of the entry.
        key -> Text,
        /// The value of the entry.
        value -> Nullable<Text>,
        /// The time the original computation took.
        duration_ms -> Integer
    }
}

/// The Sqlite command to initialize the cache database.
pub const DB_INIT_COMMAND: &str = r#"CREATE TABLE cache (
    category TEXT NOT NULL,
    "key" TEXT NOT NULL,
    value TEXT,
    duration_ms INTEGER,
    UNIQUE(category, "key") ON CONFLICT REPLACE
)"#;

/// An entry in the cache database.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = cache)]
pub struct CacheEntry {
    /// The category of the entry.
    pub category: String,
    /// The key of the entry.
    pub key: String,
    /// The value of the entry.
    pub value: Option<String>,
    /// The time the original computation took.
    pub duration_ms: i32
}

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
    pub duration_ms: i32
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
