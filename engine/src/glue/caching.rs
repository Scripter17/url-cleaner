//! Caching to allow for only expanding redirects the first time you encounter them.

pub mod path;
pub use path::*;
pub mod inner;
pub use inner::*;
pub mod outer;
pub use outer::*;


use thiserror::Error;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
#[expect(unused_imports, reason = "Used in docs.")]
use diesel::query_builder::SqlQuery;


diesel::table! {
    /// The table containing cache entries.
    cache (category, key) {
        /// The "category" of the entry.
        category -> Text,
        /// The "key" of the entry.
        key -> Text,
        /// The value of the entry.
        value -> Nullable<Text>,
    }
}

/// The Sqlite command to initialize the cache database.
pub const DB_INIT_COMMAND: &str = r#"CREATE TABLE cache (
    category TEXT NOT NULL,
    "key" TEXT NOT NULL,
    value TEXT,
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
    pub value: Option<String>
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
    pub value: Option<&'a str>
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
