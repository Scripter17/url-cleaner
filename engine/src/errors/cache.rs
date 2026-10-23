//! [`CacheClient`].

use crate::prelude::*;

/// [`CacheClient::read`].
#[derive(Debug, Error)]
pub enum ReadFromCacheError {
    /** [`sqlx::Error`]. **/ #[error(transparent)] SqlxError(#[from] Box<sqlx::Error>),
}

/// [`CacheClient::write`].
#[derive(Debug, Error)]
pub enum WriteToCacheError {
    /** [`sqlx::Error`]. **/ #[error(transparent)] SqlxError(#[from] Box<sqlx::Error>),
}

impl From<sqlx::Error> for ReadFromCacheError {fn from(value: sqlx::Error) -> Self {Box::new(value).into()}}
impl From<sqlx::Error> for WriteToCacheError  {fn from(value: sqlx::Error) -> Self {Box::new(value).into()}}
