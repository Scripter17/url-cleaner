//! [`HttpProxyConfig`].

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use url::Url;
#[expect(unused_imports, reason = "Used in doc comment.")]
use reqwest::header::HeaderValue;
use reqwest::Proxy;

use crate::prelude::*;

/// Rules on how to make a [`reqwest::Proxy`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct HttpProxyConfig {
    /// The [`Url`] to proxy requests to.
    pub url: Url,
    /// The protocol(s) to redirect.
    ///
    /// Defaults to [`ProxyMode::All`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub mode: ProxyMode,
    /// The authentication to use.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub auth: Option<ProxyAuth>
}

crate::util::string_or_struct_magic!(HttpProxyConfig);

impl FromStr for HttpProxyConfig {
    type Err = <Url as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Url::from_str(s)?.into())
    }
}

impl TryFrom<&str> for HttpProxyConfig {
    type Error = <Self as FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl From<Url> for HttpProxyConfig {
    fn from(url: Url) -> Self {
        Self {
            url,
            mode: ProxyMode::default(),
            auth: None
        }
    }
}

/// The protocol(s) to proxy.
///
/// Defaults to [`Self::All`].
#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum ProxyMode {
    /// Proxy only HTTP.
    ///
    /// Corresponds to [`Proxy::http`].
    Http,
    /// Proxy only HTTPS.
    ///
    /// Corresponds to [`Proxy::https`].
    Https,
    /// Proxy all protocols.
    ///
    /// Corresponds to [`Proxy::all`].
    #[default]
    All
}

/// The authentication to use for a proxy.
///
/// Uses the [`Proxy-Authentication`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Proxy-Authorization) HTTP header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum ProxyAuth {
    /// Uses the [`Basic`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Authentication#basic_authentication_scheme) mode.
    ///
    /// Corresponds to [`Proxy::basic_auth`].
    Basic {
        /// The username to use.
        username: String,
        /// The password to use.
        password: String
    },
    /// Uses a custom value.
    ///
    /// Corresponds to [`Proxy::custom_http_auth`].
    Custom(String)
}

/// The enum of errors [`HttpProxyConfig::make`] can return.
#[derive(Debug, Error)]
pub enum MakeHttpProxyError {
    /// Returned when a [`reqwest::Error`] is encountered.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Returned when a [`reqwest::header::InvalidHeaderName`] is encountered.
    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue)
}

impl TryFrom<HttpProxyConfig> for reqwest::Proxy {
    type Error = MakeHttpProxyError;

    /// [`HttpProxyConfig::make`].
    fn try_from(value: HttpProxyConfig) -> Result<Self, Self::Error> {
        value.make()
    }
}

impl HttpProxyConfig {
    /// Makes a [`reqwest::Proxy`].
    /// # Errors
    #[doc = edoc!(callerr(Proxy::http), callerr(Proxy::https), callerr(Proxy::all))]
    ///
    /// If converting [`ProxyAuth::Custom`]'s [`String`] to a [`HeaderValue`] returns an error, that error is returned.
    pub fn make(self) -> Result<Proxy, MakeHttpProxyError> {
        let temp = match self.mode {
            ProxyMode::Http  => Proxy::http (self.url),
            ProxyMode::Https => Proxy::https(self.url),
            ProxyMode::All   => Proxy::all  (self.url)
        }?;
        Ok(match self.auth {
            None => temp,
            Some(ProxyAuth::Basic {username, password}) => temp.basic_auth(&username, &password),
            Some(ProxyAuth::Custom(value)) => temp.custom_http_auth(value.try_into()?)
        })
    }
}
