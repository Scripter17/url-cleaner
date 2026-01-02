//! Like [`url::Host`] but better.

use std::str::FromStr;
use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Error}};

use crate::prelude::*;

/// A URL host and its details.
#[derive(Debug, Clone)]
pub struct BetterHost<T: AsRef<str>> {
    /// The host string.
    pub(crate) string: T,
    /// The [`HostDetails`].
    pub(crate) details: HostDetails
}

impl<T: AsRef<str>> BetterHost<T> {
    /// The host string.
    pub fn as_str(&self) -> &str {
        self.string.as_ref()
    }

    /// The host's [`HostDetails`].
    pub fn details(&self) -> HostDetails {
        self.details
    }

    /// The [`Self::details`]'s [`HostDetails::domain_details`].
    pub fn domain_details(&self) -> Option<DomainDetails> {
        self.details().domain_details()
    }

    /// The [`Self::details`]'s [`HostDetails::ipv4_details`].
    pub fn ipv4_details(&self) -> Option<Ipv4Details> {
        self.details().ipv4_details()
    }

    /// The [`Self::details`]'s [`HostDetails::ipv6_details`].
    pub fn ipv6_details(&self) -> Option<Ipv6Details> {
        self.details().ipv6_details()
    }

    /// The [`Self::as_str`] with any `www,` prefix and `.` suffix removed.
    pub fn normalized_host(&self) -> &str {
        let mut ret = self.as_str();
        ret = ret.strip_prefix("www.").unwrap_or(ret);
        ret = ret.strip_suffix(".").unwrap_or(ret);
        ret
    }

    /// The [`BetterUrl::domain`].
    pub fn domain(&self) -> Option<&str> {
        self.as_str().get(self.domain_details()?.domain_bounds())
    }

    /// The [`BetterUrl::subdomain`].
    pub fn subdomain(&self) -> Option<&str> {
        self.as_str().get(self.domain_details()?.subdomain_bounds()?)
    }

    /// The [`BetterUrl::not_domain_suffix`].
    pub fn not_domain_suffix(&self) -> Option<&str> {
        self.as_str().get(self.domain_details()?.not_domain_suffix_bounds()?)
    }

    /// The [`BetterUrl::domain_middle`].
    pub fn domain_middle(&self) -> Option<&str> {
        self.as_str().get(self.domain_details()?.domain_middle_bounds()?)
    }

    /// The [`BetterUrl::reg_domain`].
    pub fn reg_domain(&self) -> Option<&str> {
        self.as_str().get(self.domain_details()?.reg_domain_bounds()?)
    }

    /// The [`BetterUrl::domain_suffix`].
    pub fn domain_suffix(&self) -> Option<&str> {
        self.as_str().get(self.domain_details()?.domain_suffix_bounds()?)
    }
}

impl<'a> From<BetterHost<&'a str>> for &'a str {
    fn from(value: BetterHost<&'a str>) -> Self {
        value.string
    }
}

impl<'a> From<BetterHost<&'a str>> for Cow<'a, str> {
    fn from(value: BetterHost<&'a str>) -> Self {
        Cow::Borrowed(value.string)
    }
}

impl<'a> From<BetterHost<&'a str>> for String {
    fn from(value: BetterHost<&'a str>) -> Self {
        value.string.into()
    }
}

impl<'a> From<BetterHost<Cow<'a, str>>> for Cow<'a, str> {
    fn from(value: BetterHost<Cow<'a, str>>) -> Self {
        value.string
    }
}

impl<'a> From<BetterHost<Cow<'a, str>>> for String {
    fn from(value: BetterHost<Cow<'a, str>>) -> Self {
        value.string.into_owned()
    }
}

impl<'a> From<BetterHost<String>> for Cow<'a, str> {
    fn from(value: BetterHost<String>) -> Self {
        Cow::Owned(value.string)
    }
}

impl From<BetterHost<String>> for String {
    fn from(value: BetterHost<String>) -> Self {
        value.string
    }
}

impl<T: PartialEq<U> + AsRef<str>, U: AsRef<str>> PartialEq<BetterHost<U>> for BetterHost<T> {
    fn eq(&self, other: &BetterHost<U>) -> bool {
        self.string == other.string
    }
}
impl<T: Eq + AsRef<str>> Eq for BetterHost<T> {}

impl FromStr for BetterHost<String> {
    type Err = <HostDetails as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.to_owned().try_into()
    }
}

impl<'a> TryFrom<&'a str> for BetterHost<&'a str> {
    type Error = <HostDetails as FromStr>::Err;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            details: value.parse()?,
            string: value
        })
    }
}

impl TryFrom<String> for BetterHost<String> {
    type Error = <HostDetails as FromStr>::Err;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self {
            details: value.parse()?,
            string: value
        })
    }
}

impl<'a> TryFrom<Cow<'a, str>> for BetterHost<Cow<'a, str>> {
    type Error = <HostDetails as FromStr>::Err;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(Self {
            details: value.parse()?,
            string: value
        })
    }
}

impl<T: AsRef<str> + std::fmt::Display> std::fmt::Display for BetterHost<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.string)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: AsRef<str> + Deserialize<'de>> Deserialize<'de> for BetterHost<T> where BetterHost<T>: TryFrom<T>, <T as TryInto<BetterHost<T>>>::Error: std::fmt::Display {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer)?.try_into().map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl<T: AsRef<str> + Serialize> Serialize for BetterHost<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.string.serialize(serializer)
    }
}
