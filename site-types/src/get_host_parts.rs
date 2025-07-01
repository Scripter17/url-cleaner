//! `/get-host-parts` stuff.

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use url_cleaner_engine::types::*;

/// The [`Result`] returned by the `/get-host-parts` route.
pub type GetHostPartsResult = Result<HostParts, CouldntParseHost>;

/// The error returned by the `host-parts` route when given an invalid host.
#[derive(Debug, Error, Serialize, Deserialize)]
#[error("Couldn't parse host")]
pub struct CouldntParseHost;

/// Various parts of a host.
///
/// Used by the userscript to send the `SOURCE_REG_DOMAIN` job var.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostParts {
    /// Various parts of a domain.
    Domain(DomainParts),
    /// Various parts of an IPv4 host.
    Ipv4(Ipv4Parts),
    /// Various parts of an IPv6 host.
    Ipv6(Ipv6Parts)
}

impl FromStr for HostParts {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match HostDetails::parse(s)? {
            HostDetails::Domain(dd) => Self::Domain(DomainParts {
                domain           : s.get(dd.domain_bounds()).expect("A domain.").into(),
                subdomain        : dd.subdomain_bounds        ().and_then(|x| s.get(x).map(Into::into)),
                not_domain_suffix: dd.not_domain_suffix_bounds().and_then(|x| s.get(x).map(Into::into)),
                domain_middle    : dd.domain_middle_bounds    ().and_then(|x| s.get(x).map(Into::into)),
                reg_domain       : dd.reg_domain_bounds       ().and_then(|x| s.get(x).map(Into::into)),
                domain_suffix    : dd.domain_suffix_bounds    ().and_then(|x| s.get(x).map(Into::into))
            }),
            HostDetails::Ipv4(_) => Self::Ipv4(Ipv4Parts),
            HostDetails::Ipv6(_) => Self::Ipv6(Ipv6Parts)
        })
    }
}

impl TryFrom<&str> for HostParts {
    type Error = url::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

/// Various parts of a domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainParts {
    /// The [`UrlPart::Domain`].
    pub domain           : String,
    /// The [`UrlPart::Subdomain`].
    pub subdomain        : Option<String>,
    /// The [`UrlPart::NotDomainSuffix`].
    pub not_domain_suffix: Option<String>,
    /// The [`UrlPart::DomainMiddle`].
    pub domain_middle    : Option<String>,
    /// The [`UrlPart::RegDomain`].
    pub reg_domain       : Option<String>,
    /// The [`UrlPart::DomainSuffix`].
    pub domain_suffix    : Option<String>
}

/// Various parts of an IPv4 host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ipv4Parts;

/// Various parts of an IPv6 host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ipv6Parts;
