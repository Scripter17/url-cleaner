//! The home if [`InnerCache`].

use std::cell::OnceCell;
use std::time::Duration;

use diesel::prelude::*;
#[expect(unused_imports, reason = "Used in docs.")]
use diesel::query_builder::SqlQuery;
use rand::rngs::SmallRng;
use rand::{SeedableRng, Rng};

use crate::util::*;
use super::*;

/// A lazily connected connection to the cache database.
/// # Examples
/// ```
/// use url_cleaner_engine::glue::*;
/// use std::time::Duration;
///
/// // Note the mutability.
/// let mut cache = InnerCache::new(CachePath::Memory);
///
/// assert_eq!(cache.read("category", "key", false).unwrap(), None);
/// cache.write("category", "key", None, Default::default()).unwrap();
/// assert_eq!(cache.read("category", "key", false).unwrap(), Some(None));
/// cache.write("category", "key", Some("value"), Default::default()).unwrap();
/// assert_eq!(cache.read("category", "key", false).unwrap(), Some(Some("value".into())));
/// ```
pub struct InnerCache {
    /// The path of the database.
    path: CachePath,
    /// The connection to the database.
    connection: OnceCell<SqliteConnection>,
    /// RNG source for artificial delays.
    rng: SmallRng
}

impl Default for InnerCache {
    fn default() -> Self {
        Self {
            path: Default::default(),
            connection: Default::default(),
            rng: SmallRng::from_rng(&mut rand::rng())
        }
    }
}

impl ::core::fmt::Debug for InnerCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.debug_struct("InnerCache")
            .field("path", &self.path)
            .field("rng", &self.rng)
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

impl From<CachePath> for InnerCache {
    fn from(value: CachePath) -> Self {
        Self {
            path: value,
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
    pub fn read(&mut self, category: &str, key: &str, delay: bool) -> Result<Option<Option<String>>, ReadFromCacheError> {
        debug!(InnerCache::read, self, category, key);
        Ok(cache::dsl::cache
            .filter(cache::dsl::category.eq(category))
            .filter(cache::dsl::key.eq(key))
            .limit(1)
            .select(CacheEntry::as_select())
            .load(self.connect()?)?
            .first()
            .map(|CacheEntry {value, duration_ms, ..}| {
                if delay {
                    std::thread::sleep(Duration::from_millis(((*duration_ms as f32) * self.rng.random_range(0.75..1.25)) as u64));
                }
                value.to_owned()
            }))
    }

    /// Writes to the database, overwriting the entry the equivalent call to [`Self::read`] would return.
    /// # Errors
    /// If the call to [`Self::connect`] returns an error, that error is returned.
    ///
    /// If the call to [`RunQueryDsl::get_result`] returns an error, that error is returned.
    pub fn write(&mut self, category: &str, key: &str, value: Option<&str>, duration: Duration) -> Result<(), WriteToCacheError> {
        debug!(InnerCache::write, self, category, key, value);
        diesel::insert_into(cache::table)
            .values(NewCacheEntry {category, key, value, duration_ms: duration.as_millis() as i32})
            .execute(self.connect()?)?;
        Ok(())
    }
}

impl From<InnerCache> for (CachePath, OnceCell<SqliteConnection>) {
    fn from(value: InnerCache) -> Self {
        (value.path, value.connection)
    }
}
