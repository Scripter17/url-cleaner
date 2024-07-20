use std::sync::Mutex;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

mod schema;
pub use schema::cache;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = cache)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CacheEntry {
    pub id: i32,
    pub category: String,
    pub key: String,
    pub value: Option<String>
}

#[derive(Debug, PartialEq, Eq, Serialize, Insertable)]
#[diesel(table_name = cache)]
pub struct NewCacheEntry<'a> {
    pub category: &'a str,
    pub key: &'a str,
    pub value: Option<&'a str>
}

pub struct CacheHandler(pub Mutex<SqliteConnection>);

impl ::core::fmt::Debug for CacheHandler {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "CacheHandler")
    }
}

impl From<SqliteConnection> for CacheHandler {
    fn from(value: SqliteConnection) -> Self {
        Self(Mutex::new(value))
    }
}

#[derive(Debug, Error)]
pub enum ReadCacheError {
    #[error("{0}")]
    MutexPoisonError(String),
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error)
}

#[derive(Debug, Error)]
pub enum WriteCacheError {
    #[error("{0}")]
    MutexPoisonError(String),
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error)
}

impl CacheHandler {
    pub fn read_cache(&self, category: &str, key: &str) -> Result<Option<Option<String>>, ReadCacheError> {
        Ok(cache::dsl::cache
            .filter(cache::dsl::category.eq(category))
            .filter(cache::dsl::key.eq(key))
            .limit(1)
            .select(CacheEntry::as_select())
            .load(&mut *self.0.lock().map_err(|e| ReadCacheError::MutexPoisonError(e.to_string()))?)?
            .first()
            .map(|cache_entry| cache_entry.value.to_owned()))
    }

    pub fn write_cache(&self, category: &str, key: &str, value: Option<&str>) -> Result<(), WriteCacheError> {
        diesel::insert_into(cache::table)
            .values(&NewCacheEntry {category, key, value})
            .returning(CacheEntry::as_returning())
            .get_result(&mut *self.0.lock().map_err(|e| WriteCacheError::MutexPoisonError(e.to_string()))?)?;
        Ok(())
    }
}
