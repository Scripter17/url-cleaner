//! Allows making requests, cache reads, etc. effectively single threaded to hide thread count.

use std::time::{Instant, Duration};
use std::cell::Cell;

use serde::{Serialize, Deserialize};
use serde_with::{serde_as, DurationSeconds};
use parking_lot::{ReentrantMutex, ReentrantMutexGuard};

use crate::prelude::*;

/// Allows optionally unthreading and rate-limiting network, cache, etc. operations to avoid leaking the thread count.
///
/// Yes this does work multiple times at once in the same thread. Under the hood it uses a [`ReentrantMutex`].
#[derive(Debug, Default)]
pub struct Unthreader {
    /// The [`UnthreaderMode`] to use.
    pub mode: UnthreaderMode,
    /// The actual unthrading handler.
    inner: UnthreaderInner
}

/// The mode for how an [`Unthreader`] should behave.
///
/// Defaults to [`Self::Multithread`].
#[serde_as]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnthreaderMode {
    /// Don't do any unthreading.
    ///
    /// The default.
    /// # Examples
    /// ```
    /// use std::time::{Instant, Duration};
    /// use std::thread::sleep;
    ///
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let unthreader = Unthreader::from(UnthreaderMode::Multithread);
    ///
    /// let start = Instant::now();
    ///
    /// std::thread::scope(|s| {
    ///     s.spawn(|| {
    ///         let x = unthreader.unthread();
    ///         sleep(Duration::from_secs(1));
    ///     });
    ///    
    ///     s.spawn(|| {
    ///         let x = unthreader.unthread();
    ///         sleep(Duration::from_secs(1));
    ///     });
    /// });
    ///
    /// assert_eq!(start.elapsed().as_secs(), 1);
    /// ```
    #[default]
    Multithread,
    /// Unthread.
    /// # Examples
    /// ```
    /// use std::time::{Instant, Duration};
    /// use std::thread::sleep;
    ///
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let unthreader = Unthreader::from(UnthreaderMode::Unthread);
    ///
    /// let start = Instant::now();
    ///
    /// std::thread::scope(|s| {
    ///     s.spawn(|| {
    ///         let x = unthreader.unthread();
    ///         sleep(Duration::from_secs(1));
    ///     });
    ///    
    ///     s.spawn(|| {
    ///         let x = unthreader.unthread();
    ///         sleep(Duration::from_secs(1));
    ///     });
    /// });
    ///
    /// assert_eq!(start.elapsed().as_secs(), 2);
    /// ```
    Unthread,
    /// [`Self::Unthread`] but if the last unthread started less than [`Self::Ratelimit::0`] ago, waits the remaining duration between starting the new unthread and returning the [`UnthreadHandle`].
    ///
    /// Currently has difficult to predict and probably bad effects in async code due to using [`std::thread::sleep`].
    ///
    /// If you know of an equivalent to [`std::thread::sleep`] that doesn't mess up async please let me know so I can switch to that.
    /// # Examples
    /// ```
    /// use std::time::{Instant, Duration};
    /// use std::thread::sleep;
    ///
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let unthreader = Unthreader::from(UnthreaderMode::Ratelimit(Duration::from_secs(5)));
    ///
    /// let start = Instant::now();
    ///
    /// std::thread::scope(|s| {
    ///     s.spawn(|| {
    ///         let x = unthreader.unthread();
    ///         sleep(Duration::from_secs(1));
    ///     });
    ///    
    ///     s.spawn(|| {
    ///         let x = unthreader.unthread();
    ///         sleep(Duration::from_secs(1));
    ///     });
    /// });
    ///
    /// assert_eq!(start.elapsed().as_secs(), 6);
    /// ```
    Ratelimit(#[serde_as(as = "DurationSeconds<f64>")] Duration)
}

impl From<UnthreaderMode> for Unthreader {
    fn from(mode: UnthreaderMode) -> Self {
        Self {
            mode,
            ..Default::default()
        }
    }
}

/// The actual unthreading handler.
#[derive(Debug, Default)]
pub struct UnthreaderInner(pub ReentrantMutex<Cell<Option<Instant>>>);

/// A handle for an [`UnthreaderInner`]/[`Unthreader`].
///
/// Should be assigned to a variable that goes out of scope when the thing being unthrode is over.
#[allow(dead_code, reason = "Used for its drop glue.")]
#[derive(Debug)]
pub struct UnthreadHandle<'a>(Option<ReentrantMutexGuard<'a, Cell<Option<Instant>>>>);

impl Unthreader {
    /// Gets an [`UnthreadHandle`] that should be kept in scope until the thing you want to unthread is over.
    ///
    /// See each variant of [`UnthreaderMode`] to see what each does.
    #[must_use]
    pub fn unthread(&self) -> UnthreadHandle<'_> {
        debug!(Unthreader::unthread, self);
        UnthreadHandle (
            match self.mode {
                UnthreaderMode::Multithread => None,
                UnthreaderMode::Unthread    => Some(self.inner.0.lock()),
                UnthreaderMode::Ratelimit(d) => {
                    let lock = self.inner.0.lock();
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
