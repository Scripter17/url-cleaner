//! A wrapper around [`url::Url`] with extra metadata.

use std::net::IpAddr;
use std::str::FromStr;
use std::ops::Deref;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, ser::Serializer, de::Deserializer};
use url::{Url, PathSegmentsMut, ParseError};
use thiserror::Error;

use crate::*;

mod path_impl;
pub use path_impl::*;
mod domain_impl;
pub use domain_impl::*;
mod query_impl;
pub use query_impl::*;

/// A wrapper around a [`Url`] with extra metadata.
///
/// Currently the only included metadata is a [`HostDetails`], which currently only caches [PSL](https://publicsuffix.org/) information for more efficient reg domain, domain suffix, etc..
#[derive(Clone)]
pub struct BetterUrl {
    /// The [`Url`].
    url: Url,
    /// The [`HostDetails`] of [`Self::url`].
    host_details: Option<HostDetails>
}

/// The error [`BetterUrl::set_port`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the port.")]
pub struct SetPortError;

/// The error [`BetterUrl::set_ip_host`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the host to an IP.")]
pub struct SetIpHostError;

/// The error [`BetterUrl::set_password`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the password.")]
pub struct SetPasswordError;

/// The error [`BetterUrl::set_username`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the username.")]
pub struct SetUsernameError;

/// The error [`BetterUrl::set_scheme`] returns when it fails.
#[derive(Debug, Error)]
#[error("Failed to set the scheme.")]
pub struct SetSchemeError;

/// The error [`BetterUrl::set_host`] returns when it fails.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct SetHostError(#[from] pub ParseError);

impl BetterUrl {
    /// Parse a URL.
    /// # Errors
    /// If the call to [`Url::parse`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use better_url::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    /// ```
    pub fn parse(value: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::from_str(value)
    }

    /// Get a borrowing [`BetterHost`].
    pub fn host(&self) -> Option<BetterHost<&str>> {
        match (self.host_str(), self.host_details()) {
            (Some(string), Some(details)) => Some(BetterHost {string, details}),
            _ => None
        }
    }

    /// Get the contained [`HostDetails`].
    /// # Examples
    /// ```
    /// use better_url::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert!(matches!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(0), suffix_start: Some(8), fqdn_period: None}))));
    ///
    /// let url = BetterUrl::parse("https://127.0.0.1").unwrap();
    ///
    /// assert!(matches!(url.host_details(), Some(HostDetails::Ipv4(Ipv4Details {parsed: _}))));
    ///
    /// let url = BetterUrl::parse("https://[::1]").unwrap();
    ///
    /// assert!(matches!(url.host_details(), Some(HostDetails::Ipv6(Ipv6Details {parsed: _}))));
    /// ```
    pub fn host_details(&self) -> Option<HostDetails> {
        self.host_details
    }

    /// If [`Self::host_details`] returns [`HostDetails::Domain`], return it.
    /// ```
    /// use better_url::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert!(matches!(url.domain_details(), Some(DomainDetails {middle_start: Some(0), suffix_start: Some(8), fqdn_period: None})));
    /// assert!(matches!(url.ipv4_details  (), None));
    /// assert!(matches!(url.ipv6_details  (), None));
    /// ```
    pub fn domain_details(&self) -> Option<DomainDetails> {
        self.host_details()?.domain_details()
    }

    /// If [`Self::host_details`] is [`HostDetails::Ipv4`] or [`HostDetails::Ipv6`], reutrn the equivalent [`IpDetails`].
    pub fn ip_details(&self) -> Option<IpDetails> {
        self.host_details()?.ip_details()
    }

    /// If [`Self::host_details`] returns [`HostDetails::Ipv4`], return it.
    /// ```
    /// use better_url::*;
    /// let url = BetterUrl::parse("https://127.0.0.1").unwrap();
    ///
    /// assert!(matches!(url.domain_details(), None));
    /// assert!(matches!(url.ipv4_details  (), Some(Ipv4Details {parsed: _})));
    /// assert!(matches!(url.ipv6_details  (), None));
    /// ```
    pub fn ipv4_details(&self) -> Option<Ipv4Details> {
        self.host_details()?.ipv4_details()
    }

    /// If [`Self::host_details`] returns [`HostDetails::Ipv6`], return it.
    /// ```
    /// use better_url::*;
    /// let url = BetterUrl::parse("https://[::1]").unwrap();
    ///
    /// assert!(matches!(url.domain_details(), None));
    /// assert!(matches!(url.ipv4_details  (), None));
    /// assert!(matches!(url.ipv6_details  (), Some(Ipv6Details {parsed: _})));
    /// ```
    pub fn ipv6_details(&self) -> Option<Ipv6Details> {
        self.host_details()?.ipv6_details()
    }

    /// [`Url::host_str`] with any `www.` prefix and `.` suffix removed.
    pub fn normalized_host(&self) -> Option<&str> {
        let x = self.host_str()?;
        let x = x.strip_prefix("www.").unwrap_or(x);
        Some(x.strip_suffix(".").unwrap_or(x))
    }

    /// [`Url::set_scheme`].
    /// # Errors
    /// If the call to [`Url::set_scheme`] returns an error, returns the error [`SetSchemeError`].
    pub fn set_scheme(&mut self, scheme: &str) -> Result<(), SetSchemeError> {
        self.url.set_scheme(scheme).map_err(|()| SetSchemeError)
    }

    /// [`Url::set_username`].
    /// # Errors
    /// If the call to [`Url::set_username`] returns an error, returns the error [`SetUsernameError`].
    pub fn set_username(&mut self, username: &str) -> Result<(), SetUsernameError> {
        self.url.set_username(username).map_err(|()| SetUsernameError)
    }

    /// [`Url::set_password`].
    /// # Errors
    /// If the call to [`Url::set_password`] returns an error, returns the error [`SetPasswordError`].
    pub fn set_password(&mut self, password: Option<&str>) -> Result<(), SetPasswordError> {
        self.url.set_password(password).map_err(|()| SetPasswordError)
    }

    /// [`Self::set_host`] but use a precomputed [`HostDetails`].
    fn set_host_with_known_details(&mut self, host: Option<&str>, host_details: Option<HostDetails>) -> Result<(), SetHostError> {
        self.url.set_host(host)?;
        self.host_details = host_details;
        Ok(())
    }

    /// [`Url::set_host`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, the error is returned..
    pub fn set_host(&mut self, host: Option<&str>) -> Result<(), SetHostError> {
        self.url.set_host(host)?;
        self.host_details = self.url.host().map(|host| HostDetails::from_host(&host));
        Ok(())
    }

    /// [`Url::set_ip_host`].
    /// # Errors
    /// If the call to [`Url::set_ip_host`] returns an error, returns the error [`SetIpHostError`].
    pub fn set_ip_host(&mut self, address: IpAddr) -> Result<(), SetIpHostError> {
        self.url.set_ip_host(address).map_err(|()| SetIpHostError)?;
        self.host_details = Some(address.into());
        Ok(())
    }

    /// [`Url::set_port`].
    /// # Errors
    /// If the call to [`Url::set_port`] returns an error, returns the error [`SetPortError`].
    pub fn set_port(&mut self, port: Option<u16>) -> Result<(), SetPortError> {
        self.url.set_port(port).map_err(|()| SetPortError)
    }

    /// [`Url::set_fragment`].
    pub fn set_fragment(&mut self, fragment: Option<&str>) {
        self.url.set_fragment(fragment)
    }
}

impl std::fmt::Debug for BetterUrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl std::fmt::Display for BetterUrl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.url.fmt(formatter)
    }
}

impl Deref for BetterUrl {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.url
    }
}


impl PartialEq for BetterUrl {fn eq(&self, other: &Self) -> bool {self.url == other.url}}
impl Eq for BetterUrl {}

impl PartialEq<Url      > for BetterUrl {fn eq(&self, other: &Url      ) -> bool {&**self          ==    other}}
impl PartialEq<String   > for BetterUrl {fn eq(&self, other: &String   ) -> bool {   self          == &**other}}
impl PartialEq<str      > for BetterUrl {fn eq(&self, other: &str      ) -> bool {   self.as_str() ==    other}}
impl PartialEq<&str     > for BetterUrl {fn eq(&self, other: &&str     ) -> bool {   self          ==   *other}}

impl PartialEq<BetterUrl> for Url       {fn eq(&self, other: &BetterUrl) -> bool {other == self}}
impl PartialEq<BetterUrl> for String    {fn eq(&self, other: &BetterUrl) -> bool {other == self}}
impl PartialEq<BetterUrl> for str       {fn eq(&self, other: &BetterUrl) -> bool {other == self}}
impl PartialEq<BetterUrl> for &str      {fn eq(&self, other: &BetterUrl) -> bool {other == self}}

impl std::hash::Hash for BetterUrl {
    /// Hashes the same as [`Url`].
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.url, state)
    }
}

impl PartialOrd for BetterUrl {
    /// Ordered the same as [`Url`].
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BetterUrl {
    /// Ordered the same as [`Url`].
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.url.cmp(&other.url)
    }
}

impl std::convert::AsRef<Url> for BetterUrl {
    fn as_ref(&self) -> &Url {
        &self.url
    }
}

impl std::convert::AsRef<str> for BetterUrl {
    fn as_ref(&self) -> &str {
        self.url.as_ref()
    }
}

impl FromStr for BetterUrl {
    type Err = <Url as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Url::from_str(s).map(Into::into)
    }
}

impl TryFrom<&str> for BetterUrl {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl From<Url> for BetterUrl {
    fn from(value: Url) -> Self {
        Self {
            host_details: HostDetails::from_url(&value),
            url: value
        }
    }
}

impl From<BetterUrl> for Url {
    fn from(value: BetterUrl) -> Self {
        value.url
    }
}

impl From<BetterUrl> for String {
    fn from(value: BetterUrl) -> Self {
        value.url.into()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BetterUrl {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Url::deserialize(deserializer).map(Into::into)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterUrl {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
