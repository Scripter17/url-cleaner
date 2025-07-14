//! A common API for getting and setting various parts of [`BetterHost`]s.

use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

/// A common API for getting various parts of [`BetterHost`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HostPart {
    /// The host.
    Host,
    /// The normalized host.
    NormalizedHost,
    /// The domain.
    Domain,
    /// The subdomain.
    Subdomain,
    /// The not domain suffix.
    NotDomainSuffix,
    /// The domain middle.
    DomainMiddle,
    /// The reg domain.
    RegDomain,
    /// The domain suffix.
    DomainSuffix
}

impl HostPart {
    /// Get the part.
    pub fn get<'a>(&self, host: &'a BetterHost) -> Option<&'a str> {
        match self {
            Self::Host            => Some(host.host_str()),
            Self::NormalizedHost  => {
                let mut ret = host.host_str();
                ret = ret.strip_prefix("www.").unwrap_or(ret);
                ret = ret.strip_suffix(".").unwrap_or(ret);
                Some(ret)
            },
            Self::Domain          => host.domain(),
            Self::Subdomain       => host.subdomain(),
            Self::NotDomainSuffix => host.not_domain_suffix(),
            Self::DomainMiddle    => host.domain_middle(),
            Self::RegDomain       => host.reg_domain(),
            Self::DomainSuffix    => host.domain_suffix()
        }
    }
}
