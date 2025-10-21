//! Glue for [`reqwest`].

pub mod client;
pub mod proxy;
pub mod request;
pub mod body;
pub mod response;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::client::*;
    pub use super::proxy::*;
    pub use super::request::*;
    pub use super::body::*;
    pub use super::response::*;
}
