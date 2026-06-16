//! [`SpecialSchemeDetails`].

use crate::prelude::*;

/// Either [`FileSchemeDetails`] or [`SpecialNotFileSchemeDetails`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialSchemeDetails {
    /// [`FileSchemeDetails`].
    File(FileSchemeDetails),
    /// [`SpecialNotFileSchemeDetails`].
    SpecialNotFile(SpecialNotFileSchemeDetails),
}

impl SpecialSchemeDetails {
    /// The [`SchemeType`].
    pub fn r#type(self) -> SchemeType {
        match self {
            Self::File          (x) => x.r#type(),
            Self::SpecialNotFile(x) => x.r#type(),
        }
    }

    /// If it's [`Self::File`].
    pub fn is_file(self) -> bool {
        matches!(self, Self::File(_))
    }

    /// If it's [`Self::SpecialNotFile`].
    pub fn is_special_not_file(self) -> bool {
        matches!(self, Self::SpecialNotFile(_))
    }



    /// [`SpecialNotFileSchemeDetails::is_http`].
    pub fn is_http(self) -> bool {
        match self {
            Self::File          (_) => false,
            Self::SpecialNotFile(x) => x.is_http(),
        }
    }

    /// [`SpecialNotFileSchemeDetails::is_https`].
    pub fn is_https(self) -> bool {
        match self {
            Self::File          (_) => false,
            Self::SpecialNotFile(x) => x.is_https(),
        }
    }

    /// [`SpecialNotFileSchemeDetails::is_http_or_https`].
    pub fn is_http_or_https(self) -> bool {
        match self {
            Self::File          (_) => false,
            Self::SpecialNotFile(x) => x.is_http_or_https(),
        }
    }

    /// [`SpecialNotFileSchemeDetails::is_ws`].
    pub fn is_ws(self) -> bool {
        match self {
            Self::File          (_) => false,
            Self::SpecialNotFile(x) => x.is_ws(),
        }
    }

    /// [`SpecialNotFileSchemeDetails::is_wss`].
    pub fn is_wss(self) -> bool {
        match self {
            Self::File          (_) => false,
            Self::SpecialNotFile(x) => x.is_wss(),
        }
    }

    /// [`SpecialNotFileSchemeDetails::is_ws_or_wss`].
    pub fn is_ws_or_wss(self) -> bool {
        match self {
            Self::File          (_) => false,
            Self::SpecialNotFile(x) => x.is_ws_or_wss(),
        }
    }

    /// [`SpecialNotFileSchemeDetails::is_ftp`].
    pub fn is_ftp(self) -> bool {
        match self {
            Self::File          (_) => false,
            Self::SpecialNotFile(x) => x.is_ftp(),
        }
    }



    /// The default port.
    pub fn default_port(self) -> Option<u16> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(x) => Some(x.default_port()),
        }
    }

    /// The default port as a [`str`].
    pub fn default_port_str(self) -> Option<&'static str> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(x) => Some(x.default_port_str()),
        }
    }
}

impl From<FileSchemeDetails          > for SpecialSchemeDetails {fn from(value: FileSchemeDetails          ) -> Self {Self::File          (value)}}
impl From<SpecialNotFileSchemeDetails> for SpecialSchemeDetails {fn from(value: SpecialNotFileSchemeDetails) -> Self {Self::SpecialNotFile(value)}}
