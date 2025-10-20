//! The engine of URL Cleaner.
//!
//! Can be used with various frontends like [CLIs](https://github.com/Scripter17/url-cleaner/tree/main/cli), [HTTP servers](https://github.com/Scripter17/url-cleaner/tree/main/site), and [discord apps/bots](https://github.com/Scripter17/url-cleaner/tree/main/discord-app).
//!
//! The main types you want to start rabbit holes from are [`Job`], [`Cleaner`], and [`ProfiledCleaner`].

pub mod job;
pub mod task;
pub mod cleaner;
pub mod data_structures;
pub mod refs;
pub mod unthreader;
pub mod glue;
pub mod testing;
pub(crate) mod util;

/// A prelude module to make importing all the various types.
///
/// Generally not meant for external use.
pub mod prelude {
    pub use better_url::*;

    pub use crate::job::*;
    pub use crate::task::*;
    pub use crate::cleaner::*;
    pub use crate::data_structures::*;
    pub use crate::refs::*;
    pub use crate::unthreader::*;

    pub use crate::glue::prelude::*;

    pub(crate) use crate::util::*;

    pub use crate::task_state as ts;
    pub use crate::task_state_view as tsv;
    pub use crate::task;
}

use prelude::*;
