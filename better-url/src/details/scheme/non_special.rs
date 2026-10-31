//! [`NonSpecialSchemeDetails`].

use crate::prelude::*;

/// Details for a non-special scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct NonSpecialSchemeDetails;


impl NonSpecialSchemeDetails {
    /// The [`SchemeType`].
    pub const fn r#type(self) -> SchemeType {
        SchemeType::NonSpecial
    }
}
