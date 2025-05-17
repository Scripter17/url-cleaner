//! Types used by URL Cleaner Site.
//!
//! Can be used to parse its output.

use std::str::FromStr;

use serde::{Serialize, Deserialize, ser::Serializer};
use thiserror::Error;

use url_cleaner_engine::types::*;

/// Returns [`true`] if `x` is `T`'s [`Default::default`].
fn is_default<T: Default + PartialEq>(x: &T) -> bool {x == &T::default()}

/// The payload of the `/clean` route.
/// 
/// Used to construct a [`Job`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkJob {
    /// The [`LazyTaskConfig`]s to use.
    pub tasks: Vec<LazyTaskConfig>,
    /// The [`JobContext`] to use.
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: JobContext,
    /// The [`ParamsDiff`] to use.
    #[serde(default, skip_serializing_if = "is_default")]
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
///
/// Used by the userscript to send the `SOURCE_REG_DOMAIN` job var.
#[derive(Debug, Serialize)]
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
        Ok(match HostDetails::from_host_str(s)? {
            HostDetails::Domain(dd) => Self::Domain(DomainParts {
                domain           : s.into(),
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
    /// The entire domain.
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
#[derive(Debug, Serialize)]
pub struct Ipv4Parts;

/// Various parts of an IPv6 host.
#[derive(Debug, Serialize)]
pub struct Ipv6Parts;
