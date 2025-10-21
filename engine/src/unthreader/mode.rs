//! [`UnthreaderMode`].

use std::time::Duration;

use serde::{Serialize, Deserialize};
use serde_with::{serde_as, DurationSecondsWithFrac};

#[expect(unused_imports, reason = "Used in a doc comment.")]
use crate::prelude::*;

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
    /// // Make sure unthreading works.
    ///
    /// let unthreader = Unthreader::from(UnthreaderMode::Unthread);
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
    ///
    /// // Make sure deadlocks don't happen.
    ///
    /// let unthreader = Unthreader::from(UnthreaderMode::Unthread);
    /// let start = Instant::now();
    ///
    /// let x = unthreader.unthread();
    /// sleep(Duration::from_secs(1));
    ///
    /// let y = unthreader.unthread();
    /// sleep(Duration::from_secs(1));
    ///
    /// assert_eq!(start.elapsed().as_secs(), 2);
    /// ```
    Unthread,
    /// [`Self::Unthread`] but if the last unthread started less than [`Self::Ratelimit::0`] ago, waits the remaining duration between starting the new unthread and returning the [`UnthreaderHandle`].
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
    /// // Make sure deserializing from a number works.
    ///
    /// assert_eq!(
    ///     UnthreaderMode::Ratelimit(Duration::from_secs(5)),
    ///     serde_json::from_str(r#"{"Ratelimit": 5}"#).unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     UnthreaderMode::Ratelimit(Duration::from_secs(5)),
    ///     serde_json::from_str(r#"{"Ratelimit": 5.0}"#).unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     UnthreaderMode::Ratelimit(Duration::from_millis(5500)),
    ///     serde_json::from_str(r#"{"Ratelimit": 5.5}"#).unwrap()
    /// );
    ///
    /// // Make sure ratelimiting works.
    ///
    /// let unthreader = Unthreader::from(UnthreaderMode::Ratelimit(Duration::from_secs(5)));
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
    ///
    /// // Make sure deadlocks don't happen.
    ///
    /// let unthreader = Unthreader::from(UnthreaderMode::Ratelimit(Duration::from_secs(5)));
    /// let start = Instant::now();
    ///
    /// let x = unthreader.unthread();
    /// sleep(Duration::from_secs(1));
    ///
    /// let y = unthreader.unthread();
    /// sleep(Duration::from_secs(1));
    ///
    /// assert_eq!(start.elapsed().as_secs(), 6);
    /// ```
    Ratelimit(#[serde_as(as = "DurationSecondsWithFrac<f64>")] Duration)
}
