//! A wrapper around the [`url`] crate that provides higher level operations.

mod better_url;
pub use better_url::*;
mod better_host;
pub use better_host::*;
mod util;
pub(crate) use util::*;
