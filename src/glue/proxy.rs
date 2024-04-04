//! Proxy support for HTTP and HTTPS requests.

use serde::{Serialize, Deserialize};
use url::Url;
use reqwest::header::HeaderValue;
use reqwest::Proxy;

// Used for doc links.
#[allow(unused_imports)]
use crate::types::HttpClientConfig;

/// Used by [`HttpClientConfig`] to detail how a [`reqwest::Proxy`] should be made.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// The URL to proxy traffic to. Not the URL whose traffic to proxy.
    pub url: Url,
    /// The type of requests to proxy. Defaults to [`ProxyMode::All`] which proxies HTTP and HTTPS requests.
    #[serde(default)]
    pub mode: ProxyMode,
    /// Authentication for the proxy server. Defaults to [`None`].
    #[serde(default)]
    pub auth: Option<ProxyAuth>
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

impl ProxyConfig {
    /// Create a [`reqwest::Proxy`].
    /// # Errors
    /// If the call to [`reqwest::Proxy::all`], [`reqwest::Proxy::https`], or [`reqwest::Proxy::http`] return an error, that error is returned.
    pub fn make(&self) -> reqwest::Result<reqwest::Proxy> {
        let temp = match self.mode {
            ProxyMode::All   => Proxy::all  (self.url.clone()),
            ProxyMode::Https => Proxy::https(self.url.clone()),
            ProxyMode::Http  => Proxy::http (self.url.clone())
        }?;
        Ok(match &self.auth {
            None => temp,
            Some(ProxyAuth::Basic {username, password}) => temp.basic_auth(username, password),
            Some(ProxyAuth::Custom(value)) => temp.custom_http_auth(value.clone())
        })
    }
}
