//! [`Unthreader`] and co.

use std::time::Instant;

use crate::prelude::*;

pub mod mode;
pub mod inner;
pub mod handle;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::mode::*;
    pub use super::inner::*;
    pub use super::handle::*;

    pub use super::Unthreader;
}

/// Allows optionally unthreading and rate-limiting network, cache, etc. operations to avoid leaking the thread count to things that can see how long a [`Job`] took.
#[derive(Debug, Default)]
pub struct Unthreader {
    /// The [`UnthreaderMode`] to use.
    pub mode: UnthreaderMode,
    /// The actual unthrading handler.
    inner: UnthreaderInner
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
        debug!(Unthreader::unthread, self);
        UnthreaderHandle (
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
