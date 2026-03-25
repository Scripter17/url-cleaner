//! [`BetterRefHost`].

use std::borrow::{Cow, Borrow};

use crate::prelude::*;

#[cfg(feature = "serde")]
use serde::{Serialize, ser::Serializer, Deserialize, de::{Deserializer, Error as _}};

/// A borrowed host.
#[derive(Debug, Clone, Copy)]
pub enum BetterRefHost<'a> {
    /// [`BetterRefDomainHost`].
    Domain(BetterRefDomainHost<'a>),
    /// [`BetterRefIpv4Host`].
    Ipv4(BetterRefIpv4Host<'a>),
    /// [`BetterRefIpv6Host`].
    Ipv6(BetterRefIpv6Host<'a>),
}

impl<'a> BetterRefHost<'a> {
    /// Get the host as a string.
    pub fn as_str(self) -> &'a str {
        match self {
            Self::Domain(x) => x.as_str(),
            Self::Ipv4  (x) => x.as_str(),
            Self::Ipv6  (x) => x.as_str(),
        }
    }

    /// Get the [`HostDetails`].
    pub fn details(self) -> HostDetails {
        match self {
            Self::Domain(x) => x.details().into(),
            Self::Ipv4  (x) => x.details().into(),
            Self::Ipv6  (x) => x.details().into(),
        }
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (&'a str, HostDetails) {
        match self {
            Self::Domain(x) => {let (host, details) = x.into_parts(); (host, details.into())},
            Self::Ipv4  (x) => {let (host, details) = x.into_parts(); (host, details.into())},
            Self::Ipv6  (x) => {let (host, details) = x.into_parts(); (host, details.into())},
        }
    }

    /// Get the normal host.
    ///
    /// For [`Self::Domain`], [`BetterRefDomainHost::normal`].
    ///
    /// Otherwise, [`Self::as_str`].
    pub fn normal(self) -> &'a str {
        match self {
            Self::Domain(x) => x.normal(),
            Self::Ipv4  (x) => x.as_str(),
            Self::Ipv6  (x) => x.as_str(),
        }
    }

    /// Get the [`BetterRefDomainHost`].
    pub fn domain(self) -> Option<BetterRefDomainHost<'a>> {
        self.try_into().ok()
    }

    /// Get the [`BetterRefIpv4Host`].
    pub fn ipv4(self) -> Option<BetterRefIpv4Host<'a>> {
        self.try_into().ok()
    }

    /// Get the [`BetterRefIpv6Host`].
    pub fn ipv6(self) -> Option<BetterRefIpv6Host<'a>> {
        self.try_into().ok()
    }
}

impl PartialEq<str>          for BetterRefHost<'_> {fn eq(&self, other: &str         ) -> bool {self.as_str() ==  other}}
impl PartialEq<&str>         for BetterRefHost<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_str() == *other}}
impl PartialEq<String>       for BetterRefHost<'_> {fn eq(&self, other: &String      ) -> bool {self.as_str() ==  other}}
impl PartialEq<Cow<'_, str>> for BetterRefHost<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_str() ==  other}}

impl Eq for BetterRefHost<'_> {}

impl PartialEq<BetterHost      <'_>> for BetterRefHost<'_> {fn eq(&self, other: &BetterHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterDomainHost<'_>> for BetterRefHost<'_> {fn eq(&self, other: &BetterDomainHost<'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterIpHost    <'_>> for BetterRefHost<'_> {fn eq(&self, other: &BetterIpHost    <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterIpv4Host  <'_>> for BetterRefHost<'_> {fn eq(&self, other: &BetterIpv4Host  <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterIpv6Host  <'_>> for BetterRefHost<'_> {fn eq(&self, other: &BetterIpv6Host  <'_>) -> bool {self.as_str() == other.as_str()}}

impl PartialEq<BetterRefHost      <'_>> for BetterRefHost<'_> {fn eq(&self, other: &BetterRefHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefDomainHost<'_>> for BetterRefHost<'_> {fn eq(&self, other: &BetterRefDomainHost<'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefIpHost    <'_>> for BetterRefHost<'_> {fn eq(&self, other: &BetterRefIpHost    <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefIpv4Host  <'_>> for BetterRefHost<'_> {fn eq(&self, other: &BetterRefIpv4Host  <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefIpv6Host  <'_>> for BetterRefHost<'_> {fn eq(&self, other: &BetterRefIpv6Host  <'_>) -> bool {self.as_str() == other.as_str()}}

impl std::hash::Hash for BetterRefHost<'_> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}

impl PartialOrd for BetterRefHost<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BetterRefHost<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Borrow<str> for BetterRefHost<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<BetterRefDomainHost<'a>> for BetterRefHost<'a> {fn from(value: BetterRefDomainHost<'a>) -> Self {Self::Domain(value)}}
impl<'a> From<BetterRefIpv4Host  <'a>> for BetterRefHost<'a> {fn from(value: BetterRefIpv4Host  <'a>) -> Self {Self::Ipv4  (value)}}
impl<'a> From<BetterRefIpv6Host  <'a>> for BetterRefHost<'a> {fn from(value: BetterRefIpv6Host  <'a>) -> Self {Self::Ipv6  (value)}}

impl<'a> TryFrom<&'a str> for BetterRefHost<'a> {
    type Error = InvalidHost;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value.parse()? {
            HostDetails::Domain(details) => Self::Domain(BetterRefDomainHost {host: value, details}),
            HostDetails::Ipv4  (details) => Self::Ipv4  (BetterRefIpv4Host   {host: value, details}),
            HostDetails::Ipv6  (details) => Self::Ipv6  (BetterRefIpv6Host   {host: value, details}),
        })
    }
}

impl<'a> TryFrom<&'a String> for BetterRefHost<'a> {
    type Error = InvalidHost;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

impl<'a> TryFrom<&'a Cow<'_, str>> for BetterRefHost<'a> {
    type Error = InvalidHost;

    fn try_from(value: &'a Cow<'_, str>) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BetterRefHost<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <&str>::deserialize(deserializer)?.try_into().map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterRefHost<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl std::fmt::Display for BetterRefHost<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}
