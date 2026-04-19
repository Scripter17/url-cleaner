//! [`NonSpecialSchemeDetails`].

use crate::prelude::*;

/// Details for a non-special scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NonSpecialSchemeDetails;


impl NonSpecialSchemeDetails {
    /// The [`SchemeType`].
    pub fn r#type(self) -> SchemeType {
        SchemeType::NonSpecial
    }
}
