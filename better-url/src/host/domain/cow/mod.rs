//! [`BetterDomainHost`].

use std::ops::RangeBounds;
use std::borrow::{Cow, Borrow};
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Error as _}};

mod domain;
mod prefix;
mod middle;
mod suffix;
mod fqddot;
mod origin;
mod labels;
mod normal;

use crate::prelude::*;

/// A maybe owned domain host.
///
/// The only invariant the various setter methods upholds is that a [`DomainDetails`] made from [`Self::as_str`] is always identical to [`Self::details`].
///
/// For example, the following is valid and intended behavior:
///
/// ```
/// use better_url::prelude::*;
///
/// let mut domain = BetterDomainHost::try_from("example.co.uk").unwrap();
///
/// domain.set_suffix_segment(0, Some("abc.com")).unwrap();
///
/// assert_eq!(domain.prefix(), Some("example.abc"));
/// assert_eq!(domain.middle(), Some("com"));
/// assert_eq!(domain.suffix(), "uk");
/// ```
#[derive(Debug, Clone)]
pub struct BetterDomainHost<'a> {
    /// The host string.
    pub(crate) host: Cow<'a, str>,
    /// The [`DomainDetails`].
    pub(crate) details: DomainDetails
}

impl<'a> BetterDomainHost<'a> {
    /// Get the full host.
    pub fn as_str(&self) -> &str {
        &self.host
    }

    /// Get the [`DomainDetails`].
    pub fn details(&self) -> DomainDetails {
        self.details
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (Cow<'a, str>, DomainDetails) {
        (self.host, self.details)
    }

    /// Convert into an owned [`BetterDomainHost`].
    pub fn into_owned(self) -> BetterDomainHost<'static> {
        BetterDomainHost {
            host: self.host.into_owned().into(),
            details: self.details
        }
    }

    /// Make a [`BetterRefDomainHost`].
    pub fn to_ref(&self) -> BetterRefDomainHost<'_> {
        BetterRefDomainHost {
            host: &self.host,
            details: self.details
        }
    }

    /// Get a range of [`DomainPart`]s.
    pub fn get<B: RangeBounds<DomainPart>>(&self, range: B) -> Option<&str> {
        self.details.range(range).map(|r| &self.host[r])
    }
}

impl PartialEq<str>          for BetterDomainHost<'_> {fn eq(&self, other: &str         ) -> bool {self.as_str() ==  other}}
impl PartialEq<&str>         for BetterDomainHost<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_str() == *other}}
impl PartialEq<String>       for BetterDomainHost<'_> {fn eq(&self, other: &String      ) -> bool {self.as_str() ==  other}}
impl PartialEq<Cow<'_, str>> for BetterDomainHost<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_str() ==  other}}

impl Eq for BetterDomainHost<'_> {}

impl PartialEq<BetterHost      <'_>> for BetterDomainHost<'_> {fn eq(&self,  other: &BetterHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterDomainHost<'_>> for BetterDomainHost<'_> {fn eq(&self,  other: &BetterDomainHost<'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterIpHost    <'_>> for BetterDomainHost<'_> {fn eq(&self, _other: &BetterIpHost    <'_>) -> bool {false}}
impl PartialEq<BetterIpv4Host  <'_>> for BetterDomainHost<'_> {fn eq(&self, _other: &BetterIpv4Host  <'_>) -> bool {false}}
impl PartialEq<BetterIpv6Host  <'_>> for BetterDomainHost<'_> {fn eq(&self, _other: &BetterIpv6Host  <'_>) -> bool {false}}

impl PartialEq<BetterRefHost      <'_>> for BetterDomainHost<'_> {fn eq(&self,  other: &BetterRefHost      <'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefDomainHost<'_>> for BetterDomainHost<'_> {fn eq(&self,  other: &BetterRefDomainHost<'_>) -> bool {self.as_str() == other.as_str()}}
impl PartialEq<BetterRefIpHost    <'_>> for BetterDomainHost<'_> {fn eq(&self, _other: &BetterRefIpHost    <'_>) -> bool {false}}
impl PartialEq<BetterRefIpv4Host  <'_>> for BetterDomainHost<'_> {fn eq(&self, _other: &BetterRefIpv4Host  <'_>) -> bool {false}}
impl PartialEq<BetterRefIpv6Host  <'_>> for BetterDomainHost<'_> {fn eq(&self, _other: &BetterRefIpv6Host  <'_>) -> bool {false}}

impl std::hash::Hash for BetterDomainHost<'_> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}

impl PartialOrd for BetterDomainHost<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BetterDomainHost<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Borrow<str> for BetterDomainHost<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for BetterDomainHost<'static> {
    type Err = InvalidDomainHost;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.to_string().try_into()
    }
}

impl<'a> TryFrom<&'a str> for BetterDomainHost<'a> {
    type Error = InvalidDomainHost;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Cow::from(value).try_into()
    }
}

impl TryFrom<String> for BetterDomainHost<'static> {
    type Error = InvalidDomainHost;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Cow::from(value).try_into()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for BetterDomainHost<'a> {
    type Error = InvalidDomainHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(Self {
            details: value.parse()?,
            host: value,
        })
    }
}

impl<'a> TryFrom<&'a String> for BetterDomainHost<'a> {
    type Error = InvalidDomainHost;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

impl<'a> TryFrom<&'a Cow<'_, str>> for BetterDomainHost<'a> {
    type Error = InvalidDomainHost;

    fn try_from(value: &'a Cow<'_, str>) -> Result<Self, Self::Error> {
        (&**value).try_into()
    }
}

impl<'a> From<BetterRefDomainHost<'a>> for BetterDomainHost<'a> {
    fn from(value: BetterRefDomainHost<'a>) -> Self {
        Self {
            host: value.host.into(),
            details: value.details
        }
    }
}

impl<'a> TryFrom<BetterHost<'a>> for BetterDomainHost<'a> {
    type Error = BetterHost<'a>;

    fn try_from(value: BetterHost<'a>) -> Result<Self, Self::Error> {
        match value {
            BetterHost::Domain(x) => Ok(x),
            _ => Err(value)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BetterDomainHost<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <Cow<'de, str>>::deserialize(deserializer)?.try_into().map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterDomainHost<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl std::fmt::Display for BetterDomainHost<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}
