//! Doc stuff.

#[expect(unused_imports, reason = "Used in doc comments.")]
use crate::prelude::*;

/// Create a minimal [`Job`] for use in doctests.
#[macro_export]
macro_rules! doc_job {
    ($name:ident) => {
        #[cfg(any(feature = "http", feature = "cache"))]
        let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();

        #[cfg(feature = "http")]
        let http_client = HttpClient::new_sync(runtime.handle().clone());

        #[cfg(feature = "cache")]
        let cache_client = CacheClient::new_sync(CacheTarget::Memory, runtime.handle().clone());

        let $name = $crate::prelude::Job {
            context: Default::default(),
            cleaner: Default::default(),
            secrets: &Default::default(),
            unthreader: None,
            #[cfg(feature = "http")]
            http_client: Some(&http_client),
            #[cfg(feature = "cache")]
            cache_client: &cache_client,
            #[cfg(feature = "cache")]
            cache_config: Default::default(),
        };
    }
}
