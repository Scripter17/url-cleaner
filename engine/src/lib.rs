//! The engine of URL Cleaner.
//!
//! Can be used to make various frontends like [CLIs](https://github.com/Scripter17/url-cleaner/tree/main/cli), [HTTP servers](https://github.com/Scripter17/url-cleaner/tree/main/site), and [discord apps/bots](https://github.com/Scripter17/url-cleaner/tree/main/discord-app).
//!
//! The main types you want to start rabbit holes from are [`Job`], [`Cleaner`], and [`ProfiledCleaner`].

#[expect(unused_imports, reason = "Used in doc comments.")]
use prelude::*;

pub mod job;
pub mod cleaner;
pub mod unthreader;

#[cfg(feature = "http" )] pub mod http;
#[cfg(feature = "cache")] pub mod cache;

pub mod testing;
pub(crate) mod util;

/// A prelude module to make importing all the various types nicer.
///
/// Generally not meant for external use.
pub mod prelude {
    pub use super::job::prelude::*;
    pub use super::cleaner::prelude::*;
    pub use super::unthreader::prelude::*;

    #[cfg(feature = "http" )] pub use super::http::prelude::*;
    #[cfg(feature = "cache")] pub use super::cache::prelude::*;

    pub use super::task_state as ts;
    pub use super::task_state_view as tsv;
    pub use super::task;

    pub(crate) use better_url::*;
    pub(crate) use super::util::*;
}
