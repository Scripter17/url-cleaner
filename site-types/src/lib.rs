//! Types used by URL Cleaner Site.
//!
//! Can be used to parse its output.

pub mod clean;
pub use clean::*;
pub mod get_domain_suffix;
pub use get_domain_suffix::*;
pub(crate) mod util;

pub use clean::{JobConfig, CleanResult};
pub use get_domain_suffix::GetDomainSuffixResult;
