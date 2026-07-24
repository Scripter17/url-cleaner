//! [`NonSpecialSchemeDetails`].

use crate::prelude::*;

/// Details for a non-special scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NonSpecialSchemeDetails;


impl NonSpecialSchemeDetails {
    /// The [`SchemeType`].
    pub fn r#type(self) -> SchemeType {
        SchemeType::NonSpecial
    }
}
