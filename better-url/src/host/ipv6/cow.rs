//! [`BetterRefIpv6Host`].

use std::borrow::{Cow, Borrow};
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Error as _}};

use crate::prelude::*;

/// A maybe owned IPv6 host.
#[derive(Debug, Clone)]
pub struct BetterIpv6Host<'a> {
    /// The host string.
    pub(crate) host: Cow<'a, str>,
    /// The [`Ipv6Details`].
    pub(crate) details: Ipv6Details
}

impl<'a> BetterIpv6Host<'a> {
    /// Get the host as a string.
    pub fn as_str(&self) -> &str {
        &self.host
    }

    /// Get the [`Ipv6Details`].
    pub fn details(&self) -> Ipv6Details {
        self.details
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (Cow<'a, str>, Ipv6Details) {
        (self.host, self.details)
    }

    /// Turn into an owned [`BetterIpv6Host`].
    pub fn into_owned(self) -> BetterIpv6Host<'static> {
        BetterIpv6Host {
            host: self.host.into_owned().into(),
            details: self.details
        }
    }

    /// Make a [`BetterRefIpv6Host`].
    pub fn to_ref(&self) -> BetterRefIpv6Host<'_> {
        BetterRefIpv6Host {
            host: &self.host,
            details: self.details
        }
    }
}

impl PartialEq<str>          for BetterIpv6Host<'_> {fn eq(&self, other: &str         ) -> bool {self.as_str() ==  other}}
impl PartialEq<&str>         for BetterIpv6Host<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_str() == *other}}
impl PartialEq<String>       for BetterIpv6Host<'_> {fn eq(&self, other: &String      ) -> bool {self.as_str() ==  other}}
impl PartialEq<Cow<'_, str>> for BetterIpv6Host<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_str() ==  other}}

impl Eq for BetterIpv6Host<'_> {}

impl PartialEq<BetterHost      <'_>> for BetterIpv6Host<'_> {fn eq(&self,  other: &BetterHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterDomainHost<'_>> for BetterIpv6Host<'_> {fn eq(&self, _other: &BetterDomainHost<'_>) -> bool {false}}
impl PartialEq<BetterIpHost    <'_>> for BetterIpv6Host<'_> {fn eq(&self,  other: &BetterIpHost    <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterIpv4Host  <'_>> for BetterIpv6Host<'_> {fn eq(&self, _other: &BetterIpv4Host  <'_>) -> bool {false}}
impl PartialEq<BetterIpv6Host  <'_>> for BetterIpv6Host<'_> {fn eq(&self,  other: &BetterIpv6Host  <'_>) -> bool {self.as_str() == other.as_str()}}

impl PartialEq<BetterRefHost      <'_>> for BetterIpv6Host<'_> {fn eq(&self,  other: &BetterRefHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefDomainHost<'_>> for BetterIpv6Host<'_> {fn eq(&self, _other: &BetterRefDomainHost<'_>) -> bool {false}}
impl PartialEq<BetterRefIpHost    <'_>> for BetterIpv6Host<'_> {fn eq(&self,  other: &BetterRefIpHost    <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefIpv4Host  <'_>> for BetterIpv6Host<'_> {fn eq(&self, _other: &BetterRefIpv4Host  <'_>) -> bool {false}}
impl PartialEq<BetterRefIpv6Host  <'_>> for BetterIpv6Host<'_> {fn eq(&self,  other: &BetterRefIpv6Host  <'_>) -> bool {self.as_str() == other.as_str()}}

impl std::hash::Hash for BetterIpv6Host<'_> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}

impl PartialOrd for BetterIpv6Host<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BetterIpv6Host<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Borrow<str> for BetterIpv6Host<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for BetterIpv6Host<'static> {
    type Err = InvalidIpv6Host;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.to_string().try_into()
    }
}

impl<'a> TryFrom<&'a str> for BetterIpv6Host<'a> {
    type Error = InvalidIpv6Host;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Cow::from(value).try_into()
    }
}

impl TryFrom<String> for BetterIpv6Host<'static> {
    type Error = InvalidIpv6Host;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Cow::from(value).try_into()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for BetterIpv6Host<'a> {
    type Error = InvalidIpv6Host;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(Self {
            details: value.parse()?,
            host: value,
        })
    }
}

impl<'a> TryFrom<&'a String> for BetterIpv6Host<'a> {
    type Error = InvalidIpv6Host;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

impl<'a> TryFrom<&'a Cow<'_, str>> for BetterIpv6Host<'a> {
    type Error = InvalidIpv6Host;

    fn try_from(value: &'a Cow<'_, str>) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

impl<'a> From<BetterRefIpv6Host<'a>> for BetterIpv6Host<'a> {
    fn from(value: BetterRefIpv6Host<'a>) -> Self {
        Self {
            host: value.host.into(),
            details: value.details
        }
    }
}

impl<'a> TryFrom<BetterIpHost<'a>> for BetterIpv6Host<'a> {
    type Error = BetterIpv4Host<'a>;

    fn try_from(value: BetterIpHost<'a>) -> Result<Self, Self::Error> {
        match value {
            BetterIpHost::V4(x) => Err(x),
            BetterIpHost::V6(x) => Ok(x),
        }
    }
}

impl<'a> TryFrom<BetterHost<'a>> for BetterIpv6Host<'a> {
    type Error = BetterHost<'a>;

    fn try_from(value: BetterHost<'a>) -> Result<Self, Self::Error> {
        match value {
            BetterHost::Ipv6(x) => Ok(x),
            _ => Err(value)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BetterIpv6Host<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Cow::<'de, str>::deserialize(deserializer)?.try_into().map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterIpv6Host<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl std::fmt::Display for BetterIpv6Host<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}
