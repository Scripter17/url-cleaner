//! Proxy support for HTTP and HTTPS requests.
//! 
//! Enabled by the `http` feature flag.

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use url::Url;
use reqwest::header::HeaderValue;
use reqwest::Proxy;

use crate::util::is_default;

#[expect(unused_imports, reason = "Used in a doc comment.")]
use crate::glue::HttpClientConfig;

/// Used by [`HttpClientConfig`] to detail how a [`reqwest::Proxy`] should be made.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct ProxyConfig {
    /// The URL to proxy traffic to. Not the URL whose traffic to proxy.
    pub url: Url,
    /// The type of requests to proxy. Defaults to [`ProxyMode::All`] which proxies HTTP and HTTPS requests.
    #[serde(default, skip_serializing_if = "is_default")]
    pub mode: ProxyMode,
    /// Authentication for the proxy server. Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub auth: Option<ProxyAuth>
}

crate::util::string_or_struct_magic!(ProxyConfig);

impl FromStr for ProxyConfig {
    type Err = <Url as FromStr>::Err;

    /// [`Url::from_str`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Url::from_str(s)?.into())
    }
}

impl TryFrom<&str> for ProxyConfig {
    type Error = <Self as FromStr>::Err;

    /// [`Self::from_str`].
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl From<Url> for ProxyConfig {
    /// Creates a [`Self`] with the provided URL, defaulting all other fields.
    fn from(url: Url) -> Self {
        Self {
            url,
            mode: ProxyMode::default(),
            auth: None
        }
    }
}

/// The types of traffic to proxy. Defaults to [`Self::All`].
#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, Serialize, Deserialize)]
pub enum ProxyMode {
    /// [`reqwest::Proxy::all`].
    #[default]
    All,
    /// [`reqwest::Proxy::https`].
    Https,
    /// [`reqwest::Proxy::http`].
    Http
}

/// Authentication for the proxy server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProxyAuth {
    /// [`reqwest::Proxy::basic_auth`].
    Basic {
        /// The username to use.
        username: String,
        /// The password to use.
        password: String
    },
    /// [`reqwest::Proxy::custom_http_auth`].
    Custom(#[serde(with = "crate::glue::headervalue")] HeaderValue)
}

impl TryFrom<ProxyConfig> for reqwest::Proxy {
    type Error = reqwest::Error;

    /// Create a [`reqwest::Proxy`].
    /// # Errors
    /// If `value`'s [`ProxyConfig::auth`] is [`Some`] and the call to [`reqwest::Proxy::all`], [`reqwest::Proxy::https`], or [`reqwest::Proxy::http`] return an error, that error is returned.
    fn try_from(value: ProxyConfig) -> reqwest::Result<Self> {
        let temp = match value.mode {
            ProxyMode::All   => Proxy::all  (value.url),
            ProxyMode::Https => Proxy::https(value.url),
            ProxyMode::Http  => Proxy::http (value.url)
        }?;
        Ok(match &value.auth {
            None => temp,
            Some(ProxyAuth::Basic {username, password}) => temp.basic_auth(username, password),
            Some(ProxyAuth::Custom(value)) => temp.custom_http_auth(value.clone())
        })
    }
}

impl ProxyConfig {
    /// Create a [`reqwest::Proxy`].
    /// # Errors
    /// If `value`'s [`ProxyConfig::auth`] is [`Some`] and the call to [`reqwest::Proxy::all`], [`reqwest::Proxy::https`], or [`reqwest::Proxy::http`] return an error, that error is returned.
    pub fn make(self) -> reqwest::Result<Proxy> {
        self.try_into()
    }
}
