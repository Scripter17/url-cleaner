//! [`crate::prelude::DomainSegment`].

mod encode;
mod decode;
mod joiners;
mod idna_table;

pub use encode::*;
pub use decode::*;
pub use joiners::*;
pub use idna_table::*;
