//! URL Cleaner originally started as a project to remove tracking garbage from URLs but has since grown into a very powerful URL manipulation tool.

pub mod glue;
pub mod types;
pub(crate) mod util;

pub use types::{Config, Jobs, JobConfig};
