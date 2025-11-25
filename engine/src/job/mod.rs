//! [`Job`] and co.

use crate::prelude::*;

pub mod job_config;
pub mod job_context;
pub mod job_into_iterator;
pub mod task;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::job_config::*;
    pub use super::job_context::*;
    pub use super::job_into_iterator::*;
    pub use super::task::prelude::*;

    pub use super::Job;
}

/// The main way to turn [`LazyTaskConfig`]s into [`LazyTask`]s to be [`LazyTask::make`]d and [`Task::do`]ne.
///
/// Combines a [`JobConfig`] and an [`IntoIterator`] for convenience when practical.
///
/// However sometimes it's very hard to fit your queue of tasks into an [`Iterator`]. For that please feel free to use [`JobConfig`] directly.
///
/// The CLI frontend does.
/// # Examples
/// ```
/// use std::borrow::Cow;
/// use url_cleaner_engine::prelude::*;
///
/// // A Cleaner is the logic for how URLs are actually cleaned.
/// // Separated for emphasis that they should be used for multiple jobs.
/// let cleaner = Cleaner {
///     // The actions to modify the URL.
///     actions: Cow::Owned(vec![
///         // This is a simple example so we're only removing the one query parameter we use.
///         // Action::RemoveQueryParams (note the plural) can be used to remove any query parameter in a HashMap.
///         Action::RemoveQueryParam("utm_source".into())
///     ]),
///     // Docs, Params, etc. can just be blank.
///     ..Default::default()
/// };
///
/// let job = Job {
///     config: JobConfig {
///         // Information about a job, such as the webpage the URLs came from.
///         context: &Default::default(),
///         // The Cleaner defined above.
///         cleaner: &cleaner,
///         // This comes up a bit in certain situations such as browser extensions, but usually it's fine to just use the default.
///         unthreader: &Default::default(),
#[cfg_attr(feature = "cache" , doc = "        // Just making an in-memory cache because this is just example code.")]
#[cfg_attr(feature = "cache" , doc = "        cache: Cache {")]
#[cfg_attr(feature = "cache" , doc = "            config: Default::default(),")]
#[cfg_attr(feature = "cache" , doc = "            inner: &Default::default(),")]
#[cfg_attr(feature = "cache" , doc = "        },")]
#[cfg_attr(feature = "http"  , doc = "        http_client: &Default::default()")]
///     },
///     // The actual URLs, well, "tasks", which are a URL and some context, to clean/"do".
///     tasks: [
///         Ok("https://example.com?utm_source=url_cleaner".into())
///     ]
/// };
///
/// for task in job {
///     // Prints https://example.com/.
///     println!("{}", task.unwrap().make().unwrap().r#do().unwrap());
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Job<'j, 't, I: IntoIterator<Item = Result<LazyTaskConfig<'t>, GetTaskError>>> {
    /// The [`JobConfig`] whose [`JobConfig::make_lazy_task`] each [`LazyTaskConfig`] from [`Self::tasks`] is given to.
    pub config: JobConfig<'j>,
    /// The source of [`LazyTaskConfig`]s to turn into [`LazyTask`]s via [`Self::config`]'s [`JobConfig::make_lazy_task`].
    pub tasks: I
}

impl<'j, 't, I: IntoIterator<Item = Result<LazyTaskConfig<'t>, GetTaskError>>> IntoIterator for Job<'j, 't, I> {
    type IntoIter = JobIntoIterator<'j, 't, I::IntoIter>;
    type Item = Result<LazyTask<'j, 't>, GetTaskError>;

    fn into_iter(self) -> Self::IntoIter {
        JobIntoIterator {
            config: self.config,
            tasks: self.tasks.into_iter()
        }
    }
}
