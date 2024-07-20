#![doc = "Allows caching to an SQLite file."]

use std::sync::Mutex;
use std::str::FromStr;
use std::path::Path;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[allow(clippy::missing_docs_in_private_items, missing_docs)]
mod schema;
pub use schema::cache;

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
pub struct CacheHandler(pub Mutex<SqliteConnection>);

impl ::core::fmt::Debug for CacheHandler {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "CacheHandler")
    }
}

impl From<SqliteConnection> for CacheHandler {
    fn from(value: SqliteConnection) -> Self {
        Self(Mutex::new(value))
    }
}

/// The errors the various [`TryFrom::try_from`] methods return for [`CacheHandler`].
#[derive(Debug, Error)]
pub enum MakeCacheHandlerError {
    /// Returned when making a [`CacheHandler`] from a non-UTF-8 [`Path`].
    #[error("The cache's path was not UTF-8")]
    CachePathIsNotUtf8,
    /// Returned when a [`ConnectionError`] is encountered.
    #[error(transparent)]
    ConnectionError(#[from] ConnectionError)
}

impl FromStr for CacheHandler {
    type Err = MakeCacheHandlerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SqliteConnection::establish(s)?.into())
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

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
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
    DieselError(#[from] diesel::result::Error)
}

/// The enum of errors [`CacheHandler::write_to_cache`] can return.
#[derive(Debug, Error)]
pub enum WriteToCacheError {
    /// Returned when the inner [`Mutex`] is poisoned.
    #[error("{0}")]
    MutexPoisonError(String),
    /// Returned when a [`diesel::result::Error`] is encountered.
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error)
}

impl CacheHandler {
    /// Reads a string from the cache.
    /// # Errors
    /// If the call to [`Mutex::lock`] returns an error, that error is returned.
    /// 
    /// If the call to [`RunQueryDsl::get_result`] returns an error, that error is returned.
    pub fn read_from_cache(&self, category: &str, key: &str) -> Result<Option<Option<String>>, ReadFromCacheError> {
        Ok(cache::dsl::cache
            .filter(cache::dsl::category.eq(category))
            .filter(cache::dsl::key.eq(key))
            .limit(1)
            .select(CacheEntry::as_select())
            .load(&mut *self.0.lock().map_err(|e| ReadFromCacheError::MutexPoisonError(e.to_string()))?)?
            .first()
            .map(|cache_entry| cache_entry.value.to_owned()))
    }

    /// Writes a string to the cache.
    /// # Errors
    /// If the call to [`Mutex::lock`] returns an error, that error is returned.
    /// 
    /// If the call to [`RunQueryDsl::get_result`] returns an error, that error is returned.
    pub fn write_to_cache(&self, category: &str, key: &str, value: Option<&str>) -> Result<(), WriteToCacheError> {
        diesel::insert_into(cache::table)
            .values(&NewCacheEntry {category, key, value})
            .returning(CacheEntry::as_returning())
            .get_result(&mut *self.0.lock().map_err(|e| WriteToCacheError::MutexPoisonError(e.to_string()))?)?;
        Ok(())
    }
}
