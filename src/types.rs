use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use url::{Url, ParseError};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum UrlPartName {
    /// The whole URL. Corresponds to [`Url::as_str`].
    Whole,
    /// The scheme. Corresponds to [`Url::scheme`].
    Scheme,
    /// The username. Corresponds to [`Url::username`].
    Username,
    /// The password. Corresponds to [`Url::password`].
    Password,
    /// The host. Either a domain name or IPV4/6 address. Corresponds to [`Url::host`].
    Host,
    /// The domain.. Corresponds to [`Url::domain`].
    Domain,
    /// The port as a string. Correspods to [`Url::port`].
    /// Ports are always treated as strings for the sake of a simpler API.
    Port,
    /// The path. Corresponds to [`Url::path`].
    Path,
    /// The query. Corresponds to [`Url::query`].
    Query,
    /// The fragment. Corresponds to [`Url::fragment`].
    Fragment
}

#[derive(Debug, Error)]
pub enum ReplaceError {
    /// Attempted replacement would not produce a valid URL.
    #[error("Attempted replacement would not produce a valid URL.")]
    ParseError(#[from] ParseError),
    /// The provided scheme would not have produced a valid URL.
    #[error("The provided scheme would not have produced a valid URL.")]
    InvalidScheme,
    /// The provided port is not a number.
    #[error("The provided port is not a number.")]
    InvalidPort,
    /// Cannot set port for this URL. Either becasue it is cannot-be-a-base, does not have a host, or has the file scheme.
    #[error("Cannot set port for this URL. Either becasue it is cannot-be-a-base, does not have a host, or has the file scheme.")]
    CannotSetPort,
    /// Cannot set username for this URL. Either because it is cannot-be-a-base or does not have a host.
    #[error("Cannot set username for this URL. Either because it is cannot-be-a-base or does not have a host.")]
    CannotSetUsername,
    /// Cannot set password for this URL. Either because it is cannot-be-a-base or does not have a host.
    #[error("Cannot set password for this URL. Either because it is cannot-be-a-base or does not have a host.")]
    CannotSetPassword
}

impl UrlPartName {
    /// Extracts the specified part of the provided URL
    pub fn get_from<'a>(&self, url: &'a Url) -> Option<Cow<'a, str>> {
        Some(match self {
            Self::Whole    => Cow::Borrowed(url.as_str()),
            Self::Scheme   => Cow::Borrowed(url.scheme()),
            Self::Username => Cow::Borrowed(url.username()),
            Self::Password => Cow::Borrowed(url.password()?),
            Self::Host     => Cow::Owned   (url.host()?.to_string()), // IPV4/6 hosts need to be converted into Strings
            Self::Domain   => Cow::Borrowed(url.domain()?),
            Self::Port     => Cow::Owned   (url.port()?.to_string()), // I cannot be bothered to add number handling
            Self::Path     => Cow::Borrowed(url.path()),
            Self::Query    => Cow::Borrowed(url.query()?),
            Self::Fragment => Cow::Borrowed(url.fragment()?)
        })
    }

    /// Replaces the specified part of the provided URL with the provided value
    pub fn replace_with(&self, url: &mut Url, with: &str) -> Result<(), ReplaceError> {
        match self {
            Self::Whole    => *url=Url::parse(with)?,
            Self::Scheme   => url.set_scheme(with).map_err(|_| ReplaceError::InvalidScheme)?,
            Self::Username => url.set_username(with).map_err(|_| ReplaceError::CannotSetUsername)?,
            Self::Password => url.set_password(Some(with)).map_err(|_| ReplaceError::CannotSetPassword)?,
            Self::Host     => url.set_host(Some(with))?,
            Self::Domain   => url.set_host(Some(with))?,
            Self::Port     => url.set_port(Some(with.parse().map_err(|_| ReplaceError::InvalidPort)?)).map_err(|_| ReplaceError::CannotSetPort)?,
            Self::Path     => url.set_path(with),
            Self::Query    => url.set_query(Some(with)),
            Self::Fragment => url.set_fragment(Some(with))
        }
        Ok(())
    }
}
