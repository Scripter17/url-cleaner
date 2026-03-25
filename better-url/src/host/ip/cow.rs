//! [`BetterIpHost`].

use std::borrow::{Cow, Borrow};
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Error as _}};

use crate::prelude::*;

/// Either a [`BetterIpv4Host`] or a [`BetterIpv6Host`].
#[derive(Debug, Clone)]
pub enum BetterIpHost<'a> {
    /// [`BetterIpv4Host`].
    V4(BetterIpv4Host<'a>),
    /// [`BetterIpv6Host`].
    V6(BetterIpv6Host<'a>),
}

impl<'a> BetterIpHost<'a> {
    /// Get the host as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::V4(x) => x.as_str(),
            Self::V6(x) => x.as_str(),
        }
    }

    /// Get the [`IpDetails`].
    pub fn details(&self) -> IpDetails {
        match self {
            Self::V4(x) => x.details().into(),
            Self::V6(x) => x.details().into(),
        }
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (Cow<'a, str>, IpDetails) {
        match self {
            Self::V4(x) => {let (host, details) = x.into_parts(); (host, details.into())},
            Self::V6(x) => {let (host, details) = x.into_parts(); (host, details.into())},
        }
    }

    /// Turn into an owned [`BetterIpHost`].
    pub fn into_owned(self) -> BetterIpHost<'static> {
        match self {
            Self::V4(x) => x.into_owned().into(),
            Self::V6(x) => x.into_owned().into(),
        }
    }

    /// Make a [`BetterRefIpHost`].
    pub fn to_ref(&self) -> BetterRefIpHost<'_> {
        match self {
            Self::V4(x) => x.to_ref().into(),
            Self::V6(x) => x.to_ref().into(),
        }
    }

    /// Convert into [`BetterIpv4Host`].
    pub fn ipv4(self) -> Option<BetterIpv4Host<'a>> {
        self.try_into().ok()
    }

    /// Convert into [`BetterIpv6Host`].
    pub fn ipv6(self) -> Option<BetterIpv6Host<'a>> {
        self.try_into().ok()
    }
}

impl PartialEq<str>          for BetterIpHost<'_> {fn eq(&self, other: &str         ) -> bool {self.as_str() ==  other}}
impl PartialEq<&str>         for BetterIpHost<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_str() == *other}}
impl PartialEq<String>       for BetterIpHost<'_> {fn eq(&self, other: &String      ) -> bool {self.as_str() ==  other}}
impl PartialEq<Cow<'_, str>> for BetterIpHost<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_str() ==  other}}

impl Eq for BetterIpHost<'_> {}

impl PartialEq<BetterHost      <'_>> for BetterIpHost<'_> {fn eq(&self,  other: &BetterHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterDomainHost<'_>> for BetterIpHost<'_> {fn eq(&self, _other: &BetterDomainHost<'_>) -> bool {false}}
impl PartialEq<BetterIpHost    <'_>> for BetterIpHost<'_> {fn eq(&self,  other: &BetterIpHost    <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterIpv4Host  <'_>> for BetterIpHost<'_> {fn eq(&self,  other: &BetterIpv4Host  <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterIpv6Host  <'_>> for BetterIpHost<'_> {fn eq(&self,  other: &BetterIpv6Host  <'_>) -> bool {self.as_str() == other.as_str()}}

impl PartialEq<BetterRefHost      <'_>> for BetterIpHost<'_> {fn eq(&self,  other: &BetterRefHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefDomainHost<'_>> for BetterIpHost<'_> {fn eq(&self, _other: &BetterRefDomainHost<'_>) -> bool {false}}
impl PartialEq<BetterRefIpHost    <'_>> for BetterIpHost<'_> {fn eq(&self,  other: &BetterRefIpHost    <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefIpv4Host  <'_>> for BetterIpHost<'_> {fn eq(&self,  other: &BetterRefIpv4Host  <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefIpv6Host  <'_>> for BetterIpHost<'_> {fn eq(&self,  other: &BetterRefIpv6Host  <'_>) -> bool {self.as_str() == other.as_str()}}

impl std::hash::Hash for BetterIpHost<'_> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}

impl PartialOrd for BetterIpHost<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BetterIpHost<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Borrow<str> for BetterIpHost<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<BetterIpv4Host<'a>> for BetterIpHost<'a> {fn from(value: BetterIpv4Host<'a>) -> Self {Self::V4(value)}}
impl<'a> From<BetterIpv6Host<'a>> for BetterIpHost<'a> {fn from(value: BetterIpv6Host<'a>) -> Self {Self::V6(value)}}

impl<'a> TryFrom<BetterHost<'a>> for BetterIpHost<'a> {
    type Error = BetterDomainHost<'a>;

    fn try_from(value: BetterHost<'a>) -> Result<Self, Self::Error> {
        match value {
            BetterHost::Domain(x) => Err(x),
            BetterHost::Ipv4(x) => Ok(x.into()),
            BetterHost::Ipv6(x) => Ok(x.into()),
        }
    }
}

impl FromStr for BetterIpHost<'static> {
    type Err = InvalidIpHost;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.to_string().try_into()
    }
}

impl<'a> TryFrom<&'a str> for BetterIpHost<'a> {
    type Error = InvalidIpHost;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Cow::from(value).try_into()
    }
}

impl TryFrom<String> for BetterIpHost<'static> {
    type Error = InvalidIpHost;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Cow::from(value).try_into()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for BetterIpHost<'a> {
    type Error = InvalidIpHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(if value.starts_with("[") {
            Self::V6(value.try_into()?)
        } else {
            Self::V4(value.try_into()?)
        })
    }
}

impl<'a> TryFrom<&'a String> for BetterIpHost<'a> {
    type Error = InvalidIpHost;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

impl<'a> TryFrom<&'a Cow<'_, str>> for BetterIpHost<'a> {
    type Error = InvalidIpHost;

    fn try_from(value: &'a Cow<'_, str>) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BetterIpHost<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Cow::<'de, str>::deserialize(deserializer)?.try_into().map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterIpHost<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl std::fmt::Display for BetterIpHost<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

