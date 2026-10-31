//! [`SchemeDetails`].

use crate::prelude::*;

/// Either a [`SpecialSchemeDetails`] or a [`NonSpecialSchemeDetails`].
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// assert_eq!(std::mem::size_of::<SchemeDetails>(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub enum SchemeDetails {
    /** [`SpecialSchemeDetails`].    */ Special   (SpecialSchemeDetails   ),
    /** [`NonSpecialSchemeDetails`]. */ NonSpecial(NonSpecialSchemeDetails),
}

impl SchemeDetails {
    /** [`FileSchemeDetails`].                  */ pub const FILE       : Self = Self::Special(SpecialSchemeDetails::File          (FileSchemeDetails                 ));
    /** [`SpecialNotFileSchemeDetails::Http`].  */ pub const HTTP       : Self = Self::Special(SpecialSchemeDetails::SpecialNotFile(SpecialNotFileSchemeDetails::Http ));
    /** [`SpecialNotFileSchemeDetails::Https`]. */ pub const HTTPS      : Self = Self::Special(SpecialSchemeDetails::SpecialNotFile(SpecialNotFileSchemeDetails::Https));
    /** [`SpecialNotFileSchemeDetails::Ws`].    */ pub const WS         : Self = Self::Special(SpecialSchemeDetails::SpecialNotFile(SpecialNotFileSchemeDetails::Ws   ));
    /** [`SpecialNotFileSchemeDetails::Wss`].   */ pub const WSS        : Self = Self::Special(SpecialSchemeDetails::SpecialNotFile(SpecialNotFileSchemeDetails::Wss  ));
    /** [`SpecialNotFileSchemeDetails::Ftp`].   */ pub const FTP        : Self = Self::Special(SpecialSchemeDetails::SpecialNotFile(SpecialNotFileSchemeDetails::Ftp  ));
    /** [`NonSpecialSchemeDetails`].            */ pub const NON_SPECIAL: Self = Self::NonSpecial(NonSpecialSchemeDetails);

    /// Parse a scheme literal.
    /// # Errors
    /// If `value` is an invalid scheme literal, returns the error [`InvalidScheme`].
    pub fn new(value: &str) -> Result<Self, InvalidScheme> {
        Ok(match value {
            "http"  => Self::HTTP ,
            "https" => Self::HTTPS,
            "ws"    => Self::WS   ,
            "wss"   => Self::WSS  ,
            "ftp"   => Self::FTP  ,
            "file"  => Self::FILE ,
            _       => match is_valid_scheme_literal(value) {
                true  => Self::NON_SPECIAL,
                false => Err(InvalidScheme)?,
            },
        })
    }

    /// Parse a scheme literal without validity checks.
    pub fn new_unchecked(value: &str) -> Self {
        match value {
            "http"  => Self::HTTP       ,
            "https" => Self::HTTPS      ,
            "ws"    => Self::WS         ,
            "wss"   => Self::WSS        ,
            "ftp"   => Self::FTP        ,
            "file"  => Self::FILE       ,
            _       => Self::NON_SPECIAL,
        }
    }

    /// The [`SchemeType`].
    pub const fn r#type(self) -> SchemeType {
        match self {
            Self::Special   (x) => x.r#type(),
            Self::NonSpecial(x) => x.r#type(),
        }
    }

    /** [`SchemeType::is_special`].          */ pub const fn is_special         (self) -> bool {self.r#type().is_special         ()}
    /** [`SchemeType::is_special_not_file`]. */ pub const fn is_special_not_file(self) -> bool {self.r#type().is_special_not_file()}
    /** [`SchemeType::is_non_special`].      */ pub const fn is_non_special     (self) -> bool {self.r#type().is_non_special     ()}



    /** If it's [`Self::FILE`].                    */ pub const fn is_file         (self) -> bool {matches!(self, Self::FILE               )}
    /** If it's [`Self::HTTP`].                    */ pub const fn is_http         (self) -> bool {matches!(self, Self::HTTP               )}
    /** If it's [`Self::HTTPS`].                   */ pub const fn is_https        (self) -> bool {matches!(self, Self::HTTPS              )}
    /** If it's [`Self::HTTP`] or [`Self::HTTPS`]. */ pub const fn is_http_or_https(self) -> bool {matches!(self, Self::HTTP  | Self::HTTPS)}
    /** If it's [`Self::WS`].                      */ pub const fn is_ws           (self) -> bool {matches!(self, Self::WS                 )}
    /** If it's [`Self::WSS`].                     */ pub const fn is_wss          (self) -> bool {matches!(self, Self::WSS                )}
    /** If it's [`Self::WS`] or [`Self::WSS`].     */ pub const fn is_ws_or_wss    (self) -> bool {matches!(self, Self::WS    | Self::WSS  )}
    /** If it's [`Self::FTP`].                     */ pub const fn is_ftp          (self) -> bool {matches!(self, Self::FTP                )}



    /// [`SpecialSchemeDetails::default_port`].
    pub fn default_port(self) -> Option<Port<'static>> {
        match self {
            Self::Special   (x) => x.default_port(),
            Self::NonSpecial(_) => None,
        }
    }

    /// [`SpecialSchemeDetails::default_port_num`].
    pub const fn default_port_num(self) -> Option<u16> {
        match self {
            Self::Special   (x) => x.default_port_num(),
            Self::NonSpecial(_) => None,
        }
    }

    /// [`SpecialSchemeDetails::default_port_str`].
    pub const fn default_port_str(self) -> Option<&'static str> {
        match self {
            Self::Special   (x) => x.default_port_str(),
            Self::NonSpecial(_) => None,
        }
    }
}

impl From<SpecialSchemeDetails       > for SchemeDetails {fn from(value: SpecialSchemeDetails       ) -> Self {Self::Special   (value       )}}
impl From<SpecialNotFileSchemeDetails> for SchemeDetails {fn from(value: SpecialNotFileSchemeDetails) -> Self {Self::Special   (value.into())}}
impl From<FileSchemeDetails          > for SchemeDetails {fn from(value: FileSchemeDetails          ) -> Self {Self::Special   (value.into())}}
impl From<NonSpecialSchemeDetails    > for SchemeDetails {fn from(value: NonSpecialSchemeDetails    ) -> Self {Self::NonSpecial(value       )}}

impl FromStr for SchemeDetails {
    type Err = InvalidScheme;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for SchemeDetails {
    type Error = InvalidScheme;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
