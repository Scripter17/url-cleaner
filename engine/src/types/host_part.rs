//! A common API for getting and setting various parts of [`BetterHost`]s.

use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

/// A common API for getting various parts of [`BetterHost`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HostPart {
    /// [`BetterHost::as_str`].
    Host,
    /// [`BetterHost::normalized_host`].
    NormalizedHost,
    /// [`BetterHost::domain`].
    Domain,
    /// [`BetterHost::subdomain`].
    Subdomain,
    /// [`BetterHost::not_domain_suffix`].
    NotDomainSuffix,
    /// [`BetterHost::domain_middle`].
    DomainMiddle,
    /// [`BetterHost::reg_domain`].
    RegDomain,
    /// [`BetterHost::domain_suffix`].
    DomainSuffix
}

impl HostPart {
    /// Get the part.
    pub fn get<'a, T: AsRef<str>>(&self, host: &'a BetterHost<T>) -> Option<&'a str> {
        match self {
            Self::Host            => Some(host.as_str()),
            Self::NormalizedHost  => Some(host.normalized_host()),
            Self::Domain          => host.domain(),
            Self::Subdomain       => host.subdomain(),
            Self::NotDomainSuffix => host.not_domain_suffix(),
            Self::DomainMiddle    => host.domain_middle(),
            Self::RegDomain       => host.reg_domain(),
            Self::DomainSuffix    => host.domain_suffix()
        }
    }
}
