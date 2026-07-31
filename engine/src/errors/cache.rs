//! [`CacheClient`].

use crate::prelude::*;

/// [`CacheClient::read`].
#[derive(Debug, Error)]
pub enum ReadFromCacheError {
    /** [`sqlx::Error`].           **/ #[error(transparent)] SqlxError            (#[from] sqlx::Error          ),
    /** [`InitAcquireCacheError`]. **/ #[error(transparent)] InitAcquireCacheError(#[from] InitAcquireCacheError),
}

/// [`CacheClient::write`].
#[derive(Debug, Error)]
pub enum WriteToCacheError {
    /** [`sqlx::Error`].           **/ #[error(transparent)] SqlxError            (#[from] sqlx::Error          ),
    /** [`InitAcquireCacheError`]. **/ #[error(transparent)] InitAcquireCacheError(#[from] InitAcquireCacheError),
}

/// [`CacheClient::init_acquire`].
#[derive(Debug, Error)]
pub enum InitAcquireCacheError {
    /** [`sqlx::Error`]. **/ #[error(transparent)] SqlxError(#[from] sqlx::Error),
}
