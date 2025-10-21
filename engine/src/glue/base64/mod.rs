//! Glue for [`base64`].

pub mod config;
pub mod alphabet;
pub mod decode_padding_mode;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::config::*;
    pub use super::alphabet::*;
    pub use super::decode_padding_mode::*;
}
