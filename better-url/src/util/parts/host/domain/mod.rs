//! Domain stuff.

mod uts46;
mod punycode;
mod validity;
mod segment;
mod segments;
mod host;
#[cfg(feature = "psl")] mod psl;

pub use uts46::*;
pub use punycode::*;
pub use validity::*;
pub use segment::*;
pub use segments::*;
pub use host::*;
#[cfg(feature = "psl")] pub use psl::*;
