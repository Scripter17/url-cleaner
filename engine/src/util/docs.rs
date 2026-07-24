//! Doc stuff.

#[expect(unused_imports, reason = "Used in doc comments.")]
use crate::prelude::*;

/// Create a minimal [`Job`] for use in doctests.
#[macro_export]
macro_rules! job {
    () => {
        $crate::prelude::Job {
            context: Default::default(),
            cleaner: Default::default(),
            unthreader: &Default::default(),
            secrets: &Default::default(),
            #[cfg(feature = "cache")]
            cache: $crate::prelude::Cache {
                inner: &Default::default(),
                config: Default::default(),
            },
            #[cfg(feature = "http")]
            http_client: &$crate::prelude::MaybeHttpClient(None),
        }
    }
}
