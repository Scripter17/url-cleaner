//! [`ThreadHider`] and [`ThreadLock`].

use parking_lot::{ReentrantMutex, ReentrantMutexGuard};

#[expect(unused_imports, reason = "Used in a doc comment.")]
use crate::prelude::*;

/// A simple way to hide how many threads are being used for a [`Job`] by letting [`Cleaner`]s force long running operations to happen sequentially.
///
/// For most frontends [`Job::thread_hider`] is [`None`] by default but this is used by URL Cleaner Site Userscript to prevent websites from timing how long groups of redirect URLs take to clean to fingerprint users by worker thread count.
///
/// Unless it is actively difficult and obviously useless to do so, when writing a frontend you should let users choose to enable thread hiding at [`Job`] creation.
///
/// # Usage
///
/// Consider the following pseudo[`Cleaner`]:
///
/// ```Json
/// {
///   "action": {"All": [
///     {"Repeat": {
///       "actions": [
///         {"If": {
///           "if": "UrlIsARedirect",
///           "then": {"All": [
///             "AcquireThreadLock",
///             {"Cache": "DoRedirect"}
///           ]}
///         }}
///       ]
///     }},
///     "ReleaseThreadLock"
///   ]}
/// }
/// ```
///
/// When a redirect URL is encountered, [`Action::AcquireThreadLock`] gets a [`ThreadLock`] from [`Job::thread_hider`] and stores it in [`TaskState::thread_lock`] until [`Action::ReleaseThreadLock`] releases it.
///
/// When and only when a different thread attempts to use [`Action::AcquireThreadLock`] with the same [`ThreadHider`] will that thread be forced to wait until the locking task either calls [`Action::ReleaseThreadLock`] or finishes.
///
/// Critically:
///
/// - Using [`Action::AcquireThreadLock`] multiple times in a row is fine because internally a [`parking_lot::ReentrantMutex`] is used.
///
/// - Using [`Action::AcquireThreadLock`] with a [`Job`] whose [`Job::thread_hider`] is [`None`] is effectively a no-op.
///
/// - Using [`Action::ReleaseThreadLock`] with a [`TaskState`] whose [`TaskState::thread_lock`] is [`None`] is also effectively a no-op.
///
/// ## Misusage
///
/// It's important to only use [`Action::ReleaseThreadLock`] once every usage of [`Action::AcquireThreadLock`] is in the past.
///
/// Consider the following pseudo[`Cleaner`]:
///
/// ```Json
/// {
///   "action": {"All": [
///     {"Repeat": {
///       "actions": [
///         {"If": {
///           "if": "UrlIsARedirect",
///           "then": {"All": [
///             "AcquireThreadLock",
///             {"Cache": "DoRedirect"},
///             "ReleaseThreadLock"
///           ]}
///         }}
///       ]
///     }}
///   ]}
/// }
/// ```
///
/// With this, doing two redirect chains at the same time would still have their individual HTTP requests/cache operations be done one at a time but could have them be interwoven, which leaks the worker thread count.
///
/// Additionally, [`Action::AcquireThreadLock`] should be used outside of any caching in order to make hiding cache reads not also leak the thread count.
#[derive(Debug, Default)]
pub struct ThreadHider(pub ReentrantMutex<()>);

/// A lock from a [`ThreadHider`].
#[derive(Debug)]
#[must_use = "Dropping a ThreadLock immediately immediately unlocks the ThreadHider."]
pub struct ThreadLock<'a>(pub ReentrantMutexGuard<'a, ()>);

impl ThreadHider {
    /// Get a [`ThreadLock`].
    pub fn lock<'a>(&'a self) -> ThreadLock<'a> {
        ThreadLock(self.0.lock())
    }
}
