//! Like [`url::Host`] but better.

use std::str::FromStr;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Error}};

use crate::prelude::*;

/// A URL host and its details.
#[derive(Debug, Clone)]
pub struct BetterHost {
    /// The host string.
    pub(crate) string: String,
    /// The [`HostDetails`].
    pub(crate) details: HostDetails
}

impl BetterHost {
    /// Parse a host string.
    /// # Errors
    /// If the call to [`HostDetails::from_str`] returns an error, that error is returned.
    pub fn parse(s: &str) -> Result<Self, <HostDetails as FromStr>::Err> {
        s.try_into()
    }

    /// Make a [`RefBetterHost`].
    pub fn borrowed(&self) -> RefBetterHost<'_> {
        RefBetterHost {
            string: &self.string,
            details: self.details
        }
    }

    /// The host string.
    pub fn as_str(&self) -> &str {
        self.borrowed().as_str()
    }

    /// The host's [`HostDetails`].
    pub fn details(&self) -> HostDetails {
        self.borrowed().details()
    }

    /// The [`Self::details`]'s [`HostDetails::domain_details`].
    pub fn domain_details(&self) -> Option<DomainDetails> {
        self.borrowed().domain_details()
    }

    /// The [`Self::details`]'s [`HostDetails::ipv4_details`].
    pub fn ipv4_details(&self) -> Option<Ipv4Details> {
        self.borrowed().ipv4_details()
    }

    /// The [`Self::details`]'s [`HostDetails::ipv6_details`].
    pub fn ipv6_details(&self) -> Option<Ipv6Details> {
        self.borrowed().ipv6_details()
    }

    /// The [`Self::as_str`] with any `www,` prefix and `.` suffix removed.
    pub fn normalized_host(&self) -> &str {
        self.borrowed().normalized_host()
    }

    /// The [`BetterUrl::domain`].
    pub fn domain(&self) -> Option<&str> {
        self.borrowed().domain()
    }

    /// The [`BetterUrl::subdomain`].
    pub fn subdomain(&self) -> Option<&str> {
        self.borrowed().subdomain()
    }

    /// The [`BetterUrl::not_domain_suffix`].
    pub fn not_domain_suffix(&self) -> Option<&str> {
        self.borrowed().not_domain_suffix()
    }

    /// The [`BetterUrl::domain_middle`].
    pub fn domain_middle(&self) -> Option<&str> {
        self.borrowed().domain_middle()
    }

    /// The [`BetterUrl::reg_domain`].
    pub fn reg_domain(&self) -> Option<&str> {
        self.borrowed().reg_domain()
    }

    /// The [`BetterUrl::domain_suffix`].
    pub fn domain_suffix(&self) -> Option<&str> {
        self.borrowed().domain_suffix()
    }
}

impl FromStr for BetterHost {
    type Err = <HostDetails as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl TryFrom<&str> for BetterHost {
    type Error = <HostDetails as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl TryFrom<String> for BetterHost {
    type Error = <HostDetails as FromStr>::Err;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self {
            details: value.parse()?,
            string: value
        })
    }
}

impl PartialEq for BetterHost {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl Eq for BetterHost {}

impl PartialOrd for BetterHost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BetterHost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.string.cmp(&other.string)
    }
}

impl Hash for BetterHost {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.string.hash(state)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BetterHost {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?.try_into().map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterHost {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.string.serialize(serializer)
    }
}
