//! [`SmallJob`].

use crate::prelude::*;

/// [`Job`] but small.
#[derive(Debug, Clone, Copy)]
pub struct SmallJob<'j, 't, I: IntoIterator<Item = Result<SmallLazyTaskConfig<'t>, GetLazyTaskConfigError>>> {
    /// The [`JobConfig`] whose [`JobConfig::make_lazy_task`] each [`LazyTaskConfig`] from [`Self::small_lazy_task_configs`] is given to.
    pub config: JobConfig<'j>,
    /// The source of [`SmallLazyTaskConfig`]s to turn into [`SmallLazyTask`]s via [`Self::config`]'s [`JobConfig::make_small_lazy_task`].
    pub small_lazy_task_configs: I
}

impl<'j, 't, I: IntoIterator<Item = Result<SmallLazyTaskConfig<'t>, GetLazyTaskConfigError>>> IntoIterator for SmallJob<'j, 't, I> {
    type IntoIter = SmallJobIntoIterator<'j, 't, I::IntoIter>;
    type Item = Result<SmallLazyTask<'j, 't>, MakeLazyTaskError>;

    fn into_iter(self) -> Self::IntoIter {
        SmallJobIntoIterator {
            config: self.config,
            small_lazy_task_configs: self.small_lazy_task_configs.into_iter()
        }
    }
}
