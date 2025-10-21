//! The home if [`InnerCache`].

use std::cell::OnceCell;

use diesel::prelude::*;
#[expect(unused_imports, reason = "Used in docs.")]
use diesel::query_builder::SqlQuery;

use crate::prelude::*;

/// A lazily connected connection to a Sqlite database.
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
/// use std::time::Duration;
///
/// // Note the mutability.
/// let mut cache = InnerCache::new(CachePath::Memory);
///
/// assert_eq!(cache.read(CacheEntryKeys { subject: "subject", key: "key" }).unwrap().map(|entry| entry.value), None);
/// cache.write(NewCacheEntry { subject: "subject", key: "key", value: None, duration: Default::default() }).unwrap();
/// assert_eq!(cache.read(CacheEntryKeys { subject: "subject", key: "key" }).unwrap().map(|entry| entry.value), Some(None));
/// cache.write(NewCacheEntry { subject: "subject", key: "key", value: Some("value"), duration: Default::default() }).unwrap();
/// assert_eq!(cache.read(CacheEntryKeys { subject: "subject", key: "key" }).unwrap().map(|entry| entry.value), Some(Some("value".into())));
/// ```
#[derive(Default)]
pub struct InnerCache {
    /// The path of the database.
    path: CachePath,
    /// The connection to the database.
    connection: OnceCell<SqliteConnection>
}

impl ::core::fmt::Debug for InnerCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.debug_struct("InnerCache")
            .field("path", &self.path)
            .field("connection", if self.connection.get().is_some() {&"OnceCell(..)"} else {&"OnceCell(<uninit>)"})
            .finish()
    }
}

impl InnerCache {
    /// Create a new unconnected [`Self`].
    pub fn new(path: CachePath) -> Self {
        path.into()
    }
}

impl PartialEq for InnerCache {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}
impl Eq for InnerCache {}

impl<T: Into<CachePath>> From<T> for InnerCache {
    fn from(value: T) -> Self {
        Self {
            path: value.into(),
            ..Default::default()
        }
    }
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
            if let CachePath::Path(path) = &self.path && !std::fs::exists(path)? {
                needs_init = true;
                std::fs::File::create_new(path)?;
            }
            let mut connection = SqliteConnection::establish(self.path.as_str())?;
            if needs_init {
                diesel::sql_query(INIT_CACHE_COMMAND).execute(&mut connection)?;
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
    pub fn read(&mut self, keys: CacheEntryKeys) -> Result<Option<CacheEntryValues>, ReadFromCacheError> {
        debug!(InnerCache::read, self, keys);
        Ok(cache::dsl::cache
            .filter(cache::dsl::subject.eq(keys.subject))
            .filter(cache::dsl::key.eq(keys.key))
            .limit(1)
            .select(CacheEntryValues::as_select())
            .load(self.connect()?)?
            .first().cloned())
    }

    /// Writes to the database, overwriting the entry the equivalent call to [`Self::read`] would return.
    /// # Errors
    /// If the call to [`Self::connect`] returns an error, that error is returned.
    ///
    /// If the call to [`RunQueryDsl::get_result`] returns an error, that error is returned.
    pub fn write(&mut self, entry: NewCacheEntry) -> Result<(), WriteToCacheError> {
        debug!(InnerCache::write, self, entry);
        diesel::insert_into(cache::table)
            .values(entry)
            .execute(self.connect()?)?;
        Ok(())
    }
}

impl From<InnerCache> for (CachePath, OnceCell<SqliteConnection>) {
    fn from(value: InnerCache) -> Self {
        (value.path, value.connection)
    }
}
