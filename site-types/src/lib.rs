//! Types used by URL Cleaner Site.
//!
//! Can be used to parse its output.

pub mod clean;
pub use clean::*;
pub mod auth;
pub use auth::*;
pub(crate) mod util;

pub use clean::{JobConfig, CleanResult};
