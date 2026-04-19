//! IDNA stuff.

mod uts46;
mod punycode;
mod label;
mod domain;

pub use uts46::*;
pub use punycode::*;
pub use label::*;
pub use domain::*;
