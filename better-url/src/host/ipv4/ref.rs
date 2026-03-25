//! [`BetterRefIpv4Host`].

use std::borrow::{Cow, Borrow};

use crate::prelude::*;

#[cfg(feature = "serde")]
use serde::{Serialize, ser::Serializer, Deserialize, de::{Deserializer, Error as _}};

/// A borrowed IPv4 host.
#[derive(Debug, Clone, Copy)]
pub struct BetterRefIpv4Host<'a> {
    /// The host string.
    pub(crate) host: &'a str,
    /// The [`Ipv4Details`].
    pub(crate) details: Ipv4Details
}

impl<'a> BetterRefIpv4Host<'a> {
    /// Get the host as a string.
    pub fn as_str(self) -> &'a str {
        self.host
    }

    /// Get the [`Ipv4Details`].
    pub fn details(self) -> Ipv4Details {
        self.details
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (&'a str, Ipv4Details) {
        (self.host, self.details)
    }
}

impl PartialEq<str>          for BetterRefIpv4Host<'_> {fn eq(&self, other: &str         ) -> bool {self.as_str() ==  other}}
impl PartialEq<&str>         for BetterRefIpv4Host<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_str() == *other}}
impl PartialEq<String>       for BetterRefIpv4Host<'_> {fn eq(&self, other: &String      ) -> bool {self.as_str() ==  other}}
impl PartialEq<Cow<'_, str>> for BetterRefIpv4Host<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_str() ==  other}}

impl Eq for BetterRefIpv4Host<'_> {}

impl PartialEq<BetterHost      <'_>> for BetterRefIpv4Host<'_> {fn eq(&self,  other: &BetterHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterDomainHost<'_>> for BetterRefIpv4Host<'_> {fn eq(&self, _other: &BetterDomainHost<'_>) -> bool {false}}
impl PartialEq<BetterIpHost    <'_>> for BetterRefIpv4Host<'_> {fn eq(&self,  other: &BetterIpHost    <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterIpv4Host  <'_>> for BetterRefIpv4Host<'_> {fn eq(&self,  other: &BetterIpv4Host  <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterIpv6Host  <'_>> for BetterRefIpv4Host<'_> {fn eq(&self, _other: &BetterIpv6Host  <'_>) -> bool {false}}

impl PartialEq<BetterRefHost      <'_>> for BetterRefIpv4Host<'_> {fn eq(&self,  other: &BetterRefHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefDomainHost<'_>> for BetterRefIpv4Host<'_> {fn eq(&self, _other: &BetterRefDomainHost<'_>) -> bool {false}}
impl PartialEq<BetterRefIpHost    <'_>> for BetterRefIpv4Host<'_> {fn eq(&self,  other: &BetterRefIpHost    <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefIpv4Host  <'_>> for BetterRefIpv4Host<'_> {fn eq(&self,  other: &BetterRefIpv4Host  <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefIpv6Host  <'_>> for BetterRefIpv4Host<'_> {fn eq(&self, _other: &BetterRefIpv6Host  <'_>) -> bool {false}}

impl std::hash::Hash for BetterRefIpv4Host<'_> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}

impl PartialOrd for BetterRefIpv4Host<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BetterRefIpv4Host<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Borrow<str> for BetterRefIpv4Host<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<'a> TryFrom<&'a str> for BetterRefIpv4Host<'a> {
    type Error = InvalidIpv4Host;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            host: value,
            details: value.parse()?
        })
    }
}

impl<'a> TryFrom<&'a String> for BetterRefIpv4Host<'a> {
    type Error = InvalidIpv4Host;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

impl<'a> TryFrom<&'a Cow<'_, str>> for BetterRefIpv4Host<'a> {
    type Error = InvalidIpv4Host;

    fn try_from(value: &'a Cow<'_, str>) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

impl<'a> TryFrom<BetterRefIpHost<'a>> for BetterRefIpv4Host<'a> {
    type Error = BetterRefIpv6Host<'a>;

    fn try_from(value: BetterRefIpHost<'a>) -> Result<Self, Self::Error> {
        match value {
            BetterRefIpHost::V4(x) => Ok(x),
            BetterRefIpHost::V6(x) => Err(x),
        }
    }
}

impl<'a> TryFrom<BetterRefHost<'a>> for BetterRefIpv4Host<'a> {
    type Error = BetterRefHost<'a>;

    fn try_from(value: BetterRefHost<'a>) -> Result<Self, Self::Error> {
        match value.details() {
            HostDetails::Ipv4(details) => Ok(Self {host: value.as_str(), details}),
            _ => Err(value)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BetterRefIpv4Host<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <&str>::deserialize(deserializer)?.try_into().map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterRefIpv4Host<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl std::fmt::Display for BetterRefIpv4Host<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}
