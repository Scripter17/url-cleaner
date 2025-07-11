//! Allows making requests, cache reads, etc. effectively single threaded to hide thread count.

use parking_lot::{ReentrantMutex, ReentrantMutexGuard};

use serde::{Serialize, Deserialize, ser::Serializer, de::{Visitor, Deserializer, Error}};

/// Allows making requests, cache reads, etc. effectively single threaded to hide thread count.
///
/// In URL Cleaner Site Userscript, it's possible for a website to give you 1 redirect URL, then 2 of that same URL, then 3, then 4, and so on and so on until the time your instance takes to clean them suddenly doubles.
///
/// Unthreading means that long running operations can be forced to run sequentially, making N redirect URLs always take N times as long as 1, while keeping the benefits of parallelizing everything else.
///
/// It's not a perfect defense, websites can probably give you extremely expensive but non-redirect URLs and use the previously mentioned schcme to figure out your thread count, but that is very unlikely to give useful results in the vast majority of situations.
#[derive(Debug, Default)]
pub enum Unthreader {
    /// Don't do any unthreading.
    ///
    /// The default variant.
    #[default]
    No,
    /// Do unthreading.
    Yes(ReentrantMutex<()>)
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

    /// If `self` is [`Self::Yes`], return a [`ReentrantMutexGuard`].
    ///
    /// Assign this to variable and drop it when you want to rethread.
    #[must_use]
    pub fn unthread(&self) -> Option<ReentrantMutexGuard<()>> {
        match self {
            Self::No => None,
            Self::Yes(x) => Some(x.lock())
        }
    }
}

/// Serde helper for deserializing [`Unthreader`].
struct UnthreaderVisitor;

impl<'de> Visitor<'de> for UnthreaderVisitor {
    type Value = Unthreader;

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(Unthreader::r#if(v))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Expected a bool")
    }
}

impl<'de> Deserialize<'de> for Unthreader {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(UnthreaderVisitor)
    }
}

impl Serialize for Unthreader {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(match self {
            Self::No => false,
            Self::Yes(_) => true
        })
    }
}
