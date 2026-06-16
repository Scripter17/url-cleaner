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
pub enum SchemeDetails {
    /// [`SpecialSchemeDetails`].
    Special(SpecialSchemeDetails),
    /// [`NonSpecialSchemeDetails`].
    NonSpecial(NonSpecialSchemeDetails),
}

impl SchemeDetails {
    /// Make a new [`Self`].
    /// # Errors
    /// If `value` is an invalid scheme, returns the error [`InvalidScheme`].
    pub fn new(value: &str) -> Result<Self, InvalidScheme> {
             if value.eq_ignore_ascii_case("http" ) {Ok(SpecialNotFileSchemeDetails::Http .into())}
        else if value.eq_ignore_ascii_case("https") {Ok(SpecialNotFileSchemeDetails::Https.into())}
        else if value.eq_ignore_ascii_case("ws"   ) {Ok(SpecialNotFileSchemeDetails::Ws   .into())}
        else if value.eq_ignore_ascii_case("wss"  ) {Ok(SpecialNotFileSchemeDetails::Wss  .into())}
        else if value.eq_ignore_ascii_case("ftp"  ) {Ok(SpecialNotFileSchemeDetails::Ftp  .into())}
        else if value.eq_ignore_ascii_case("file" ) {Ok(FileSchemeDetails                 .into())}
        else if is_valid_scheme(value)              {Ok(NonSpecialSchemeDetails           .into())}
        else {Err(InvalidScheme)}
    }

    /// Make a new [`Self`] from a [`url::Url`].
    pub fn from_url(value: &url::Url) -> Self {
        Self::new_unchecked(value.scheme())
    }

    /// Make a new [`Self`] without case or validity checks.
    pub(crate) fn new_unchecked(value: &str) -> Self {
        match value {
            "http"  => SpecialNotFileSchemeDetails::Http .into(),
            "https" => SpecialNotFileSchemeDetails::Https.into(),
            "ws"    => SpecialNotFileSchemeDetails::Ws   .into(),
            "wss"   => SpecialNotFileSchemeDetails::Wss  .into(),
            "ftp"   => SpecialNotFileSchemeDetails::Ftp  .into(),
            "file"  => FileSchemeDetails                 .into(),
            _       => NonSpecialSchemeDetails           .into(),
        }
    }

    /// The [`SchemeType`].
    pub fn r#type(self) -> SchemeType {
        match self {
            Self::Special   (x) => x.r#type(),
            Self::NonSpecial(x) => x.r#type(),
        }
    }

    /// [`SchemeType::is_special`].
    pub fn is_special(self) -> bool {
        self.r#type().is_special()
    }

    /// [`SchemeType::is_special_not_file`].
    pub fn is_special_not_file(self) -> bool {
        self.r#type().is_special_not_file()
    }

    /// [`SchemeType::is_file`].
    pub fn is_file(self) -> bool {
        self.r#type().is_file()
    }

    /// [`SchemeType::is_non_special`].
    pub fn is_non_special(self) -> bool {
        self.r#type().is_non_special()
    }



    /// [`SpecialSchemeDetails::is_http`].
    pub fn is_http(self) -> bool {
        match self {
            Self::Special   (x) => x.is_http(),
            Self::NonSpecial(_) => false,
        }
    }

    /// [`SpecialSchemeDetails::is_https`].
    pub fn is_https(self) -> bool {
        match self {
            Self::Special   (x) => x.is_https(),
            Self::NonSpecial(_) => false,
        }
    }

    /// [`SpecialSchemeDetails::is_http_or_https`].
    pub fn is_http_or_https(self) -> bool {
        match self {
            Self::Special   (x) => x.is_http_or_https(),
            Self::NonSpecial(_) => false,
        }
    }

    /// [`SpecialSchemeDetails::is_ws`].
    pub fn is_ws(self) -> bool {
        match self {
            Self::Special   (x) => x.is_ws(),
            Self::NonSpecial(_) => false,
        }
    }

    /// [`SpecialSchemeDetails::is_wss`].
    pub fn is_wss(self) -> bool {
        match self {
            Self::Special   (x) => x.is_wss(),
            Self::NonSpecial(_) => false,
        }
    }

    /// [`SpecialSchemeDetails::is_ws_or_wss`].
    pub fn is_ws_or_wss(self) -> bool {
        match self {
            Self::Special   (x) => x.is_ws_or_wss(),
            Self::NonSpecial(_) => false,
        }
    }

    /// [`SpecialSchemeDetails::is_ftp`].
    pub fn is_ftp(self) -> bool {
        match self {
            Self::Special   (x) => x.is_ftp(),
            Self::NonSpecial(_) => false,
        }
    }



    /// [`SpecialSchemeDetails::default_port`].
    pub fn default_port(self) -> Option<u16> {
        match self {
            Self::Special   (x) => x.default_port(),
            Self::NonSpecial(_) => None
        }
    }

    /// [`SpecialSchemeDetails::default_port_str`].
    pub fn default_port_str(self) -> Option<&'static str> {
        match self {
            Self::Special   (x) => x.default_port_str(),
            Self::NonSpecial(_) => None
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
