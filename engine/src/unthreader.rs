//! [`Unthreader`].

use parking_lot::{ReentrantMutex, ReentrantMutexGuard};

/// Allows optionally forcing long running operations in multiple threads to happen one after another to hide how many threads you have.
///
/// Defaults to [`Self::Off`].
#[derive(Debug, Default)]
pub enum Unthreader {
    /// Don't do any unthreading.
    ///
    /// The default.
    #[default]
    Off,
    /// Do unthreading.
    On(ReentrantMutex<()>)
}

impl Unthreader {
    /// [`Self::Off`].
    pub fn off() -> Self {
        Self::Off
    }

    /// [`Self::On`].
    pub fn on() -> Self {
        Self::On(Default::default())
    }

    /// If [`true`], [`Self::On`]. If [`false`], [`Self::Off`].
    pub fn r#if(x: bool) -> Self {
        match x {
            false => Self::off(),
            true => Self::on()
        }
    }

    /// If [`Self::On`], do unthreading.
    ///
    /// Generally you should assign this to a variable that is dropped once the unthreaded operation is over.
    #[must_use]
    pub fn unthread(&self) -> Option<ReentrantMutexGuard<'_, ()>> {
        match self {
            Self::Off => None,
            Self::On(x) => Some(x.lock())
        }
    }

    /// If `x` is [`true`], return `self`. If `x` is false, return [`Self::Off`].
    pub fn filter(&self, x: bool) -> &Self {
        match x {
            true => self,
            false => &Self::Off
        }
    }
}
