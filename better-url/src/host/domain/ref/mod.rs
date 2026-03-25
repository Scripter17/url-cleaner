//! [`BetterRefDomainHost`].

use std::ops::RangeBounds;
use std::borrow::{Cow, Borrow};

#[cfg(feature = "serde")]
use serde::{Serialize, ser::Serializer, Deserialize, de::{Deserializer, Error as _}};

use crate::prelude::*;

mod domain;
mod prefix;
mod middle;
mod suffix;
mod fqddot;
mod origin;
mod labels;
mod normal;

/// A borrowed domain host.
#[derive(Debug, Clone, Copy)]
pub struct BetterRefDomainHost<'a> {
    /// The host string.
    pub(crate) host: &'a str,
    /// The [`DomainDetails`].
    pub(crate) details: DomainDetails
}

impl<'a> BetterRefDomainHost<'a> {
    /// Get the full host.
    pub fn as_str(self) -> &'a str {
        self.host
    }

    /// Get the [`DomainDetails`].
    pub fn details(self) -> DomainDetails {
        self.details
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (&'a str, DomainDetails) {
        (self.host, self.details)
    }

    /// Get a range of [`DomainPart`]s.
    pub fn get<B: RangeBounds<DomainPart>>(self, range: B) -> Option<&'a str> {
        self.details.range(range).map(|r| &self.host[r])
    }
}

impl PartialEq<str>          for BetterRefDomainHost<'_> {fn eq(&self, other: &str         ) -> bool {self.as_str() ==  other}}
impl PartialEq<&str>         for BetterRefDomainHost<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_str() == *other}}
impl PartialEq<String>       for BetterRefDomainHost<'_> {fn eq(&self, other: &String      ) -> bool {self.as_str() ==  other}}
impl PartialEq<Cow<'_, str>> for BetterRefDomainHost<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_str() ==  other}}

impl Eq for BetterRefDomainHost<'_> {}

impl PartialEq<BetterHost      <'_>> for BetterRefDomainHost<'_> {fn eq(&self,  other: &BetterHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterDomainHost<'_>> for BetterRefDomainHost<'_> {fn eq(&self,  other: &BetterDomainHost<'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterIpHost    <'_>> for BetterRefDomainHost<'_> {fn eq(&self, _other: &BetterIpHost    <'_>) -> bool {false}}
impl PartialEq<BetterIpv4Host  <'_>> for BetterRefDomainHost<'_> {fn eq(&self, _other: &BetterIpv4Host  <'_>) -> bool {false}}
impl PartialEq<BetterIpv6Host  <'_>> for BetterRefDomainHost<'_> {fn eq(&self, _other: &BetterIpv6Host  <'_>) -> bool {false}}

impl PartialEq<BetterRefHost      <'_>> for BetterRefDomainHost<'_> {fn eq(&self,  other: &BetterRefHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefDomainHost<'_>> for BetterRefDomainHost<'_> {fn eq(&self,  other: &BetterRefDomainHost<'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefIpHost    <'_>> for BetterRefDomainHost<'_> {fn eq(&self, _other: &BetterRefIpHost    <'_>) -> bool {false}}
impl PartialEq<BetterRefIpv4Host  <'_>> for BetterRefDomainHost<'_> {fn eq(&self, _other: &BetterRefIpv4Host  <'_>) -> bool {false}}
impl PartialEq<BetterRefIpv6Host  <'_>> for BetterRefDomainHost<'_> {fn eq(&self, _other: &BetterRefIpv6Host  <'_>) -> bool {false}}

impl std::hash::Hash for BetterRefDomainHost<'_> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}

impl PartialOrd for BetterRefDomainHost<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BetterRefDomainHost<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Borrow<str> for BetterRefDomainHost<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<'a> TryFrom<&'a str> for BetterRefDomainHost<'a> {
    type Error = InvalidDomainHost;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            host: value,
            details: value.parse()?
        })
    }
}

impl<'a> TryFrom<&'a String> for BetterRefDomainHost<'a> {
    type Error = InvalidDomainHost;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

impl<'a> TryFrom<&'a Cow<'_, str>> for BetterRefDomainHost<'a> {
    type Error = InvalidDomainHost;

    fn try_from(value: &'a Cow<'_, str>) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

impl<'a> TryFrom<BetterRefHost<'a>> for BetterRefDomainHost<'a> {
    type Error = BetterRefHost<'a>;

    fn try_from(value: BetterRefHost<'a>) -> Result<Self, Self::Error> {
        match value.details() {
            HostDetails::Domain(details) => Ok(Self {host: value.as_str(), details}),
            _ => Err(value)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BetterRefDomainHost<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <&str>::deserialize(deserializer)?.try_into().map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterRefDomainHost<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl std::fmt::Display for BetterRefDomainHost<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}
