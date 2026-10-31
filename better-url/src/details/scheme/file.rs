//! [`FileSchemeDetails`].

use crate::prelude::*;

/// The details of the file scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct FileSchemeDetails;

impl FileSchemeDetails {
    /// The scheme as a [`str`].
    pub const fn as_str(self) -> &'static str {
        "file"
    }

    /// The [`SchemeType`].
    pub const fn r#type(self) -> SchemeType {
        SchemeType::File
    }
}
