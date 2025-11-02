//! [`Unthreader`] and co.

use std::time::Instant;
use std::cell::Cell;

use parking_lot::ReentrantMutex;

use crate::prelude::*;

pub mod mode;
pub mod handle;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::mode::*;
    pub use super::handle::*;

    pub use super::Unthreader;
}

/// Allows optionally hiding thread count by making certain long operations happen sequentially.
///
/// Without unthreading, a website URL Cleaner Site Userscript is being run on can figure out the worker thread count of its URL Cleaner Site instacne
/// by giving it one redirect URL, then two, then three, and so on until the time the job takes doubles at the worker count plus one, where one worker has to do two redirects.
///
/// Because the default worker thread count computed using (and currently literally is just) the CPU's thread count,
/// that'd let websites fingerprint you based on the thread count of the computer your URL Cleaner Site instance is running on.
///
/// [`UnthreaderMode::Unthread`] makes long operations like HTTP requests and cache reads happen one after another, avoiding most risk of fingerprinting on thread counts.
///
/// By default CLI, Discord App, and even Site disable unthreading (using [`UnthreaderMode::Multithread`]),
/// but Site Userscript tells Site to use unthreading to avoid this particular method for fingerprinting.
#[derive(Debug, Default)]
pub struct Unthreader {
    /// The [`UnthreaderMode`] to use.
    pub mode: UnthreaderMode,
    /// The actual unthrading handler.
    inner: ReentrantMutex<Cell<Option<Instant>>>
}

impl From<UnthreaderMode> for Unthreader {
    fn from(mode: UnthreaderMode) -> Self {
        Self {
            mode,
            ..Default::default()
        }
    }
}

impl Unthreader {
    /// Gets an [`UnthreaderHandle`] that should be kept in scope until the thing you want to unthread is over.
    ///
    /// See each variant of [`UnthreaderMode`] to see what each does.
    #[must_use]
    pub fn unthread(&self) -> UnthreaderHandle<'_> {
        UnthreaderHandle (
            match self.mode {
                UnthreaderMode::Multithread => None,
                UnthreaderMode::Unthread    => Some(self.inner.lock()),
                UnthreaderMode::Ratelimit(d) => {
                    let lock = self.inner.lock();
                    if let Some(last) = lock.get() && let Some(sleep) = d.checked_sub(last.elapsed()) {
                        std::thread::sleep(sleep);
                    }
                    lock.set(Some(Instant::now()));
                    Some(lock)
                }
            }
        )
    }
}
