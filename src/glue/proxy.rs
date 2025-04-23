//! Glue for [`reqwest::Proxy`].

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use url::Url;
use reqwest::header::HeaderValue;
use reqwest::Proxy;

use crate::glue::*;
use crate::util::*;

#[expect(unused_imports, reason = "Used in a doc comment.")]
use crate::glue::HttpClientConfig;

/// Rules on how to make a [`reqwest::Proxy`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(remote = "Self")]
pub struct ProxyConfig {
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

crate::util::string_or_struct_magic!(ProxyConfig);

impl FromStr for ProxyConfig {
    type Err = <Url as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Url::from_str(s)?.into())
    }
}

impl TryFrom<&str> for ProxyConfig {
    type Error = <Self as FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl From<Url> for ProxyConfig {
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
    Custom(#[serde(with = "serde_headervalue")] HeaderValue)
}

impl TryFrom<ProxyConfig> for reqwest::Proxy {
    type Error = reqwest::Error;

    fn try_from(value: ProxyConfig) -> reqwest::Result<Self> {
        let temp = match value.mode {
            ProxyMode::Http  => Proxy::http (value.url),
            ProxyMode::Https => Proxy::https(value.url),
            ProxyMode::All   => Proxy::all  (value.url)
        }?;
        Ok(match &value.auth {
            None => temp,
            Some(ProxyAuth::Basic {username, password}) => temp.basic_auth(username, password),
            Some(ProxyAuth::Custom(value)) => temp.custom_http_auth(value.clone())
        })
    }
}

impl ProxyConfig {
    /// Makes a [`reqwest::Proxy`].
    /// # Errors
    /// If the call to [`Proxy::http`], [`Proxy::https`], or [`Proxy::all`] return an error, that error is returned.
    pub fn make(self) -> reqwest::Result<Proxy> {
        self.try_into()
    }
}
