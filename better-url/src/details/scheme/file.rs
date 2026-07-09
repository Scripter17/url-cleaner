//! [`FileSchemeDetails`].

use crate::prelude::*;

/// The details of the file scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileSchemeDetails;

impl FileSchemeDetails {
    /// The scheme as a [`str`].
    pub fn as_str(self) -> &'static str {
        "file"
    }

    /// The [`SchemeType`].
    pub fn r#type(self) -> SchemeType {
        SchemeType::File
    }
}
