//! [`SmallLazyTask`].

use crate::prelude::*;

/// Cheap intermediate step between [`SmallLazyTaskConfig`] and [`Task`] to allow using worker threads.
///
/// Mainly used to make [`Task`]s via [`Self::make`].
#[derive(Debug, Clone)]
pub struct SmallLazyTask<'j, 't> {
    /// The [`SmallLazyTaskConfig`].
    ///
    /// Uses the lifetime `'t` to allow re-using buffers it borrows from between [`Self::make`] and [`Task::do`].
    pub config: SmallLazyTaskConfig<'t>,
    /// The [`JobContext`].
    pub job_context: &'j JobContext,
    /// The [`Cleaner`].
    pub cleaner: &'j Cleaner<'j>,
    /// The [`Unthreader`].
    pub unthreader: &'j Unthreader,
    /// The [`Cache`].
    #[cfg(feature = "cache")]
    pub cache: Cache<'j>,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    pub http_client: &'j HttpClient
}

impl<'j> TryFrom<SmallLazyTask<'j, '_>> for Task<'j> {
    type Error = MakeTaskError;

    /// [`SmallLazyTask::make`].
    /// # Errors
    #[doc = edoc!(callerr(SmallLazyTask::make))]
    fn try_from(value: SmallLazyTask<'j, '_>) -> Result<Self, Self::Error> {
        value.make()
    }
}

impl<'j> SmallLazyTask<'j, '_> {
    /// Makes the [`Task`].
    ///
    /// Notably drops the `'t` lifetime to allow re-using buffers [`Self::config`] borrows from between [`Self::make`] and [`Task::do`].
    /// # Errors
    #[doc = edoc!(callerr(SmallLazyTaskConfig::make))]
    pub fn make(self) -> Result<Task<'j>, MakeTaskError> {
        Ok(Task {
            config     : self.config.make()?,
            job_context: self.job_context,
            cleaner    : self.cleaner,
            unthreader : self.unthreader,
            #[cfg(feature = "cache")]
            cache      : self.cache,
            #[cfg(feature = "http")]
            http_client: self.http_client
        })
    }
}

