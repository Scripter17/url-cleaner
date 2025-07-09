//! Types used by URL Cleaner Site.
//!
//! Can be used to parse its output.

pub mod clean;
pub use clean::*;
pub mod get_host_parts;
pub use get_host_parts::*;
pub(crate) mod util;

pub use clean::{JobConfig, CleanResult};
pub use get_host_parts::GetHostPartsResult;
