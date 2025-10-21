//! Glue for [`diesel`].

use std::time::Duration;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use crate::prelude::*;

pub mod handle;
pub mod handle_config;
pub mod outer;
pub mod inner;
pub mod path;
pub mod glue;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::handle::*;
    pub use super::handle_config::*;
    pub use super::path::*;
    pub use super::inner::*;
    pub use super::outer::*;
    pub use super::glue::*;

    pub use super::{cache, INIT_CACHE_COMMAND, NewCacheEntry, CacheEntryKeys, CacheEntryValues, ReadFromCacheError, WriteToCacheError, ConnectCacheError};
}

diesel::table! {
    /// The table containing cache entries.
    cache (subject, key) {
        /// The subject of the entry.
        subject -> Text,
        /// The key of the entry.
        key -> Text,
        /// The value of the entry.
        value -> Nullable<Text>,
        /// The time the original computation took.
        duration -> Float
    }
}

/// The Sqlite command to initialize the cache database.
pub const INIT_CACHE_COMMAND: &str = r#"CREATE TABLE cache (
    subject TEXT NOT NULL,
    "key" TEXT NOT NULL,
    value TEXT,
    duration FLOAT NOT NULL,
    UNIQUE(subject, "key") ON CONFLICT REPLACE
)"#;

/// A new entry for the cache database.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Insertable)]
#[diesel(table_name = cache)]
pub struct NewCacheEntry<'a> {
    /// The subject of the new entry.
    pub subject: &'a str,
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
    /// The subject of the entry.
    pub subject: &'a str,
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
