//! [`crate::prelude::DomainSegment`].

mod to_ascii;
mod to_unicode;
mod joiners;
mod idna_table;

pub use to_ascii::*;
pub use to_unicode::*;
pub use joiners::*;
pub use idna_table::*;
