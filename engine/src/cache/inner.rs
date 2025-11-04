//! [`InnerCache`].

use std::time::Duration;
use std::sync::OnceLock;
use std::path::PathBuf;

use rusqlite::{Connection, OptionalExtension};
use parking_lot::{ReentrantMutex, ReentrantMutexGuard, MappedReentrantMutexGuard};

use crate::prelude::*;

/// A [`Cache`] without a [`CacheConfig`].
#[derive(Debug, Default)]
pub struct InnerCache {
    /// The [`CacheLocation`].
    location: CacheLocation,
    /// The [`Sync`]'d and lazied [`Connection`].
    connection: ReentrantMutex<OnceLock<Connection>>
}

impl InnerCache {
    /// The command to initialize the cache.
    pub const INIT: &str = r#"CREATE TABLE IF NOT EXISTS cache (
        subject TEXT NOT NULL,
        "key" TEXT NOT NULL,
        value TEXT,
        duration FLOAT NOT NULL,
        UNIQUE(subject, "key") ON CONFLICT REPLACE
    )"#;

    /// The template to read from the cache.
    ///
    /// - `?1` is [`CacheEntryKeys::subject`].
    /// - `?2` is [`CacheEntryKeys::key`].
    pub const READ: &str = r#"SELECT * FROM cache WHERE subject = ?1 AND "key" = ?2"#;

    /// The template to write to the cache.
    ///
    /// - `?1` is [`NewCacheEntry::subject`].
    /// - `?2` is [`NewCacheEntry::key`].
    /// - `?3` is [`NewCacheEntry::value`].
    /// - `?4` is [`NewCacheEntry::duration`] as an [`f32`] of seconds.
    pub const WRITE: &str = r#"INSERT INTO cache (subject, "key", value, duration) VALUES (?1, ?2, ?3, ?4)"#;

    /// Make a new [`Self`] with the provided [`CacheLocation`].
    pub fn new(location: CacheLocation) -> Self {
        Self {
            location,
            ..Default::default()
        }
    }

    /// Get the [`CacheLocation`].
    pub fn location(&self) -> &CacheLocation {
        &self.location
    }

    /// Get a lock on the cache.
    /// # Errors
    #[doc = edoc!(callerr(Connection::open_in_memory), callerr(std::fs::exists), callerr(std::fs::File::create_new), callerr(Connection::open), callerr(Connection::execute))]
    pub fn lock(&self) -> Result<MappedReentrantMutexGuard<'_, Connection>, LockCacheError> {
        Ok(match ReentrantMutexGuard::try_map(self.connection.lock(), OnceLock::get) {
            Ok (lock) => lock,
            Err(lock) => {
                let connection = match &self.location {
                    CacheLocation::Memory => Connection::open_in_memory()?,
                    CacheLocation::Path(path) => {
                        if !std::fs::exists(path)? {
                            std::fs::File::create_new(path)?;
                        }
                        Connection::open(path)?
                    }
                };
                connection.execute(Self::INIT, [])?;
                ReentrantMutexGuard::map(lock, |ol| ol.get_or_init(move || connection))
            }
        })
    }

    /// Read an entry from the cache.
    /// # Errors
    #[doc = edoc!(callerr(Self::lock), callerr(Connection::query_one))]
    pub fn read(&self, keys: CacheEntryKeys<'_>) -> Result<Option<CacheEntryValues>, ReadFromCacheError> {
        Ok(self.lock()?.query_one(
            Self::READ,
            [keys.subject, keys.key],
            |row| Ok(CacheEntryValues {
                value   : row.get("value")?,
                duration: Duration::from_secs_f32(row.get("duration")?)
            })
        ).optional()?)
    }

    /// Write an entry to the cache.
    /// # Errors
    #[doc = edoc!(callerr(Self::lock), callerr(Connection::execute))]
    pub fn write(&self, entry: NewCacheEntry<'_>) -> Result<(), WriteToCacheError> {
        self.lock()?.execute(
            Self::WRITE,
            (entry.subject, entry.key, entry.value, entry.duration.as_secs_f32())
        )?;

        Ok(())
    }
}

impl From<PathBuf> for InnerCache {
    fn from(path: PathBuf) -> Self {
        CacheLocation::from(path).into()
    }
}

impl From<CacheLocation> for InnerCache {
    fn from(location: CacheLocation) -> Self {
        Self {
            location,
            ..Default::default()
        }
    }
}
