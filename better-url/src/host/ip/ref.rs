//! [`BetterRefIpHost`].

use std::borrow::{Cow, Borrow};

use crate::prelude::*;

#[cfg(feature = "serde")]
use serde::{Serialize, ser::Serializer, Deserialize, de::{Deserializer, Error as _}};

/// Either a [`BetterRefIpv4Host`] or a [`BetterRefIpv6Host`].
#[derive(Debug, Clone, Copy)]
pub enum BetterRefIpHost<'a> {
    /// [`BetterRefIpv4Host`].
    V4(BetterRefIpv4Host<'a>),
    /// [`BetterRefIpv6Host`].
    V6(BetterRefIpv6Host<'a>),
}

impl<'a> BetterRefIpHost<'a> {
    /// Get the host as a string.
    pub fn as_str(self) -> &'a str {
        match self {
            Self::V4(x) => x.as_str(),
            Self::V6(x) => x.as_str(),
        }
    }

    /// Get the [`IpDetails`].
    pub fn details(self) -> IpDetails {
        match self {
            Self::V4(x) => x.details().into(),
            Self::V6(x) => x.details().into(),
        }
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (&'a str, IpDetails) {
        match self {
            Self::V4(x) => {let (host, details) = x.into_parts(); (host, details.into())},
            Self::V6(x) => {let (host, details) = x.into_parts(); (host, details.into())},
        }
    }

    /// Convert into [`BetterRefIpv4Host`].
    pub fn ipv4(self) -> Option<BetterRefIpv4Host<'a>> {
        self.try_into().ok()
    }

    /// Convert into [`BetterRefIpv6Host`].
    pub fn ipv6(self) -> Option<BetterRefIpv6Host<'a>> {
        self.try_into().ok()
    }
}

impl PartialEq<str>          for BetterRefIpHost<'_> {fn eq(&self, other: &str         ) -> bool {self.as_str() ==  other}}
impl PartialEq<&str>         for BetterRefIpHost<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_str() == *other}}
impl PartialEq<String>       for BetterRefIpHost<'_> {fn eq(&self, other: &String      ) -> bool {self.as_str() ==  other}}
impl PartialEq<Cow<'_, str>> for BetterRefIpHost<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_str() ==  other}}

impl Eq for BetterRefIpHost<'_> {}

impl PartialEq<BetterHost      <'_>> for BetterRefIpHost<'_> {fn eq(&self,  other: &BetterHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterDomainHost<'_>> for BetterRefIpHost<'_> {fn eq(&self, _other: &BetterDomainHost<'_>) -> bool {false}}
impl PartialEq<BetterIpHost    <'_>> for BetterRefIpHost<'_> {fn eq(&self,  other: &BetterIpHost    <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterIpv4Host  <'_>> for BetterRefIpHost<'_> {fn eq(&self,  other: &BetterIpv4Host  <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterIpv6Host  <'_>> for BetterRefIpHost<'_> {fn eq(&self,  other: &BetterIpv6Host  <'_>) -> bool {self.as_str() == other.as_str()}}

impl PartialEq<BetterRefHost      <'_>> for BetterRefIpHost<'_> {fn eq(&self,  other: &BetterRefHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefDomainHost<'_>> for BetterRefIpHost<'_> {fn eq(&self, _other: &BetterRefDomainHost<'_>) -> bool {false}}
impl PartialEq<BetterRefIpHost    <'_>> for BetterRefIpHost<'_> {fn eq(&self,  other: &BetterRefIpHost    <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefIpv4Host  <'_>> for BetterRefIpHost<'_> {fn eq(&self,  other: &BetterRefIpv4Host  <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefIpv6Host  <'_>> for BetterRefIpHost<'_> {fn eq(&self,  other: &BetterRefIpv6Host  <'_>) -> bool {self.as_str() == other.as_str()}}

impl std::hash::Hash for BetterRefIpHost<'_> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}

impl PartialOrd for BetterRefIpHost<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BetterRefIpHost<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Borrow<str> for BetterRefIpHost<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<BetterRefIpv4Host<'a>> for BetterRefIpHost<'a> {fn from(value: BetterRefIpv4Host<'a>) -> Self {Self::V4(value)}}
impl<'a> From<BetterRefIpv6Host<'a>> for BetterRefIpHost<'a> {fn from(value: BetterRefIpv6Host<'a>) -> Self {Self::V6(value)}}

impl<'a> TryFrom<BetterRefHost<'a>> for BetterRefIpHost<'a> {
    type Error = BetterRefDomainHost<'a>;

    fn try_from(value: BetterRefHost<'a>) -> Result<Self, Self::Error> {
        match value {
            BetterRefHost::Domain(x) => Err(x),
            BetterRefHost::Ipv4(x) => Ok(x.into()),
            BetterRefHost::Ipv6(x) => Ok(x.into()),
        }
    }
}

impl<'a> TryFrom<&'a str> for BetterRefIpHost<'a> {
    type Error = InvalidIpHost;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(if value.starts_with("[") {
            Self::V6(value.try_into()?)
        } else {
            Self::V4(value.try_into()?)
        })
    }
}

impl<'a> TryFrom<&'a String> for BetterRefIpHost<'a> {
    type Error = InvalidIpHost;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

impl<'a> TryFrom<&'a Cow<'_, str>> for BetterRefIpHost<'a> {
    type Error = InvalidIpHost;

    fn try_from(value: &'a Cow<'_, str>) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BetterRefIpHost<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <&str>::deserialize(deserializer)?.try_into().map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterRefIpHost<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl std::fmt::Display for BetterRefIpHost<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}
