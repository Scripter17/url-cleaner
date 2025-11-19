//! Types used by URL Cleaner Site.
//!
//! Can be used to parse its output.

pub mod clean;
pub use clean::*;
pub mod auth;
pub use auth::*;
pub mod info;
pub use info::*;
#[cfg(feature = "server")]
pub mod server;
pub(crate) mod util;
