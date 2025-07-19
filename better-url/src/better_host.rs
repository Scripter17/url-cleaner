//! Like [`url::Host`] but better.

use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, ser::Serializer, de::{Visitor, Deserializer, Error}};

use crate::*;

/// A URL host and its details.
#[derive(Debug, Clone)]
pub struct BetterHost {
    /// The host string.
    string: String,
    /// The [`HostDetails`].
    details: HostDetails
}

impl BetterHost {
    /// The host string.
    pub fn host_str(&self) -> &str {
        &self.string
    }

    /// The host's [`HostDetails`].
    pub fn host_details(&self) -> &HostDetails {
        &self.details
    }

    /// The [`Self::host_details`]'s [`HostDetails::domain_details`].
    pub fn domain_details(&self) -> Option<&DomainDetails> {
        self.host_details().domain_details()
    }

    /// The [`Self::host_details`]'s [`HostDetails::ipv4_details`].
    pub fn ipv4_details(&self) -> Option<&Ipv4Details> {
        self.host_details().ipv4_details()
    }

    /// The [`Self::host_details`]'s [`HostDetails::ipv6_details`].
    pub fn ipv6_details(&self) -> Option<&Ipv6Details> {
        self.host_details().ipv6_details()
    }

    /// The [`Self::host_str`] with any `www,` prefix and `.` suffix removed.
    pub fn normalized_host(&self) -> &str {
        let mut ret = self.host_str();
        ret = ret.strip_prefix("www.").unwrap_or(ret);
        ret = ret.strip_suffix(".").unwrap_or(ret);
        ret
    }

    /// The [`BetterUrl::domain`].
    pub fn domain(&self) -> Option<&str> {
        self.host_str().get(self.domain_details()?.domain_bounds())
    }

    /// The [`BetterUrl::subdomain`].
    pub fn subdomain(&self) -> Option<&str> {
        self.host_str().get(self.domain_details()?.subdomain_bounds()?)
    }

    /// The [`BetterUrl::not_domain_suffix`].
    pub fn not_domain_suffix(&self) -> Option<&str> {
        self.host_str().get(self.domain_details()?.not_domain_suffix_bounds()?)
    }

    /// The [`BetterUrl::domain_middle`].
    pub fn domain_middle(&self) -> Option<&str> {
        self.host_str().get(self.domain_details()?.domain_middle_bounds()?)
    }

    /// The [`BetterUrl::reg_domain`].
    pub fn reg_domain(&self) -> Option<&str> {
        self.host_str().get(self.domain_details()?.reg_domain_bounds()?)
    }

    /// The [`BetterUrl::domain_suffix`].
    pub fn domain_suffix(&self) -> Option<&str> {
        self.host_str().get(self.domain_details()?.domain_suffix_bounds()?)
    }
}

impl PartialEq for BetterHost {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}
impl Eq for BetterHost {}

impl FromStr for BetterHost {
    type Err = <HostDetails as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            details: s.parse()?,
            string: s.into()
        })
    }
}

impl TryFrom<&str> for BetterHost {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for BetterHost {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self {
            details: value.parse()?,
            string: value
        })
    }
}

impl From<BetterHost> for String {
    fn from(value: BetterHost) -> String {
        value.string
    }
}

impl std::fmt::Display for BetterHost {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.string)
    }
}

/// Serde helper for deserializing [`BetterHost`].
#[cfg(feature = "serde")]
struct BetterHostVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for BetterHostVisitor {
    type Value = BetterHost;

    fn visit_str<E: Error>(self, s: &str) -> Result<Self::Value, E> {
        s.try_into().map_err(E::custom)
    }

    fn visit_string<E: Error>(self, s: String) -> Result<Self::Value, E> {
        s.try_into().map_err(E::custom)
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Expected a string")
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BetterHost {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(BetterHostVisitor)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterHost {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.string)
    }
}
