//! # [`Unthreader`]
//!
//! When a URL Cleaner Engine frontend is multithreaded, not only are computations done in parallel, but also network requests and cache reads.
//!
//! For example, a job of two `bit.ly` URLs will run twice as fast if done in 2 threads.
//!
//! While this is usually not an issue, if a hostile party like a website can tell how long a job took, such as when using URL Cleaner Site Userscript, that party can do tests and determine how many threads your URL Cleaner Site instance is using.
//!
//! To defend against this without completely throwing out the performance benefits of parallelism, [`Unthreader`]s can be used.
//!
//! An unthreader is simply either nothing ([`Unthreader::No`]), or a mutex ([`Unthreader::Yes`]). When a network request, cache read, etc. is about to be done, components will call [`Unthreader::unthread`].
//!
//! If the unthreader is the `No` variant, this simply returns [`None`] and has no effect.
//!
//! If the unthreader is the `Yes` variant, this returns a lock on the contained mutex. You can then bind this to a variable and drop it once the unthreaded part is over.
//!
//! Notably, each [`Task`] made by a [`Job`] shares the same unthreader from [`Job::unthreader`].
//! Therefore if two tasks are being done in parallel and both call [`Unthreader::unthread`] at the same time, one task will be forced to wait until the other releases the lock.
//!
//! ## Usage
//!
//! For the current three frontends, unthreading defaults to [`Unthreader::No`]. However, URL Cleaner Site Userscript, the main reason for unthreading, enables it in each job it sends to URL Cleaner Site.
//!
//! In general, the option to enable unthreading is named something like "hide thread count", because having both `--threads 4` and `--unthread` would look weird.
//!
//! ## Technical details
//!
//! [`Unthreader::Yes`] uses a [`parking_lot::ReentrantMutex`] to allow for unthreading components to use other unthreading components without deadlocking.
//!
//! However, in theory, this might make certain forms of parallelism not unthread at all. I have only just realized this while writing this, so I'm not entirely sure if and when that happens.
//!
//! If you're making a parallel frontend using async or some other non-thread parallelism, please make sure unthreading works properly.

pub(crate) use super::*;
