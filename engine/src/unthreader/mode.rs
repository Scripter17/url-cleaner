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
    ///         sleep(Duration::from_millis(500));
    ///     });
    ///
    ///     s.spawn(|| {
    ///         let x = unthreader.unthread();
    ///         sleep(Duration::from_millis(500));
    ///     });
    /// });
    ///
    /// let elapsed = start.elapsed();
    ///
    /// assert!(500 <= elapsed.as_millis() && elapsed.as_millis() < 750, "{elapsed:?}");
    /// ```
    ///
    /// ```
    /// use std::time::{Instant, Duration};
    /// use std::thread::sleep;
    ///
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let unthreader = Unthreader::from(UnthreaderMode::Multithread);
    /// let start = Instant::now();
    ///
    /// let x = unthreader.unthread();
    /// sleep(Duration::from_millis(500));
    ///
    /// let x = unthreader.unthread();
    /// sleep(Duration::from_millis(500));
    ///
    /// let elapsed = start.elapsed();
    ///
    /// assert!(1000 <= elapsed.as_millis() && elapsed.as_millis() < 1250, "{elapsed:?}");
    /// ```
    #[default]
    Multithread,
    /// Unthread, making unthreading operations happen one by one even in multithreaded setups.
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
    ///         sleep(Duration::from_millis(500));
    ///     });
    ///
    ///     s.spawn(|| {
    ///         let x = unthreader.unthread();
    ///         sleep(Duration::from_millis(500));
    ///     });
    /// });
    ///
    /// let elapsed = start.elapsed();
    ///
    /// assert!(1000 <= elapsed.as_millis() && elapsed.as_millis() < 1250, "{elapsed:?}");
    /// ```
    ///
    /// ```
    /// use std::time::{Instant, Duration};
    /// use std::thread::sleep;
    ///
    /// use url_cleaner_engine::prelude::*;
    ///
    /// // Make sure deadlocks don't happen.
    ///
    /// let unthreader = Unthreader::from(UnthreaderMode::Unthread);
    /// let start = Instant::now();
    ///
    /// let x = unthreader.unthread();
    /// sleep(Duration::from_millis(500));
    ///
    /// let y = unthreader.unthread();
    /// sleep(Duration::from_millis(500));
    ///
    /// let elapsed = start.elapsed();
    ///
    /// assert!(1000 <= elapsed.as_millis() && elapsed.as_millis() < 1250, "{elapsed:?}");
    /// ```
    Unthread,
    /// [`Self::Unthread`] but wait until [`Self::Ratelimit::0`] after the last unthreading to return [`UnthreaderHandle`].
    ///
    /// For example, if the ratelimit is 5 seconds and the last HTTP request was 2 seconds ago, this waits 3 seconds.
    ///
    /// This shouldn't be in unthreading but I can't think of a signficiantly better place to put it.
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
    ///     UnthreaderMode::Ratelimit(Duration::from_secs(3)),
    ///     serde_json::from_str(r#"{"Ratelimit": 3}"#).unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     UnthreaderMode::Ratelimit(Duration::from_secs(3)),
    ///     serde_json::from_str(r#"{"Ratelimit": 3.0}"#).unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     UnthreaderMode::Ratelimit(Duration::from_millis(3500)),
    ///     serde_json::from_str(r#"{"Ratelimit": 3.5}"#).unwrap()
    /// );
    /// ```
    ///
    /// ```
    /// use std::time::{Instant, Duration};
    /// use std::thread::sleep;
    ///
    /// use url_cleaner_engine::prelude::*;
    /// 
    /// // Make sure ratelimiting works.
    ///
    /// let unthreader = Unthreader::from(UnthreaderMode::Ratelimit(Duration::from_secs(1)));
    /// let start = Instant::now();
    ///
    /// std::thread::scope(|s| {
    ///     s.spawn(|| {
    ///         let x = unthreader.unthread();
    ///         sleep(Duration::from_millis(500));
    ///     });
    ///
    ///     s.spawn(|| {
    ///         let x = unthreader.unthread();
    ///         sleep(Duration::from_millis(500));
    ///     });
    /// });
    ///
    /// let elapsed = start.elapsed();
    ///
    /// assert!(1500 <= elapsed.as_millis() && elapsed.as_millis() < 1750, "{elapsed:?}");
    /// ```
    /// 
    /// ```
    /// use std::time::{Instant, Duration};
    /// use std::thread::sleep;
    ///
    /// use url_cleaner_engine::prelude::*;
    /// 
    /// // Make sure deadlocks don't happen.
    ///
    /// let unthreader = Unthreader::from(UnthreaderMode::Ratelimit(Duration::from_secs(1)));
    /// let start = Instant::now();
    ///
    /// let x = unthreader.unthread();
    /// sleep(Duration::from_millis(500));
    ///
    /// let y = unthreader.unthread();
    /// sleep(Duration::from_millis(500));
    ///
    /// let elapsed = start.elapsed();
    ///
    /// assert!(1500 <= elapsed.as_millis() && elapsed.as_millis() < 1750, "{elapsed:?}");
    /// ```
    Ratelimit(#[serde_as(as = "DurationSecondsWithFrac<f64>")] Duration)
}
