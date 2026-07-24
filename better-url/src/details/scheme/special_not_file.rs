//! [`SpecialNotFileSchemeDetails`].

use crate::prelude::*;

/// Details for a special but not file scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    /// The scheme as a [`str`].
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Http  => "http" ,
            Self::Https => "https",
            Self::Ws    => "ws"   ,
            Self::Wss   => "wss"  ,
            Self::Ftp   => "ftp"  ,
        }
    }

    /// The [`SchemeType`].
    pub fn r#type(self) -> SchemeType {
        SchemeType::SpecialNotFile
    }



    /// If it's [`Self::Http`].
    pub fn is_http(self) -> bool {
        matches!(self, Self::Http)
    }

    /// If it's [`Self::Https`].
    pub fn is_https(self) -> bool {
        matches!(self, Self::Https)
    }

    /// If it's [`Self::Http`] or [`Self::Https`].
    pub fn is_http_or_https(self) -> bool {
        matches!(self, Self::Http | Self::Https)
    }

    /// If it's [`Self::Ws`].
    pub fn is_ws(self) -> bool {
        matches!(self, Self::Ws)
    }

    /// If it's [`Self::Wss`].
    pub fn is_wss(self) -> bool {
        matches!(self, Self::Wss)
    }

    /// If it's [`Self::Ws`] or [`Self::Wss`].
    pub fn is_ws_or_wss(self) -> bool {
        matches!(self, Self::Ws | Self::Wss)
    }

    /// If it's [`Self::Ftp`].
    pub fn is_ftp(self) -> bool {
        matches!(self, Self::Ftp)
    }



    /// The default [`Port`].
    pub fn default_port(self) -> Port<'static> {
        Port {
            port: self.default_port_str().into(),
            port_num: self.default_port_num()
        }
    }

    /// The default port as a [`u16`].
    pub fn default_port_num(self) -> u16 {
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
