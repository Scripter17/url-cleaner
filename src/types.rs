use std::borrow::Cow;

use serde::Deserialize;
use url::{Url, ParseError};
use thiserror::Error;

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum UrlPartName {
    Whole,
    Host,
    Domain,
    Path,
    Query,
    Fragment
}

#[derive(Debug, Error)]
pub enum PartReplaceError {
    #[error("Could not convert result of replacement as a URL")]
    UrlParseError(#[from] ParseError)
}

impl UrlPartName {
    pub fn get_from<'a>(&self, url: &'a Url) -> Option<Cow<'a, str>> {
        Some(match self {
            Self::Whole    => Cow::Borrowed(url.as_str()),
            Self::Host     => Cow::Owned   (url.host()?.to_string()), // IPV4/6 hosts need to be converted into Strings
            Self::Domain   => Cow::Borrowed(url.domain()?),
            Self::Path     => Cow::Borrowed(url.path()),
            Self::Query    => Cow::Borrowed(url.query()?),
            Self::Fragment => Cow::Borrowed(url.fragment()?)
        })
    }

    pub fn replace_with(&self, url: &mut Url, with: &str) -> Result<(), PartReplaceError> {
        match self {
            Self::Whole    => *url=Url::parse(with)?,
            Self::Host     => url.set_host(Some(with))?,
            Self::Domain   => url.set_host(Some(with))?,
            Self::Path     => url.set_path(with),
            Self::Query    => url.set_query(Some(with)),
            Self::Fragment => url.set_fragment(Some(with))
        }
        Ok(())
    }
}
