//! The engine of URL Cleaner.
//!
//! Can be used to make various frontends like [CLIs](https://github.com/Scripter17/url-cleaner/tree/main/cli), [HTTP servers](https://github.com/Scripter17/url-cleaner/tree/main/site), and [discord apps/bots](https://github.com/Scripter17/url-cleaner/tree/main/discord-app).
//!
//! The main types you want to start rabbit holes from are [`Job`], [`Cleaner`], and [`ProfiledCleaner`].

#[expect(unused_imports, reason = "Used in doc comments.")]
use prelude::*;

pub mod job;
pub mod task;
pub mod cleaner;
pub mod profiled_cleaner;
pub mod set;
pub mod map;
pub mod partitioning;
pub mod refs;
pub mod unthreader;
pub mod glue;
pub mod testing;
pub(crate) mod util;

/// A prelude module to make importing all the various types nicer.
///
/// Generally not meant for external use.
pub mod prelude {
    pub use better_url::*;

    pub use crate::job::prelude::*;
    pub use crate::task::prelude::*;
    pub use crate::cleaner::prelude::*;
    pub use crate::profiled_cleaner::prelude::*;
    pub use crate::set::*;
    pub use crate::map::*;
    pub use crate::partitioning::*;
    pub use crate::refs::prelude::*;
    pub use crate::unthreader::prelude::*;
    pub use crate::glue::prelude::*;

    pub(crate) use crate::util::*;

    pub use crate::task_state as ts;
    pub use crate::task_state_view as tsv;
    pub use crate::task;
}
