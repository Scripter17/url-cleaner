//! Allows making requests, cache reads, etc. effectively single threaded to hide thread count.

use std::sync::{Mutex, MutexGuard, LockResult};

/// Allows making requests, cache reads, etc. effectively single threaded to hide thread count.
#[derive(Debug, Default)]
pub enum Unthreader {
    /// Don't do any unthreading.
    #[default]
    No,
    /// Do unthreading.
    Yes(Mutex<()>)
}

impl Unthreader {
    /// [`Self::No`].
    pub fn no() -> Self {
        Self::No
    }

    /// [`Self::yes`].
    pub fn yes() -> Self {
        Self::Yes(Default::default())
    }

    /// If `x` is [`true`], [`Self::Yes`], otherwise [`Self::No`].
    pub fn r#if(x: bool) -> Self {
        match x {
            false => Self::no(),
            true  => Self::yes()
        }
    }

    /// If `self` is [`Self::Yes`], return a [`MutexGuard`].
    ///
    /// Assign this to variable and drop it when you want to rethread.
    #[must_use]
    pub fn unthread(&self) -> Option<LockResult<MutexGuard<()>>> {
        match self {
            Self::No => None,
            Self::Yes(x) => Some(x.lock())
        }
    }
}
