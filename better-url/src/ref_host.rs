//! Like [`BetterHost`] but using `&str`.

use std::str::FromStr;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

#[cfg(feature = "serde")]
use serde::{Serialize, ser::Serializer};

use crate::prelude::*;

/// A URL host and its details.
#[derive(Debug, Clone, Copy)]
pub struct RefBetterHost<'a> {
    /// The host string.
    pub(crate) string: &'a str,
    /// The [`HostDetails`].
    pub(crate) details: HostDetails
}

impl<'a> RefBetterHost<'a> {
    /// Parse a host string.
    /// # Errors
    /// If the call to [`HostDetails::from_str`] returns an error, that error is returned.
    pub fn parse(s: &'a str) -> Result<Self, <HostDetails as FromStr>::Err> {
        s.try_into()
    }

    /// Make a [`BetterHost`].
    pub fn owned(self) -> BetterHost {
        BetterHost {
            string : self.string.to_owned(),
            details: self.details
        }
    }

    /// The host string.
    pub fn as_str(self) -> &'a str {
        self.string
    }

    /// The host's [`HostDetails`].
    pub fn details(self) -> HostDetails {
        self.details
    }

    /// The [`Self::details`]'s [`HostDetails::domain_details`].
    pub fn domain_details(self) -> Option<DomainDetails> {
        self.details().domain_details()
    }

    /// The [`Self::details`]'s [`HostDetails::ipv4_details`].
    pub fn ipv4_details(self) -> Option<Ipv4Details> {
        self.details().ipv4_details()
    }

    /// The [`Self::details`]'s [`HostDetails::ipv6_details`].
    pub fn ipv6_details(self) -> Option<Ipv6Details> {
        self.details().ipv6_details()
    }

    /// The [`Self::as_str`] with any `www,` prefix and `.` suffix removed.
    pub fn normalized_host(self) -> &'a str {
        let mut ret = self.as_str();
        ret = ret.strip_prefix("www.").unwrap_or(ret);
        ret = ret.strip_suffix(".").unwrap_or(ret);
        ret
    }

    /// The [`BetterUrl::domain`].
    pub fn domain(self) -> Option<&'a str> {
        self.as_str().get(self.domain_details()?.domain_bounds())
    }

    /// The [`BetterUrl::subdomain`].
    pub fn subdomain(self) -> Option<&'a str> {
        self.as_str().get(self.domain_details()?.subdomain_bounds()?)
    }

    /// The [`BetterUrl::not_domain_suffix`].
    pub fn not_domain_suffix(self) -> Option<&'a str> {
        self.as_str().get(self.domain_details()?.not_domain_suffix_bounds()?)
    }

    /// The [`BetterUrl::domain_middle`].
    pub fn domain_middle(self) -> Option<&'a str> {
        self.as_str().get(self.domain_details()?.domain_middle_bounds()?)
    }

    /// The [`BetterUrl::reg_domain`].
    pub fn reg_domain(self) -> Option<&'a str> {
        self.as_str().get(self.domain_details()?.reg_domain_bounds()?)
    }

    /// The [`BetterUrl::domain_suffix`].
    pub fn domain_suffix(self) -> Option<&'a str> {
        self.as_str().get(self.domain_details()?.domain_suffix_bounds()?)
    }
}

impl<'a> TryFrom<&'a str> for RefBetterHost<'a> {
    type Error = <HostDetails as FromStr>::Err;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            details: value.parse()?,
            string: value
        })
    }
}

impl PartialEq for RefBetterHost<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl Eq for RefBetterHost<'_> {}

impl PartialOrd for RefBetterHost<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RefBetterHost<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.string.cmp(other.string)
    }
}

impl Hash for RefBetterHost<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.string.hash(state)
    }
}

#[cfg(feature = "serde")]
impl Serialize for RefBetterHost<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.string.serialize(serializer)
    }
}
