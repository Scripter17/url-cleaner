//! [`CacheHandle`].

use rand::TryRngCore;

use crate::prelude::*;

/// A wrapper around a [`Cache`] for optional security features provided by [`CacheHandleConfig`].
///
/// Unlike [`Cache`], which is intended to be shared between [`Job`]s, [`CacheHandle`]s are intended to be made on a per-[`Job`] basis using the [`CacheHandleConfig`] appropriate for each particular [`Job`].
///
/// For example, a CLI program writing results to a file doesn't need to enable cache delay/unthreading, but a userscript should.
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
/// use std::time::Duration;
///
/// let cache = CacheHandle {
///     cache: &Default::default(),
///     config: Default::default()
/// };
///
/// assert_eq!(cache.read(CacheEntryKeys { subject: "subject", key: "key" }).unwrap().map(|entry| entry.value), None);
/// cache.write(NewCacheEntry { subject: "subject", key: "key", value: None, duration: Default::default() }).unwrap();
/// assert_eq!(cache.read(CacheEntryKeys { subject: "subject", key: "key" }).unwrap().map(|entry| entry.value), Some(None));
/// cache.write(NewCacheEntry { subject: "subject", key: "key", value: Some("value"), duration: Default::default() }).unwrap();
/// assert_eq!(cache.read(CacheEntryKeys { subject: "subject", key: "key" }).unwrap().map(|entry| entry.value), Some(Some("value".into())));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CacheHandle<'a> {
    /// The [`Cache`].
    pub cache: &'a Cache,
    /// The [`CacheHandleConfig`].
    pub config: CacheHandleConfig
}

impl CacheHandle<'_> {
    /// Reads from the cache.
    /// # Errors
    /// If the call to [`InnerCache::read`] returns an error, that error is returned.
    /// # Panics
    /// If, somehow, [`rand::rngs::OsRng`] doesn't work, this panics when [`Self::config`]'s [`CacheHandleConfig::delay`] is [`true`].
    pub fn read(&self, keys: CacheEntryKeys) -> Result<Option<CacheEntryValues>, ReadFromCacheError> {
        if self.config.read {
            let ret = self.cache.read(keys)?;
            if self.config.delay && let Some(CacheEntryValues {duration, ..}) = ret {
                let between_neg_1_and_1 = rand::rngs::OsRng.try_next_u32().expect("Os RNG to be available") as f32 / f32::MAX * 2.0 - 1.0;
                std::thread::sleep(duration.mul_f32(1.0 + between_neg_1_and_1 / 8.0));
            }
            Ok(ret)
        } else {
            Ok(None)
        }
    }

    /// Writes to the cache.
    ///
    /// If an entry for the `subject` and `key` already exists, overwrites it.
    /// # Errors
    /// If the call to [`InnerCache::write`] returns an error, that error is returned.
    pub fn write(&self, entry: NewCacheEntry) -> Result<(), WriteToCacheError> {
        if self.config.write {
            self.cache.write(entry)
        } else {
            Ok(())
        }
    }
}
