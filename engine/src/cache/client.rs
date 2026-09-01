//! [`CacheClient`].

use std::time::Duration;

use sqlx::{Arguments, Executor, Row};

use crate::prelude::*;

/// A convenient name for [`sqlx::sqlite::SqliteConnectOptions`].
///
/// Lets you not explicitly depend on [`sqlx`].
pub type CacheTarget = sqlx::sqlite::SqliteConnectOptions;

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
#[derive(Debug)]
pub struct CacheClient {
    /// The [`sqlx::SqlitePool`].
    pub pool: sqlx::SqlitePool,
    /// The [`tokio::runtime::Handle`].
    pub handle: tokio::runtime::Handle,
    /// If the SQLite database should be assumed to be initialized.
    ///
    /// Just used to let [`Self::init_acquire`] skip the initialization.
    pub assume_init: std::sync::atomic::AtomicBool,
}

impl CacheClient {
    /// The initializiation code of the database.
    pub const INIT: &str = r#"CREATE TABLE IF NOT EXISTS cache (
        subject  TEXT  NOT NULL,
        "key"    TEXT  NOT NULL,
        value    TEXT          ,
        duration FLOAT NOT NULL,
        UNIQUE(subject, "key") ON CONFLICT REPLACE
    )"#;



    /// [`tokio::runtime::Handle::block_on`] + [`Self::new`].
    /// # Panics
    /// If [`tokio::runtime::Handle::block_on`] panics (usually by being called in an async context or by pointing to a dropped runtime), that panic is not caught.
    pub fn new_sync(options: CacheTarget, handle: tokio::runtime::Handle) -> Self {
        handle.block_on(Self::new(options))
    }

    /// [`tokio::runtime::Handle::block_on`] + [`Self::read`].
    /// # Errors
    /// If [`Self::read`] returns an error, that error is returned.
    /// # Panics
    /// If [`tokio::runtime::Handle::block_on`] panics (usually by being called in an async context or by pointing to a dropped runtime), that panic is not caught.
    pub fn read_sync(&self, subject: &str, key: &str, config: CacheConfig) -> Result<Option<Option<String>>, ReadFromCacheError> {
        self.handle.block_on(self.read(subject, key, config))
    }

    /// [`tokio::runtime::Handle::block_on`] + [`Self::write`].
    /// # Errors
    /// If [`Self::write`] returns an error, that error is retuerned.
    /// # Panics
    /// If [`tokio::runtime::Handle::block_on`] panics (usually by being called in an async context or by pointing to a dropped runtime), that panic is not caught.
    pub fn write_sync(&self, subject: &str, key: &str, value: Option<&str>, duration: Duration, config: CacheConfig) -> Result<(), WriteToCacheError> {
        self.handle.block_on(self.write(subject, key, value, duration, config))
    }

    /// [`tokio::runtime::Handle::block_on`] + [`Self::init_acquire`].
    /// # Errors
    /// If interacing with the database returns an error, returns the error [`sqlx::Error`].
    /// # Panics
    /// If [`tokio::runtime::Handle::block_on`] panics (usually by being called in an async context or by pointing to a dropped runtime), that panic is not caught.
    pub fn init_acquire_sync(&self) -> Result<sqlx::pool::PoolConnection<sqlx::Sqlite>, InitAcquireCacheError> {
        self.handle.block_on(self.init_acquire())
    }



    /// Make a new [`Self`] using the current runtime's [`tokio::runtime::Handle`].
    /// # Panics
    /// If called outside a Tokio runtime, panics.
    pub async fn new(options: CacheTarget) -> Self {
        Self {
            pool       : sqlx::SqlitePool::connect_lazy_with(options),
            handle     : tokio::runtime::Handle::current(),
            assume_init: false.into(),
        }
    }

    /// Read an entry's value.
    /// # Errors
    /// If [`Self::init_acquire`] returns an error, that error is returned.
    ///
    /// If interacing with the database returns an error, returns the error [`sqlx::Error`].
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub async fn read(&self, subject: &str, key: &str, config: CacheConfig) -> Result<Option<Option<String>>, ReadFromCacheError> {
        if !config.read {
            return Ok(None);
        }

        let start = std::time::Instant::now();

        let mut connection = self.init_acquire().await?;

        let mut args = sqlx::sqlite::SqliteArguments::default();

        args.add(subject).expect("???");
        args.add(key    ).expect("???");

        let query = sqlx::query_with(r#"SELECT * FROM cache WHERE subject = $1 AND "key" = $2"#, args);

        Ok(match connection.fetch_optional(query).await? {
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
    /// # Errors
    /// If [`Self::init_acquire`] returns an error, that error is returned.
    ///
    /// If interacing with the database returns an error, returns the error [`sqlx::Error`].
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub async fn write(&self, subject: &str, key: &str, value: Option<&str>, duration: Duration, config: CacheConfig) -> Result<(), WriteToCacheError> {
        if !config.write {
            return Ok(());
        }

        let mut connection = self.init_acquire().await?;

        let mut args = sqlx::sqlite::SqliteArguments::default();

        args.add(subject               ).expect("???");
        args.add(key                   ).expect("???");
        args.add(value                 ).expect("???");
        args.add(duration.as_secs_f64()).expect("???");

        let query = sqlx::query_with(r#"INSERT INTO cache (subject, "key", value, duration) VALUES ($1, $2, $3, $4)"#, args);

        connection.execute(query).await?;

        Ok(())
    }

    /// Ensure the cache is initialized and return a connection.
    /// # Errors
    /// If interacing with the database returns an error, returns the error [`sqlx::Error`].
    pub async fn init_acquire(&self) -> Result<sqlx::pool::PoolConnection<sqlx::Sqlite>, InitAcquireCacheError> {
        let mut connection = self.pool.acquire().await?;

        if !self.assume_init.load(std::sync::atomic::Ordering::Relaxed) {
            let query = sqlx::query(Self::INIT);

            connection.execute(query).await?;

            self.assume_init.store(true, std::sync::atomic::Ordering::Relaxed);
        }

        Ok(connection)
    }
}
