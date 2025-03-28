//! URL Cleaner is an extremely powerful and configurable URL manipulation tool specialized for cleaning URLs in bulk.
//!
//! Cleaning is the process of taking a URL and removing unneccesary components used to, for example, identify you as the person who sent your friend a tweet.

pub mod glue;
pub mod types;
pub mod testing;
pub(crate) mod util;

pub use types::{Config, Job, TaskConfig};
