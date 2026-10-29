//! [`CacheClient`].

use std::time::Duration;

use sqlx::Row;

use crate::prelude::*;

/// A connection to a SQLite cache.
///
/// TODO: Fix race condition.
/// # Examples
/// ```
/// use std::time::{Instant, Duration};
/// use url_cleaner_engine::prelude::*;
///
/// let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
///
/// let client = CacheClient::new_sync(":memory:".parse().unwrap(), runtime.handle().clone());
///
/// let mut config = CacheConfig {
///     read : true,
///     write: true,
///     hide : false,
/// };
///
/// let duration = Duration::from_secs_f64(0.5);
///
/// let a = Instant::now();
/// assert_eq!(client.read_sync("", "abc", config).unwrap(), None);
/// assert!(a.elapsed() < duration);
///
/// client.write_sync("", "abc", Some("def"), duration, config).unwrap();
///
/// let a = Instant::now();
/// assert_eq!(client.read_sync("", "abc", config).unwrap(), Some(Some("def".into())));
/// assert!(a.elapsed() < duration);
///
/// config.hide = true;
///
/// let a = Instant::now();
/// assert_eq!(client.read_sync("", "abc", config).unwrap(), Some(Some("def".into())));
/// assert!(a.elapsed() > duration);
///
/// config.read = false;
///
/// let a = Instant::now();
/// assert_eq!(client.read_sync("", "abc", config).unwrap(), None);
/// assert!(a.elapsed() < duration);
/// ```
#[derive(Debug, Clone)]
pub struct CacheClient {
    /// The [`sqlx::SqlitePool`].
    pub pool: sqlx::SqlitePool,
    /// The [`tokio::runtime::Handle`].
    pub handle: tokio::runtime::Handle,
}

impl CacheClient {
    /// The query to initialize the database.
    pub const INIT: &str = r#"CREATE TABLE IF NOT EXISTS cache (
        subject  TEXT  NOT NULL,
        "key"    TEXT  NOT NULL,
        value    TEXT          ,
        duration FLOAT NOT NULL,
        UNIQUE(subject, "key") ON CONFLICT REPLACE
    )"#;

    /// The query to read from the database.
    pub const READ: &str = r#"SELECT value, duration FROM cache WHERE subject = $1 AND "key" = $2"#;

    /// The query to write to the database.
    pub const WRITE: &str = r#"INSERT INTO cache (subject, "key", value, duration) VALUES ($1, $2, $3, $4)"#;



    /// [`tokio::runtime::Handle::block_on`] + [`Self::new`].
    /// # Panics
    /// If [`tokio::runtime::Handle::block_on`] panics, that panic is not caught.
    pub fn new_sync(target: CacheTarget, handle: tokio::runtime::Handle) -> Self {
        handle.block_on(Self::new(target))
    }

    /// [`tokio::runtime::Handle::block_on`] + [`Self::read`].
    /// # Errors
    /// If [`Self::read`] returns an error, that error is returned.
    /// # Panics
    /// If [`tokio::runtime::Handle::block_on`] panics, that panic is not caught.
    pub fn read_sync(&self, subject: &str, key: &str, config: CacheConfig) -> Result<Option<Option<String>>, ReadFromCacheError> {
        self.handle.block_on(self.read(subject, key, config))
    }

    /// [`tokio::runtime::Handle::block_on`] + [`Self::write`].
    /// # Errors
    /// If [`Self::write`] returns an error, that error is retuerned.
    /// # Panics
    /// If [`tokio::runtime::Handle::block_on`] panics, that panic is not caught.
    pub fn write_sync(&self, subject: &str, key: &str, value: Option<&str>, duration: Duration, config: CacheConfig) -> Result<(), WriteToCacheError> {
        self.handle.block_on(self.write(subject, key, value, duration, config))
    }



    /// Make a new [`Self`] using the current runtime's [`tokio::runtime::Handle`].
    /// # Panics
    /// If called outside a Tokio runtime, panics.
    pub async fn new(target: CacheTarget) -> Self {
        Self {
            pool: sqlx::sqlite::SqlitePoolOptions::default().after_connect(|conn, _| Box::pin(async move {
                sqlx::query(Self::INIT).execute(conn).await?;

                Ok(())
            })).connect_lazy_with(target.options),
            handle: tokio::runtime::Handle::current(),
        }
    }

    /// Read an entry's value.
    ///
    /// If [`CacheConfig::read`] is [`false`], always immediately returns `Ok(None)`.
    ///
    /// If [`CacheConfig::hide`] is [`true`], a matching cache entry is found, and reading the cache took less time than that entry's `duration` column, sleeps for the remaining time.
    /// # Errors
    /// If interacing with the database returns an error, returns the error [`sqlx::Error`].
    pub async fn read(&self, subject: &str, key: &str, config: CacheConfig) -> Result<Option<Option<String>>, ReadFromCacheError> {
        if !config.read {
            return Ok(None);
        }

        let start = std::time::Instant::now();

        Ok(match sqlx::query(Self::READ).bind(subject).bind(key).fetch_optional(&self.pool).await? {
            Some(row) => {
                if config.hide && let Some(remainder) = Duration::from_secs_f64(row.try_get("duration")?).checked_sub(start.elapsed()) {
                    tokio::time::sleep(remainder).await;
                }

                Some(row.try_get("value")?)
            },
            None => None
        })
    }

    /// Write an entry.
    ///
    /// If [`CacheConfig::read`] is [`false`], always immediately returns `Ok(())`.
    /// # Errors
    /// If interacing with the database returns an error, returns the error [`sqlx::Error`].
    pub async fn write(&self, subject: &str, key: &str, value: Option<&str>, duration: Duration, config: CacheConfig) -> Result<(), WriteToCacheError> {
        if !config.write {
            return Ok(());
        }

        sqlx::query(Self::WRITE)
            .bind(subject).bind(key).bind(value).bind(duration.as_secs_f64())
            .execute(&self.pool).await?;

        Ok(())
    }
}
