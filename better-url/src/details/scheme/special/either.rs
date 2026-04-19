//! [`SpecialSchemeDetails`].

use crate::prelude::*;

/// Either a [`SpecialNotFileSchemeDetails`] or a [`FileSchemeDetails`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialSchemeDetails {
    /// [`SpecialNotFileSchemeDetails`].
    SpecialNotFile(SpecialNotFileSchemeDetails),
    /// [`FileSchemeDetails`].
    File(FileSchemeDetails),
}

impl SpecialSchemeDetails {
    /// The [`SchemeType`].
    pub fn r#type(self) -> SchemeType {
        match self {
            Self::SpecialNotFile(x) => x.r#type(),
            Self::File          (x) => x.r#type(),
        }
    }

    /// The default port.
    pub fn default_port(self) -> Option<u16> {
        match self {
            Self::SpecialNotFile(x) => Some(x.default_port()),
            Self::File          (_) => None
        }
    }

    /// The default port as a [`str`].
    pub fn default_port_str(self) -> Option<&'static str> {
        match self {
            Self::SpecialNotFile(x) => Some(x.default_port_str()),
            Self::File          (_) => None
        }
    }
}

impl From<SpecialNotFileSchemeDetails> for SpecialSchemeDetails {
    fn from(value: SpecialNotFileSchemeDetails) -> Self {
        Self::SpecialNotFile(value)
    }
}

impl From<FileSchemeDetails> for SpecialSchemeDetails {
    fn from(value: FileSchemeDetails) -> Self {
        Self::File(value)
    }
}

