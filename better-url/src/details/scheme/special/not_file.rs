//! [`SpecialNotFileSchemeDetails`].

use crate::prelude::*;

/// Details for a special but not file scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialNotFileSchemeDetails {
    /// `http`.
    Http,
    /// `https`.
    Https,
    /// `ws`.
    Ws,
    /// `wss`.
    Wss,
    /// `ftp`.
    Ftp,
}

impl SpecialNotFileSchemeDetails {
    /// The [`SchemeType`].
    pub fn r#type(self) -> SchemeType {
        SchemeType::SpecialNotFile
    }

    /// The default port.
    pub fn default_port(self) -> u16 {
        match self {
            Self::Http  | Self::Ws  =>  80,
            Self::Https | Self::Wss => 443,
            Self::Ftp               =>  21,
        }
    }

    /// The default port as a [`str`].
    pub fn default_port_str(self) -> &'static str {
        match self {
            Self::Http  | Self::Ws  =>  "80",
            Self::Https | Self::Wss => "443",
            Self::Ftp               =>  "21",
        }
    }
}
