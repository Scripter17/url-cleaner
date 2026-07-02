//! [`crate::task_state`].

#[expect(unused_imports, reason = "Used in doc comments.")]
use crate::prelude::*;

/// Generates a [`TaskState`] for doctests.
///
/// Does some very bad and stupid thing and should not be used in "real" code.
#[macro_export]
macro_rules! task_state {
    ($($task:expr)?) => {
        {
            let task = "https://example.com"; $(let task = $task;)?

            let Task {url, context} = task.try_into().expect("???");

            $crate::prelude::TaskState {
                url,
                context,
                job: Box::leak(Box::new($crate::prelude::Job {
                    context: Default::default(),
                    cleaner: Default::default(),
                    unthreader: Box::leak(Box::new(Default::default())),
                    secrets: Box::leak(Box::new(Default::default())),
                    #[cfg(feature = "cache")]
                    cache: $crate::prelude::Cache {
                        inner: Box::leak(Box::new(Default::default())),
                        config: Default::default(),
                    },
                    #[cfg(feature = "http")]
                    http_client: Box::leak(Box::new(Default::default()))
                }))
            }
        }
    }
}
