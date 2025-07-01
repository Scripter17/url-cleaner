//! A wrapper around [`url::Url`] with extra metadata.

use std::net::IpAddr;
use std::str::{FromStr, Split};
use std::ops::Deref;
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use url::{Url, UrlQuery, PathSegmentsMut, ParseError};
use form_urlencoded::Serializer;
use thiserror::Error;

pub mod host_details;
pub use host_details::*;

#[expect(unused_imports, reason = "Used in docs.")]
use crate::types::*;
use crate::util::*;

mod path_impl;
pub use path_impl::*;
mod domain_impl;
pub use domain_impl::*;
mod query_impl;
pub use query_impl::*;

/// A wrapper around a [`Url`] with extra metadata.
///
/// Currently the only included metadata is a [`HostDetails`], which currently only caches [PSL](https://publicsuffix.org/) information for more efficient [`UrlPart::RegDomain`], [`UrlPart::DomainSuffix`], etc..
#[derive(Clone, Serialize, Deserialize)]
#[serde(from = "Url", into = "Url")]
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

/// The error returned by [`BetterUrl::path_segments`] and [`BetterUrl::path_segments_mut`] return when the [`BetterUrl`]'s path doesn't have segments.
#[derive(Debug, Error)]
#[error("The URL does not have path segments.")]
pub struct UrlDoesNotHavePathSegments;

impl BetterUrl {
    /// Parse a URL.
    /// # Errors
    #[doc = edoc!(callerr(Url::parse))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    /// ```
    pub fn parse(value: &str) -> Result<Self, <Self as FromStr>::Err> {
        debug!(BetterUrl::parse, &(), value);
        Self::from_str(value)
    }

    /// Get the contained [`HostDetails`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(url.host_details(), Some(&HostDetails::Domain(DomainDetails {middle_start: Some(0), suffix_start: Some(8), fqdn_period: None})));
    ///
    /// let url = BetterUrl::parse("https://127.0.0.1").unwrap();
    ///
    /// assert_eq!(url.host_details(), Some(&HostDetails::Ipv4(Ipv4Details {})));
    ///
    /// let url = BetterUrl::parse("https://[::1]").unwrap();
    ///
    /// assert_eq!(url.host_details(), Some(&HostDetails::Ipv6(Ipv6Details {})));
    /// ```
    pub fn host_details(&self) -> Option<&HostDetails> {
        self.host_details.as_ref()
    }

    /// If [`Self::host_details`] returns [`HostDetails::Domain`], return it.
    /// ```
    /// use url_cleaner_engine::types::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// assert_eq!(url.domain_details(), Some(&DomainDetails {middle_start: Some(0), suffix_start: Some(8), fqdn_period: None}));
    /// assert_eq!(url.ipv4_details  (), None);
    /// assert_eq!(url.ipv6_details  (), None);
    /// ```
    pub fn domain_details(&self) -> Option<&DomainDetails> {
        self.host_details()?.domain_details()
    }

    /// If [`Self::host_details`] returns [`HostDetails::Ipv4`], return it.
    /// ```
    /// use url_cleaner_engine::types::*;
    /// let url = BetterUrl::parse("https://127.0.0.1").unwrap();
    ///
    /// assert_eq!(url.domain_details(), None);
    /// assert_eq!(url.ipv4_details  (), Some(&Ipv4Details {}));
    /// assert_eq!(url.ipv6_details  (), None);
    /// ```
    pub fn ipv4_details(&self) -> Option<&Ipv4Details> {
        self.host_details()?.ipv4_details()
    }

    /// If [`Self::host_details`] returns [`HostDetails::Ipv6`], return it.
    /// ```
    /// use url_cleaner_engine::types::*;
    /// let url = BetterUrl::parse("https://[::1]").unwrap();
    ///
    /// assert_eq!(url.domain_details(), None);
    /// assert_eq!(url.ipv4_details  (), None);
    /// assert_eq!(url.ipv6_details  (), Some(&Ipv6Details {}));
    /// ```
    pub fn ipv6_details(&self) -> Option<&Ipv6Details> {
        self.host_details()?.ipv6_details()
    }

    /// [`Url::host_str`] with any `www.` prefix and `.` suffix removed.
    pub fn normalized_host(&self) -> Option<&str> {
        let x = self.host_str()?;
        let x = x.strip_prefix("www.").unwrap_or(x);
        Some(x.strip_suffix(".").unwrap_or(x))
    }

    /// Sets the [`UrlPart::Scheme`].
    /// # Errors
    /// If the call to [`Url::set_scheme`] returns an error, returns the error [`SetSchemeError`].
    pub fn set_scheme(&mut self, scheme: &str) -> Result<(), SetSchemeError> {
        debug!(BetterUrl::set_scheme, self, scheme);
        self.url.set_scheme(scheme).map_err(|()| SetSchemeError)
    }

    /// Sets the [`UrlPart::Username`].
    /// # Errors
    /// If the call to [`Url::set_username`] returns an error, returns the error [`SetUsernameError`].
    pub fn set_username(&mut self, username: &str) -> Result<(), SetUsernameError> {
        debug!(BetterUrl::set_username, self, username);
        self.url.set_username(username).map_err(|()| SetUsernameError)
    }

    /// Sets the [`UrlPart::Password`].
    /// # Errors
    /// If the call to [`Url::set_password`] returns an error, returns the error [`SetPasswordError`].
    pub fn set_password(&mut self, password: Option<&str>) -> Result<(), SetPasswordError> {
        debug!(BetterUrl::set_password, self , password);
        self.url.set_password(password).map_err(|()| SetPasswordError)
    }

    /// [`Self::set_host`] but use a precomputed [`HostDetails`].
    fn set_host_with_known_details(&mut self, host: Option<&str>, host_details: Option<HostDetails>) -> Result<(), SetHostError> {
        debug!(BetterUrl::set_host, self, host);
        self.url.set_host(host)?;
        self.host_details = host_details;
        Ok(())
    }

    /// Sets the [`UrlPart::Host`].
    /// # Errors
    /// If the call to [`Url::set_host`] returns an error, the error is returned..
    pub fn set_host(&mut self, host: Option<&str>) -> Result<(), SetHostError> {
        debug!(BetterUrl::set_host, self, host);
        self.url.set_host(host)?;
        self.host_details = self.url.host().map(|host| HostDetails::from_host(&host));
        Ok(())
    }

    /// Sets the [`UrlPart::Host`].
    /// # Errors
    /// If the call to [`Url::set_ip_host`] returns an error, returns the error [`SetIpHostError`].
    pub fn set_ip_host(&mut self, address: IpAddr) -> Result<(), SetIpHostError> {
        debug!(BetterUrl::set_ip_host, self, address);
        self.url.set_ip_host(address).map_err(|()| SetIpHostError)?;
        self.host_details = Some(HostDetails::from_ip_addr(address));
        Ok(())
    }

    /// Sets the [`UrlPart::Port`].
    /// # Errors
    /// If the call to [`Url::set_port`] returns an error, returns the error [`SetPortError`].
    pub fn set_port(&mut self, port: Option<u16>) -> Result<(), SetPortError> {
        debug!(BetterUrl::set_port, self, port);
        self.url.set_port(port).map_err(|()| SetPortError)
    }

    /// [`Url::set_fragment`].
    pub fn set_fragment(&mut self, fragment: Option<&str>) {
        debug!(BetterUrl::set_fragment, self, fragment);
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
