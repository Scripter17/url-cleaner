#![doc = "Allows caching to an SQLite file."]

use std::sync::Mutex;
use std::str::FromStr;
use std::path::Path;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use crate::util::*;

#[allow(clippy::missing_docs_in_private_items, missing_docs)]
mod schema;
pub use schema::cache;

/// Creating a [`CacheHandler`] from a [`Path`] writes this to the path if there's nothing there.
pub const EMPTY_CACHE: &[u8] = include_bytes!("../../empty-cache.sqlite");

/// An entry in the [`cache`] table.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = cache)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CacheEntry {
    /// The ID of the entry.
    pub id: i32,
    /// The category of the entry.
    pub category: String,
    /// The key of the entry.
    pub key: String,
    /// The value of the entry.
    pub value: Option<String>
}

/// An addition to the [`cache`] table.
#[derive(Debug, PartialEq, Eq, Serialize, Insertable)]
#[diesel(table_name = cache)]
pub struct NewCacheEntry<'a> {
    /// The category of the new entry.
    pub category: &'a str,
    /// The key of the new entry.
    pub key: &'a str,
    /// The value of the new entry.
    pub value: Option<&'a str>
}

/// Convenience wrapper to contain the annoyingness of it all.
#[derive(Debug)]
pub struct CacheHandler(Mutex<InnerCacheHandler>);

/// The internals of [`CacheHandler`] that handles lazily connecting.
pub enum InnerCacheHandler {
    /// The unconnected state. Should be fast to construct.
    Unconnected {
        /// The path to connect to,
        path: String
    },
    /// The connected state. Slow to construct. Make using [`Self::connect`].
    Connected {
        /// The path being connected to. Used for [`Debug`].
        path: String,
        /// The actual [`SqliteConnection`]
        connection: SqliteConnection
    }
}

impl ::core::fmt::Debug for InnerCacheHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Self::Unconnected {path} => {
                f.debug_struct("Unconnected")
                    .field("path", &path)
                    .finish()
            },
            Self::Connected {path, ..} => {
                f.debug_struct("Connected")
                    .field("path", &path)
                    .field("connection", &"...")
                    .finish()
            }
        }
    }
}

/// The errors the various [`TryFrom::try_from`] methods return for [`CacheHandler`].
#[derive(Debug, Error)]
pub enum MakeCacheHandlerError {
    /// Returned when making a [`CacheHandler`] from a non-UTF-8 [`Path`].
    #[error("The cache's path was not UTF-8")]
    CachePathIsNotUtf8,
    /// Returned when a [`std::io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Returned when a [`ConnectionError`] is encountered.
    #[error(transparent)]
    ConnectionError(#[from] ConnectionError)
}

impl FromStr for CacheHandler {
    type Err = MakeCacheHandlerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CacheHandler(Mutex::new(InnerCacheHandler::Unconnected { path: s.to_string() })))
    }
}

impl TryFrom<&str> for CacheHandler {
    type Error = MakeCacheHandlerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<&Path> for CacheHandler {
    type Error = MakeCacheHandlerError;

    /// Makes the file if it doesn't exist.
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        if !value.exists() {
            std::fs::write(value, EMPTY_CACHE)?;
        }
        Self::from_str(value.to_str().ok_or(MakeCacheHandlerError::CachePathIsNotUtf8)?)
    }
}

/// The enum of errors [`CacheHandler::read_from_cache`] can return.
#[derive(Debug, Error)]
pub enum ReadFromCacheError {
    /// Returned when the inner [`Mutex`] is poisoned.
    #[error("{0}")]
    MutexPoisonError(String),
    /// Returned when a [`diesel::result::Error`] is encountered.
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),
    /// Returned when a [`ConnectCacheError`] is encountered.
    #[error(transparent)]
    ConnectCacheError(#[from] ConnectCacheError)
}

/// The enum of errors [`CacheHandler::write_to_cache`] can return.
#[derive(Debug, Error)]
pub enum WriteToCacheError {
    /// Returned when the inner [`Mutex`] is poisoned.
    #[error("{0}")]
    MutexPoisonError(String),
    /// Returned when a [`diesel::result::Error`] is encountered.
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),
    /// Returned when a [`ConnectCacheError`] is encountered.
    #[error(transparent)]
    ConnectCacheError(#[from] ConnectCacheError)
}

impl CacheHandler {
    /// Reads a string from the cache.
    /// # Errors
    /// If the call to [`Mutex::lock`] returns an error, that error is returned.
    /// 
    /// If the call to [`RunQueryDsl::get_result`] returns an error, that error is returned.
    pub fn read_from_cache(&self, category: &str, key: &str) -> Result<Option<Option<String>>, ReadFromCacheError> {
        debug!(CacheHandler::read_from_cache, self, category, key);
        Ok(cache::dsl::cache
            .filter(cache::dsl::category.eq(category))
            .filter(cache::dsl::key.eq(key))
            .limit(1)
            .select(CacheEntry::as_select())
            .load(self.0.lock().map_err(|e| ReadFromCacheError::MutexPoisonError(e.to_string()))?.connect()?)?
            .first()
            .map(|cache_entry| cache_entry.value.to_owned()))
    }

    /// Writes a string to the cache.
    /// # Errors
    /// If the call to [`Mutex::lock`] returns an error, that error is returned.
    /// 
    /// If the call to [`RunQueryDsl::get_result`] returns an error, that error is returned.
    pub fn write_to_cache(&self, category: &str, key: &str, value: Option<&str>) -> Result<(), WriteToCacheError> {
        debug!(CacheHandler::write_to_cache, self, category, key, value);
        diesel::insert_into(cache::table)
            .values(&NewCacheEntry {category, key, value})
            .returning(CacheEntry::as_returning())
            .get_result(self.0.lock().map_err(|e| WriteToCacheError::MutexPoisonError(e.to_string()))?.connect()?)?;
        Ok(())
    }
}

/// The enum of errors [`InnerCacheHandler::connect`] can return.
#[derive(Debug, Error)]
pub enum ConnectCacheError {
    /// Returned when a [`diesel::ConnectionError`] is encountered.
    #[error(transparent)]
    ConnectionError(#[from] diesel::ConnectionError)
}

impl InnerCacheHandler {
    /// # Errors
    /// If the call to [`SqliteConnection::establish`] returns an error, that error is returned.
    pub fn connect(&mut self) -> Result<&mut SqliteConnection, ConnectCacheError> {
        #[cfg(feature = "debug")]
        if matches!(self, Self::Unconnected{..}) {
            debug!(InnerCacheHandler::connect, "actually connecting", self);
        } else if matches!(self, Self::Connected{..}) {
            debug!(InnerCacheHandler::connect, "already connected", self);
        }
        Ok(match self {
            Self::Unconnected { path } => {
                *self = InnerCacheHandler::Connected {
                    connection: SqliteConnection::establish(path)?,
                    path: path.clone()
                };
                match self {
                    Self::Connected { ref mut connection, .. } => connection,
                    _ => unreachable!()
                }
            },
            Self::Connected { ref mut connection, .. } => connection
        })
    }
}
