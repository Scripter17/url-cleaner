//! The engine of URL Cleaner.
//!
//! Can be integrated into other projects for things like [CLIs](https://github.com/Scripter17/url-cleaner/tree/main/url-cleaner) and [HTTP servers](https://github.com/Scripter17/url-cleaner/tree/main/url-cleaner-site).

pub mod glue;
pub mod types;
pub mod testing;
pub(crate) mod util;

pub use types::{Cleaner, Job, LazyTaskConfig};
