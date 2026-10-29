//! [`SpecialSchemeDetails`].

use crate::prelude::*;

/// Either [`FileSchemeDetails`] or [`SpecialNotFileSchemeDetails`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SpecialSchemeDetails {
    /** [`FileSchemeDetails`].           */ File          (FileSchemeDetails          ),
    /** [`SpecialNotFileSchemeDetails`]. */ SpecialNotFile(SpecialNotFileSchemeDetails),
}

impl SpecialSchemeDetails {
    /** [`FileSchemeDetails`].                  */ pub const FILE : Self = Self::File          (FileSchemeDetails                 );
    /** [`SpecialNotFileSchemeDetails::Http`].  */ pub const HTTP : Self = Self::SpecialNotFile(SpecialNotFileSchemeDetails::Http );
    /** [`SpecialNotFileSchemeDetails::Https`]. */ pub const HTTPS: Self = Self::SpecialNotFile(SpecialNotFileSchemeDetails::Https);
    /** [`SpecialNotFileSchemeDetails::Ws`].    */ pub const WS   : Self = Self::SpecialNotFile(SpecialNotFileSchemeDetails::Ws   );
    /** [`SpecialNotFileSchemeDetails::Wss`].   */ pub const WSS  : Self = Self::SpecialNotFile(SpecialNotFileSchemeDetails::Wss  );
    /** [`SpecialNotFileSchemeDetails::Ftp`].   */ pub const FTP  : Self = Self::SpecialNotFile(SpecialNotFileSchemeDetails::Ftp  );

    /// Either [`FileSchemeDetails::as_str`] or [`SpecialNotFileSchemeDetails::as_str`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File          (x) => x.as_str(),
            Self::SpecialNotFile(x) => x.as_str(),
        }
    }

    /// The [`SchemeType`].
    pub const fn r#type(self) -> SchemeType {
        match self {
            Self::File          (x) => x.r#type(),
            Self::SpecialNotFile(x) => x.r#type(),
        }
    }

    /** If it's [`Self::SpecialNotFile`]. */ pub const fn is_special_not_file(self) -> bool {matches!(self, Self::SpecialNotFile(_))}

    /** If it's [`Self::FILE`].                    */ pub const fn is_file         (self) -> bool {matches!(self, Self::FILE               )}
    /** If it's [`Self::HTTP`].                    */ pub const fn is_http         (self) -> bool {matches!(self, Self::HTTP               )}
    /** If it's [`Self::HTTPS`].                   */ pub const fn is_https        (self) -> bool {matches!(self, Self::HTTPS              )}
    /** If it's [`Self::HTTP`] or [`Self::HTTPS`]. */ pub const fn is_http_or_https(self) -> bool {matches!(self, Self::HTTP  | Self::HTTPS)}
    /** If it's [`Self::WS`].                      */ pub const fn is_ws           (self) -> bool {matches!(self, Self::WS                 )}
    /** If it's [`Self::WSS`].                     */ pub const fn is_wss          (self) -> bool {matches!(self, Self::WSS                )}
    /** If it's [`Self::WS`] or [`Self::WSS`].     */ pub const fn is_ws_or_wss    (self) -> bool {matches!(self, Self::WS    | Self::WSS  )}
    /** If it's [`Self::FTP`].                     */ pub const fn is_ftp          (self) -> bool {matches!(self, Self::FTP                )}



    /// [`SpecialNotFileSchemeDetails::default_port`].
    pub fn default_port(self) -> Option<Port<'static>> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(x) => Some(x.default_port()),
        }
    }

    /// [`SpecialNotFileSchemeDetails::default_port_num`].
    pub const fn default_port_num(self) -> Option<u16> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(x) => Some(x.default_port_num()),
        }
    }

    /// [`SpecialNotFileSchemeDetails::default_port_str`].
    pub const fn default_port_str(self) -> Option<&'static str> {
        match self {
            Self::File          (_) => None,
            Self::SpecialNotFile(x) => Some(x.default_port_str()),
        }
    }
}

impl From<FileSchemeDetails          > for SpecialSchemeDetails {fn from(value: FileSchemeDetails          ) -> Self {Self::File          (value)}}
impl From<SpecialNotFileSchemeDetails> for SpecialSchemeDetails {fn from(value: SpecialNotFileSchemeDetails) -> Self {Self::SpecialNotFile(value)}}
