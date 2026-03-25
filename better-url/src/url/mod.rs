//! [`BetterUrl`].

use std::str::FromStr;
use std::ops::Deref;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, ser::Serializer, de::Deserializer};
use url::Url;

use crate::prelude::*;

mod host;
mod path;
mod query;
mod fragment;

/// A wrapper around a [`Url`] with extra metadata.
///
/// Currently the only included metadata is a [`HostDetails`], which currently only caches [PSL](https://publicsuffix.org/) information.
#[derive(Debug, Clone)]
pub struct BetterUrl {
    /// The [`Url`].
    url: Url,
    /// The [`HostDetails`] of [`Self::url`].
    host_details: Option<HostDetails>
}

impl BetterUrl {
    /// Parse a URL.
    /// # Errors
    /// If the call to [`Url::parse`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    /// let url = BetterUrl::parse("https://example.com").unwrap();
    /// ```
    pub fn parse(value: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::from_str(value)
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

    /// [`Url::set_port`].
    /// # Errors
    /// If the call to [`Url::set_port`] returns an error, returns the error [`SetPortError`].
    pub fn set_port(&mut self, port: Option<u16>) -> Result<(), SetPortError> {
        self.url.set_port(port).map_err(|()| SetPortError)
    }
}

impl std::fmt::Display for BetterUrl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.url)
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
