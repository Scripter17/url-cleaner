//! Caching to allow for only expanding redirects the first time you encounter them.

use std::sync::Mutex;
use std::str::FromStr;
use std::cell::OnceCell;
use std::path::Path;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
#[expect(unused_imports, reason = "Used in docs.")]
use diesel::query_builder::SqlQuery;

use crate::util::*;

diesel::table! {
    /// The table containing cache entries.
    cache (id) {
        /// The entry's unique ID.
        id -> Integer,
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
    id INTEGER NOT NULL PRIMARY KEY,
    category TEXT NOT NULL,
    "key" TEXT NOT NULL,
    value TEXT
)"#;

/// An entry in the cache database.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = cache)]
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

/// A new entry for the cache database.
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

/// A [`Mutex`]ed [`InnerCache`].
#[derive(Debug, Default)]
pub struct Cache(pub Mutex<InnerCache>);

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

/// A lazily connected connection to the cache database.
#[derive(Default)]
pub struct InnerCache {
    /// The path of the database.
    path: CachePath,
    /// The connection to the database.
    connection: OnceCell<SqliteConnection>
}

impl PartialEq for InnerCache {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}
impl Eq for InnerCache {}

impl From<CachePath> for InnerCache {
    fn from(value: CachePath) -> Self {
        Self {
            path: value,
            connection: Default::default()
        }
    }
}

/// The path of a cache database.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub enum CachePath {
    /// Stores the database in memory, wiping it on program exit.
    ///
    /// Has the string representation of `:memory:`.
    #[default]
    Memory,
    /// A filesystem/network/whatever path.
    Path(String)
}

crate::util::string_or_struct_magic!(CachePath);

impl CachePath {
    /// The cache's path as a [`str`].
    ///
    /// If `self` is [`Self::Memory`], returns `:memory:`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Memory => ":memory:",
            Self::Path(x) => x
        }
    }

    /// The cache's path as a [`Path`].
    ///
    /// If `self` is [`Self::Memory`], returns [`None`].
    ///
    /// If `self` is [`Self::Path`] and the path starts with `file://`, returns the path without the `file://`.
    pub fn as_path(&self) -> Option<&Path> {
        match self {
            Self::Memory => None,
            Self::Path(x) => Some(x.strip_prefix("file://").unwrap_or(x).as_ref())
        }
    }
}

impl AsRef<str> for CachePath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for CachePath {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.to_string().into())
    }
}

impl From<&str> for CachePath {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for CachePath {
    fn from(value: String) -> Self {
        match &*value {
            ":memory:" => Self::Memory,
            _ => Self::Path(value)
        }
    }
}

impl ::core::fmt::Debug for InnerCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.debug_struct("InnerCache")
            .field("path", &self.path)
            .field("connection", if self.connection.get().is_some() {&"OnceCell(..)"} else {&"OnceCell(<uninit>)"})
            .finish()
    }
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

impl Cache {
    /// Reads from the cache.
    /// # Errors
    /// If the call to [`InnerCache::read`] returns an error, that error is returned.
    pub fn read(&self, category: &str, key: &str) -> Result<Option<Option<String>>, ReadFromCacheError> {
        self.0.lock().map_err(|e| ReadFromCacheError::MutexPoisonError(e.to_string()))?.read(category, key)
    }

    /// Writes to the cache.
    ///
    /// If an entry for the `category` and `key` already exists, overwrites it.
    /// # Errors
    /// If the call to [`InnerCache::write`] returns an error, that error is returned.
    pub fn write(&self, category: &str, key: &str, value: Option<&str>) -> Result<(), WriteToCacheError> {
        self.0.lock().map_err(|e| WriteToCacheError::MutexPoisonError(e.to_string()))?.write(category, key, value)
    }
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

impl InnerCache {
    /// Gets the [`CachePath`] of the connection.
    pub fn path(&self) -> &CachePath {
        &self.path
    }

    /// Gets the connection itself, if `self` has been connected via [`Self::connect`] yet.
    pub fn connection(&mut self) -> Option<&mut SqliteConnection> {
        self.connection.get_mut()
    }

    /// Returns the connection, connecting if not already connected.
    /// # Errors
    /// If the call to [`std::fs::exists`] to check if the database exists returns an error, that error is returned.
    ///
    /// If the call to [`std::fs::File::create_new`] to create the database returns an error, that error is returned.
    ///
    /// If the call to [`SqliteConnection::establish`] to connect to the database returns an error, that error is returned.
    ///
    /// If the call to [`SqlQuery::execute`] to initialize the database returns an error, that error is returned.
    #[allow(clippy::missing_panics_doc, reason = "Doesn't panic, but should be replaced with OnceCell::get_or_try_init once that's stable.")]
    pub fn connect(&mut self) -> Result<&mut SqliteConnection, ConnectCacheError> {
        debug!(InnerCache::connect, self);
        if self.connection.get().is_none() {
            let mut needs_init = self.path == CachePath::Memory;
            if let CachePath::Path(path) = &self.path {
                if !std::fs::exists(path)? {
                    needs_init = true;
                    std::fs::File::create_new(path)?;
                }
            }
            let mut connection = SqliteConnection::establish(self.path.as_str())?;
            if needs_init {
                diesel::sql_query(DB_INIT_COMMAND).execute(&mut connection)?;
            }
            self.connection.set(connection).map_err(|_| ()).expect("The connection to have just been confirmed unset.");
        }
        Ok(self.connection.get_mut().expect("The connection to have just been set."))
    }

    /// Disconnects from the database.
    pub fn disconnect(&mut self) {
        let _ = self.connection.take();
    }

    /// Reads from the database.
    /// # Errors
    /// If the call to [`Self::connect`] returns an error, that error is returned.
    ///
    /// If the call to [`RunQueryDsl::load`] returns an error, that error is returned.
    pub fn read(&mut self, category: &str, key: &str) -> Result<Option<Option<String>>, ReadFromCacheError> {
        debug!(InnerCache::read, self, category, key);
        Ok(cache::dsl::cache
            .filter(cache::dsl::category.eq(category))
            .filter(cache::dsl::key.eq(key))
            .limit(1)
            .select(CacheEntry::as_select())
            .load(self.connect()?)?
            .first()
            .map(|cache_entry| cache_entry.value.to_owned()))
    }

    /// Writes to the database, overwriting the entry the equivalent call to [`Self::read`] would return.
    /// # Errors
    /// If the call to [`Self::connect`] returns an error, that error is returned.
    ///
    /// If the call to [`RunQueryDsl::get_result`] returns an error, that error is returned.
    pub fn write(&mut self, category: &str, key: &str, value: Option<&str>) -> Result<(), WriteToCacheError> {
        debug!(InnerCache::write, self, category, key, value);
        diesel::replace_into(cache::table)
            .values(&NewCacheEntry {category, key, value})
            .returning(CacheEntry::as_returning())
            .get_result(self.connect()?)?;
        Ok(())
    }
}

impl From<InnerCache> for (CachePath, OnceCell<SqliteConnection>) {
    fn from(value: InnerCache) -> Self {
        (value.path, value.connection)
    }
}
