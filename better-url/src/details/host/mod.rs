//! Details.

mod either;
mod file;
mod special_not_file;
mod non_special;
mod ipv4;
mod ipv6;
mod domain;
mod opaque;
mod empty;

pub use either::*;
pub use file::*;
pub use special_not_file::*;
pub use non_special::*;
pub use ipv4::*;
pub use ipv6::*;
pub use domain::*;
pub use opaque::*;
pub use empty::*;
