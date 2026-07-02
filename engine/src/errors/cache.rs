//! Cache.

use crate::prelude::*;

/// [`Cache::read`]/[`InnerCache::read`].
#[derive(Debug, Error)]
pub enum ReadFromCacheError {
    /** [`rusqlite::Error`]. **/ #[error(transparent)] RusqliteError (#[from] Box<rusqlite::Error>),
    /** [`LockCacheError`].  **/ #[error(transparent)] LockCacheError(#[from] LockCacheError      ),
}

/// [`Cache::write`]/[`InnerCache::write`].
#[derive(Debug, Error)]
pub enum WriteToCacheError {
    /** [`rusqlite::Error`]. **/ #[error(transparent)] RusqliteError (#[from] Box<rusqlite::Error>),
    /** [`LockCacheError`].  **/ #[error(transparent)] LockCacheError(#[from] LockCacheError      ),
}

/// [`InnerCache::lock`].
#[derive(Debug, Error)]
pub enum LockCacheError {
    /** [`rusqlite::Error`]. **/ #[error(transparent)] RusqliteError (#[from] Box<rusqlite::Error>),
    /** [`io::Error`].       **/ #[error(transparent)] IoError       (#[from] io::Error           ),
}

impl From<rusqlite::Error> for ReadFromCacheError {fn from(value: rusqlite::Error) -> Self {Box::new(value).into()}}
impl From<rusqlite::Error> for WriteToCacheError  {fn from(value: rusqlite::Error) -> Self {Box::new(value).into()}}
impl From<rusqlite::Error> for LockCacheError     {fn from(value: rusqlite::Error) -> Self {Box::new(value).into()}}
