//! [`SmallJobIntoIterator`].

use crate::prelude::*;

/// A [`SmallJob`] currently being iterated over.
#[derive(Debug, Clone, Copy)]
pub struct SmallJobIntoIterator<'j, 't, I: Iterator<Item = Result<SmallLazyTaskConfig<'t>, GetLazyTaskConfigError>>> {
    /// [`SmallJob::config`].
    pub config: JobConfig<'j>,
    /// [`SmallJob::small_lazy_task_configs`]'s [`IntoIterator::IntoIter`].
    pub small_lazy_task_configs: I
}

impl<'j, 't, I: Iterator<Item = Result<SmallLazyTaskConfig<'t>, GetLazyTaskConfigError>>> Iterator for SmallJobIntoIterator<'j, 't, I> {
    type Item = Result<SmallLazyTask<'j, 't>, MakeLazyTaskError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.small_lazy_task_configs.next()? {
            Ok(config) => Ok(self.config.make_small_lazy_task(config)),
            Err(e) => Err(e.into())
        })
    }
}

