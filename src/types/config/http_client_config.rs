//! Allows configuring HTTP clients.

use serde::{Serialize, Deserialize};
#[cfg(feature = "http")]
use reqwest::header::HeaderMap;

// Used for doc links.
#[allow(unused_imports)]
use crate::types::*;
use crate::glue::*;
use crate::util::is_default;

/// Used by [`Params`] to detail how a [`reqwest::blocking::Client`] should be made.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpClientConfig {
    /// [`reqwest::blocking::ClientBuilder::default_headers`]. Defaults to an empty [`HeaderMap`].
    #[serde(default, skip_serializing_if = "is_default", with = "crate::glue::headermap")]
    pub default_headers: HeaderMap,
    /// Roughly corresponds to [`reqwest::redirect::Policy`]. Defaults to [`RedirectPolicy::default`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub redirect_policy: RedirectPolicy,
    /// [`reqwest::blocking::ClientBuilder::https_only`]. Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub https_only: bool,
    /// [`reqwest::blocking::ClientBuilder::proxy`]. Defaults to an empty [`Vec`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub proxies: Vec<ProxyConfig>,
    /// [`reqwest::blocking::ClientBuilder::no_proxy`]. Applied after and therefore overrides [`Self::proxies`]. Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub no_proxy: bool,
    /// [`reqwest::blocking::ClientBuilder::referer`]. Defaults to [`false`]
    #[serde(default, skip_serializing_if = "is_default")]
    pub referer: bool,
    /// [`reqwest::blocking::ClientBuilder::danger_accept_invalid_certs`]. Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub danger_accept_invalid_certs: bool
}

/// Bandaid fix until [`reqwest::redirect::Policy`] stops sucking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RedirectPolicy {
    /// [`reqwest::redirect::Policy::limited`].
    Limited(usize),
    /// [`reqwest::redirect::Policy::none`].
    None
}

impl Default for RedirectPolicy {
    /// Defaults to `Self::Limited(10)` because that's what reqwest does.
    fn default() -> Self {
        Self::Limited(10)
    }
}

impl From<RedirectPolicy> for reqwest::redirect::Policy {
    fn from(value: RedirectPolicy) -> Self {
        match value {
            RedirectPolicy::Limited(x) => Self::limited(x),
            RedirectPolicy::None => Self::none()
        }
    }
}

impl HttpClientConfig {
    /// Unfortunately has to consume `client` due to [`reqwest::blocking::ClientBuilder`]'s API sucking.
    /// # Errors
    /// If the call to [`ProxyConfig::make`] returns an error, that error is returned.
    pub fn apply(&self, client: reqwest::blocking::ClientBuilder) -> reqwest::Result<reqwest::blocking::ClientBuilder> {
        let mut temp = client.default_headers(self.default_headers.clone())
            .redirect(self.redirect_policy.clone().into())
            .https_only(self.https_only)
            .referer(self.referer)
            .danger_accept_invalid_certs(self.danger_accept_invalid_certs);
        for proxy in &self.proxies {
            temp = temp.proxy(proxy.make()?);
        }
        if self.no_proxy {temp = temp.no_proxy();}
        Ok(temp)
    }
}

/// Allows changing [`HttpClientConfig`].
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HttpClientConfigDiff {
    /// If [`Some`], overwrites [`HttpClientConfig::redirect_policy`]. Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub redirect_policy: Option<RedirectPolicy>,
    /// Appends headers to [`HttpClientConfig::default_headers`]. Defaults to an empty [`HeaderMap`].
    #[serde(default, skip_serializing_if = "is_default", with = "crate::glue::headermap")]
    pub add_default_headers: HeaderMap,
    /// If [`Some`], overwrites [`HttpClientConfig::https_only`]. Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub https_only: Option<bool>,
    /// If [`Some`], overwrites [`HttpClientConfig::proxies`]. Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub set_proxies: Option<Vec<ProxyConfig>>,
    /// Appends proxies to [`HttpClientConfig::proxies`] after handling [`Self::set_proxies`]. Defaults to an empty [`Vec`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub add_proxies: Vec<ProxyConfig>,
    /// If [`Some`], overwrites [`HttpClientConfig::no_proxy`]. Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub no_proxy: Option<bool>,
    /// If [`Some`], overwrites [`HttpClientConfig::referer`]. Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub referer: Option<bool>,
    /// IF [`Some`], overwrites [`HttpClientConfig::danger_accept_invalid_certs`]. Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub danger_accept_invalid_certs: Option<bool>
}

impl HttpClientConfigDiff {
    /// Applies the differences specified in `self` to `to` in the following order:
    /// 1. If [`Self::redirect_policy`] is [`Some`], overwrite `to`'s [`HttpClientConfig::redirect_policy`].
    /// 2. Append [`Self::add_default_headers`] to `to`'s [`HttpClientConfig::default_headers`].
    /// 3. If [`Self::https_only`] is [`Some`], overwrite `to`'s [`HttpClientConfig::https_only`].
    /// 4. If [`Self::set_proxies`] is [`Some`], overwrite `to`'s [`HttpClientConfig::proxies`].
    /// 5. Append [`Self::add_proxies`] to `to`'s [`HttpClientConfig::proxies`].
    /// 6. If [`Self::no_proxy`] is [`Some`], overwrite `to`'s [`HttpClientConfig::no_proxy`].
    /// 7. If [`Self::referer`] is [`Some`], overwrite `to`'s [`HttpClientConfig::referer`].
    /// 8. If [`Self::danger_accept_invalid_certs`] is [`Some`], overwrite `to`'s [`HttpClientConfig::danger_accept_invalid_certs`].
    pub fn apply(&self, to: &mut HttpClientConfig) {
        if let Some(new_redirect_policy) = &self.redirect_policy {to.redirect_policy = new_redirect_policy.clone();}
        to.default_headers.extend(self.add_default_headers.clone());
        if let Some(https_only) = self.https_only {to.https_only = https_only;}
        if let Some(set_proxies) = &self.set_proxies {to.proxies.clone_from(set_proxies);}
        to.proxies.extend(self.add_proxies.clone());
        if let Some(no_proxy) = self.no_proxy {to.no_proxy = no_proxy;}
        if let Some(referer) = self.referer {to.no_proxy = referer;}
        if let Some(danger_accept_invalid_certs) = self.danger_accept_invalid_certs {to.danger_accept_invalid_certs = danger_accept_invalid_certs;}
    }
}
