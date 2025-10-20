//! [`JobIntoIterator`].

use crate::prelude::*;

/// A [`Job`] currently being iterated over.
#[derive(Debug, Clone, Copy)]
pub struct JobIntoIterator<'j, 't, I: Iterator<Item = Result<LazyTaskConfig<'t>, GetLazyTaskConfigError>>> {
    /// [`Job::config`].
    pub config: JobConfig<'j>,
    /// [`Job::lazy_task_configs`]'s [`IntoIterator::IntoIter`].
    pub lazy_task_configs: I
}

impl<'j, 't, I: Iterator<Item = Result<LazyTaskConfig<'t>, GetLazyTaskConfigError>>> Iterator for JobIntoIterator<'j, 't, I> {
    type Item = Result<LazyTask<'j, 't>, MakeLazyTaskError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.lazy_task_configs.next()? {
            Ok(config) => Ok(self.config.make_lazy_task(config)),
            Err(e) => Err(e.into())
        })
    }
}
