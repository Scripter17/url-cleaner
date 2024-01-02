use std::borrow::Cow;

use url::{Url, ParseError};
use thiserror::Error;
use std::str::FromStr;
use std::io::Error as IoError;

use serde::{
    Serialize,
    ser::Serializer,
    {de::Error as DeError, Deserialize, Deserializer}
};

/// An enum that makes using the various [`Url`] getter simplers.
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
    /// The domain. Corresponds to [`Url::domain`].
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

/// An enum of all possible errors [`UrlPartName::replace_with`] can return.
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
            Self::Host     => Cow::Borrowed(url.host_str()?),
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

/// The method [`crate::rules::conditions::Condition::DomainCondition`] should use.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum DomainConditionRule {
    /// Use the specified URL. If the source of the URL being cleaned is a link on a webpage then this should contain the URL of that webpage.
    #[serde(serialize_with = "serialize_url", deserialize_with = "deserialize_url")]
    Url(Url),
    /// Makes [`crate::rules::conditions::Condition::DomainCondition`] always pass.
    Always,
    /// Makes [`crate::rules::conditions::Condition::DomainCondition`] always fail.
    Never,
    /// Similar to [`DomainConditionRule::Url`] except the contained URL would always be the URL being cleaned.
    /// This is the default as I assusme it's the one that works most of the time.
    #[default]
    UseUrlBeingCleaned
}

fn deserialize_url<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Url, D::Error> {
    let x: &'de str=Deserialize::deserialize(deserializer)?;
    Url::parse(x).map_err(|_| D::Error::custom(format!("Invalid URL pattern: {x:?}.")))
}
fn serialize_url<S: Serializer>(value: &Url, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(value.as_str())
}

impl FromStr for DomainConditionRule {
    type Err=ParseError;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        Ok(match x {
            "Always" => DomainConditionRule::Always,
            "Never" => DomainConditionRule::Never,
            "UseUrlBeingCleaned" => DomainConditionRule::UseUrlBeingCleaned,
            _ => DomainConditionRule::Url(Url::parse(x)?)
        })
    }
}

impl ToString for DomainConditionRule {
    fn to_string(&self) -> String {
        match self {
            Self::Url(url) => url.to_string(),
            Self::Always => "Always".to_string(),
            Self::Never => "Never".to_string(),
            Self::UseUrlBeingCleaned => "UseUrlBeingCleaned".to_string()
        }
    }
}

/// An enum that, if I've done my job properly, contains details on any possible error that can heppen when cleaning a URL.
/// Except for if a [`crate::rules::mappers::Mapper::Expand301`] can't be cached. That error is ignored pending a version of [`Result`] that can handle partial errors.
/// Not only is it a recoverable error, it's an error that doesn't need to be recovered from.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum CleaningError {
    /// There was an error getting the rules.
    #[error("There was an errot getting the rules.")]
    GetRulesError(#[from] crate::rules::GetRulesError),
    /// There was an error executing a rule.
    #[error("There was an error executing a rule.")]
    RuleError(#[from] crate::rules::RuleError),
    /// There was an error parsing the URL.
    #[error("There was an error parsing the URL.")]
    UrlParseError(#[from] ParseError),
    /// IO error.
    #[error("IO error")]
    IoError(#[from] IoError)
}
