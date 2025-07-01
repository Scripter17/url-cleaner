//! Rules for how to make a [`reqwest::blocking::Client`].

use std::collections::HashSet;

use serde::{Serialize, Deserialize};
#[cfg(feature = "http")]
use reqwest::header::HeaderMap;

#[expect(unused_imports, reason = "Used in docs.")]
use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Rules for how to make a [`reqwest::blocking::Client`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct HttpClientConfig {
    /// The headers to send by default.
    #[serde(default, skip_serializing_if = "is_default", with = "serde_headermap")]
    pub default_headers: HeaderMap,
    /// The redirect policy.
    ///
    /// Somewhat nuanced so check [`RedirectPolicy`]'s docs.
    #[serde(default, skip_serializing_if = "is_default")]
    pub redirect_policy: RedirectPolicy,
    /// The value passed to [`reqwest::blocking::ClientBuilder::https_only`].
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub https_only: bool,
    /// The value passed to [`reqwest::blocking::ClientBuilder::referer`].
    ///
    /// Defaults to [`false`] and frankly there's no legitimate reason for the header to exist or for you to turn it on.
    #[serde(default, skip_serializing_if = "is_default")]
    pub referer: bool,
    /// Proxies to use.
    ///
    /// All proxies supported by [`reqwest`] should always be supported, but if I missed anything let me know.
    #[serde(default, skip_serializing_if = "is_default")]
    pub proxies: Vec<ProxyConfig>,
    /// The value passed to [`reqwest::blocking::ClientBuilder::no_proxy`].
    ///
    /// Defaults to [`false`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub no_proxy: bool,
    /// Extra PEM encoded TLS certificates to trust.
    ///
    /// See [`reqwest::blocking::ClientBuilder::add_root_certificate`] and [`reqwest::tls::Certificate::from_pem`] for details.
    ///
    /// Defaults to an empty list.
    #[serde(default, skip_serializing_if = "is_default")]
    pub extra_root_certificates: HashSet<String>
}

/// The policy on how to handle [HTTP redirects](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Redirections).
///
/// Defaults to [`Self::Limited`] with a value of `10`, as that's what reqwest does.
///
/// For the default config (and all real use) it's recommended to use [`Self::None`] in a [`Action::Repeat`].
///
/// That has the added benefit of not sending a request to the final URL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub enum RedirectPolicy {
    /// If a request encounters [`Self::Limited::0`] redirects, the entire request fails.
    ///
    /// So if your policy is `RedirectPolicy::Limited(0)`, any redirects at all will return an error.
    Limited(usize),
    /// Don't follow redirects and instead return the page doing the redirecting.
    None
}

impl Default for RedirectPolicy {
    /// [`Self::Limited`] with a value of `10`, as that's what reqwest does.
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
    /// Makes a [`reqwest::blocking::Client`].
    /// # Errors
    /// If a call to [`ProxyConfig::make`] returns an error, that error is returned.
    ///
    /// If the call to [`reqwest::blocking::ClientBuilder::build`] returns an error, that error is returned.
    pub fn make(&self) -> reqwest::Result<reqwest::blocking::Client> {
        let mut temp = reqwest::blocking::Client::builder().default_headers(self.default_headers.clone())
            .redirect(self.redirect_policy.clone().into())
            .https_only(self.https_only)
            .referer(self.referer);
        for proxy in &self.proxies {
            temp = temp.proxy(proxy.clone().make()?);
        }
        if self.no_proxy {temp = temp.no_proxy();}
        for cert in &self.extra_root_certificates {
            temp = temp.add_root_certificate(reqwest::tls::Certificate::from_pem(cert.as_bytes())?);
        }
        temp.build()
    }
}

/// Rules for updating a [`HttpClientConfig`].
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, Suitability)]
pub struct HttpClientConfigDiff {
    /// If [`Some`], overwrites [`HttpClientConfig::redirect_policy`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub redirect_policy: Option<RedirectPolicy>,
    /// Appends each header into [`HttpClientConfig::default_headers`].
    #[serde(default, skip_serializing_if = "is_default", with = "serde_headermap")]
    pub add_default_headers: HeaderMap,
    /// If [`Some`], overwrites [`HttpClientConfig::https_only`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub https_only: Option<bool>,
    /// If [`Some`], overwrites [`HttpClientConfig::proxies`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub set_proxies: Option<Vec<ProxyConfig>>,
    /// Appends each [`ProxyConfig`] to [`HttpClientConfig::proxies`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub add_proxies: Vec<ProxyConfig>,
    /// If [`Some`], overwrites [`HttpClientConfig::no_proxy`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub no_proxy: Option<bool>,
    /// If [`Some`], overwrites [`HttpClientConfig::referer`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub referer: Option<bool>,
    /// Adds to [`HttpClientConfig::extra_root_certificates`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub add_extra_root_certificates: HashSet<String>,
    /// Removes from [`HttpClientConfig::extra_root_certificates`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub remove_extra_root_certificates: HashSet<String>
}

impl HttpClientConfigDiff {
    /// Applies the diff.
    pub fn apply(&self, to: &mut HttpClientConfig) {
        if let Some(new_redirect_policy) = &self.redirect_policy {to.redirect_policy = new_redirect_policy.clone();}
        to.default_headers.extend(self.add_default_headers.clone());
        if let Some(https_only) = self.https_only {to.https_only = https_only;}
        if let Some(set_proxies) = &self.set_proxies {to.proxies.clone_from(set_proxies);}
        to.proxies.extend(self.add_proxies.clone());
        if let Some(no_proxy) = self.no_proxy {to.no_proxy = no_proxy;}
        if let Some(referer) = self.referer {to.no_proxy = referer;}
        to.extra_root_certificates.extend(self.add_extra_root_certificates.clone());
        to.extra_root_certificates.retain(|extra_root_certificate| !self.remove_extra_root_certificates.contains(extra_root_certificate));
    }
}
