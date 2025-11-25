//! [`JobIntoIterator`].

use crate::prelude::*;

/// A [`Job`] currently being iterated over.
#[derive(Debug, Clone, Copy)]
pub struct JobIntoIterator<'j, 't, I: Iterator<Item = Result<LazyTaskConfig<'t>, GetTaskError>>> {
    /// [`Job::config`].
    pub config: JobConfig<'j>,
    /// [`Job::tasks`]'s [`IntoIterator::IntoIter`].
    pub tasks: I
}

impl<'j, 't, I: Iterator<Item = Result<LazyTaskConfig<'t>, GetTaskError>>> Iterator for JobIntoIterator<'j, 't, I> {
    type Item = Result<LazyTask<'j, 't>, GetTaskError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tasks.next()?.map(|config| self.config.make_lazy_task(config)))
    }
}
