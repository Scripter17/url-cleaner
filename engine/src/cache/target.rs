//! [`CacheTarget`]

use crate::prelude::*;

/// A simple wrapper around [`sqlx::sqlite::SqliteConnectOptions`] whose [`FromStr`] includes some better defaults.
///
/// Specifically:
///
/// - [`sqlx::sqlite::SqliteConnectOptions::create_if_missing`] set to [`true`].
///
/// - [`sqlx::sqlite::SqliteConnectOptions::optimize_on_close`] set to [`true`] and `Some(400)`.
#[derive(Debug, Clone)]
pub struct CacheTarget {
    /// The [`sqlx::sqlite::SqliteConnectOptions`].
    pub options: sqlx::sqlite::SqliteConnectOptions
}

impl From<sqlx::sqlite::SqliteConnectOptions> for CacheTarget {
    fn from(value: sqlx::sqlite::SqliteConnectOptions) -> Self {
        Self {
            options: value
        }
    }
}

impl From<CacheTarget> for sqlx::sqlite::SqliteConnectOptions {
    fn from(value: CacheTarget) -> Self {
        value.options
    }
}

impl FromStr for CacheTarget {
    type Err = <sqlx::sqlite::SqliteConnectOptions as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            options: s.parse::<sqlx::sqlite::SqliteConnectOptions>()?
                .create_if_missing(true)
                .optimize_on_close(true, Some(400))
        })
    }
}

impl TryFrom<&str> for CacheTarget {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
