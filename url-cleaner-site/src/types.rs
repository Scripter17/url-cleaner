//! The types used to make API responses.

use serde::{Serialize, Deserialize, ser::Serializer};
use thiserror::Error;

use url_cleaner_engine::types::*;

/// The payload of the `/clean` route.
/// 
/// Used to construct a [`Task`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkJob {
    /// The [`LazyTaskConfig`]s to use.
    pub tasks: Vec<LazyTaskConfig>,
    /// The [`JobContext`] to use.
    #[serde(default)]
    pub context: JobContext,
    /// The [`ParamsDiff`] to use.
    #[serde(default)]
    pub params_diff: Option<ParamsDiff>
}

/// The success state of doing a [`BulkJob`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleaningSuccess {
    /// The [`Task`] results.
    pub urls: Vec<Result<BetterUrl, String>>
}

/// The error state of doing a [`BulkJob`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleaningError {
    /// The HTTP status code.
    pub status: u16,
    /// The HTTP status reason.
    pub reason: Option<&'static str>
}

/// The error returned by the `host-parts` route when given an invalid host.
#[derive(Debug, Error)]
#[error("Couldn't parse host")]
pub struct CouldntParseHost;

impl Serialize for CouldntParseHost {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str("Couldn't parse the host.")
    }
}

/// Various parts of a host.
#[derive(Debug, Serialize)]
pub enum HostParts<'a> {
    /// Various parts of a domain.
    Domain(DomainParts<'a>),
    /// Various parts of an IPv4 host.
    Ipv4(Ipv4Parts),
    /// Various parts of an IPv6 host.
    Ipv6(Ipv6Parts)
}

impl<'a> TryFrom<&'a str> for HostParts<'a> {
    type Error = url::ParseError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match HostDetails::from_host_str(value)? {
            HostDetails::Domain(dd) => Self::Domain(DomainParts {
                domain           : value,
                subdomain        : dd.subdomain_bounds        ().and_then(|x| value.get(x)),
                not_domain_suffix: dd.not_domain_suffix_bounds().and_then(|x| value.get(x)),
                domain_middle    : dd.domain_middle_bounds    ().and_then(|x| value.get(x)),
                reg_domain       : dd.reg_domain_bounds       ().and_then(|x| value.get(x)),
                domain_suffix    : dd.domain_suffix_bounds    ().and_then(|x| value.get(x))
            }),
            HostDetails::Ipv4(_) => Self::Ipv4(Ipv4Parts),
            HostDetails::Ipv6(_) => Self::Ipv6(Ipv6Parts)
        })
    }
}

/// Various parts of a domain.
#[derive(Debug, Serialize)]
pub struct DomainParts<'a> {
    /// The entire domain.
    pub domain           : &'a str,
    /// The [`UrlPart::Subdomain`].
    pub subdomain        : Option<&'a str>,
    /// The [`UrlPart::NotDomainSuffix`].
    pub not_domain_suffix: Option<&'a str>,
    /// The [`UrlPart::DomainMiddle`].
    pub domain_middle    : Option<&'a str>,
    /// The [`UrlPart::RegDomain`].
    pub reg_domain       : Option<&'a str>,
    /// The [`UrlPart::DomainSuffix`].
    pub domain_suffix    : Option<&'a str>
}

/// Various parts of an IPv4 host.
#[derive(Debug, Serialize)]
pub struct Ipv4Parts;

/// Various parts of an IPv6 host.
#[derive(Debug, Serialize)]
pub struct Ipv6Parts;
