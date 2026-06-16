use std::num::NonZero;
use std::borrow::Cow;

use thiserror::Error;

use better_url::prelude::*;

mod file;
mod special_not_file;
mod non_special;

#[derive(Debug, Error)]
pub enum InvalidUrl {
    #[error(transparent)]
    InvalidScheme(#[from] InvalidScheme),
    #[error(transparent)]
    InvalidHost(#[from] InvalidHost),
    #[error(transparent)]
    InvalidPort(#[from] InvalidPort),
    #[error(transparent)]
    TooLong(#[from] TooLong),
    #[error("Other")]
    Other
}

#[derive(Debug, Clone)]
pub struct MyUrl {
    /// The serialization.
    serialization : String,
    /// The end of the scheme.
    scheme_end    : u32,
    /// The end of the username.
    username_end  : Option<NonZero<u32>>,
    /// The start of the host.
    host_start    : Option<NonZero<u32>>,
    /// The end of the host.
    host_end      : Option<NonZero<u32>>,
    /// If [`Self::host_end`] is [`Some`] and not [`Self::path_start`], the port.
    port          : u16,
    /// The start of the path.
    path_start    : u32,
    /// The start of the query.
    query_start   : Option<NonZero<u32>>,
    /// The start of the fragment.
    fragment_start: Option<NonZero<u32>>,
    /// The [`UrlDetails`].
    details       : UrlDetails,
}

impl MyUrl {
    pub fn as_str(&self) -> &str {
        &self.serialization
    }
    
    pub fn new(value: &str) -> Result<Self, InvalidUrl> {
        let start = value.bytes(). position(|b| b > 0x20 && b != 0x7F).unwrap_or(0);
        let end   = value.bytes().rposition(|b| b > 0x20 && b != 0x7F).unwrap_or(value.len());

        let mut value = Cow::Borrowed(&value[start..=end]);

        if value.bytes().any(|b| b == b'\t' || b == b'\n' || b == b'\r') {
            value.to_mut().retain(|c| c != '\t' && c != '\n' && c != '\r');
        }

        let (scheme, rest) = match value.split_once(':') {
            Some((scheme, rest)) => (scheme, rest),
            None                 => Err(InvalidUrl::Other)?
        };

        let scheme = Scheme::new(scheme)?;

        match scheme.r#type() {
            SchemeType::File           => Self::new_file            (scheme, rest),
            SchemeType::SpecialNotFile => Self::new_special_not_file(scheme, rest),
            SchemeType::NonSpecial     => Self::new_non_special     (scheme, rest),
        }
    }
}
