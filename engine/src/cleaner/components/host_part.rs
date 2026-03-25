//! [`HostPart`].

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;

/// A common API for getting various parts of [`BetterHost`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HostPart {
    /// [`BetterRefHost::as_str`].
    Host,
    /// [`BetterRefHost::normal`].
    NormalHost,
    /// [`BetterRefDomainHost::as_str`].
    Domain,
    /// [`BetterRefDomainHost::prefix`].
    DomainPrefix,
    /// [`BetterRefDomainHost::middle`].
    DomainMiddle,
    /// [`BetterRefDomainHost::suffix`].
    DomainSuffix,
    /// [`BetterRefDomainHost::labels`].
    DomainLabels,
    /// [`BetterRefDomainHost::origin`].
    DomainOrigin,
    /// [`BetterRefDomainHost::normal`].
    DomainNormal,
}

impl HostPart {
    /// Make a [`HostPart`].
    /// # Errors
    /// If `s` is an invalid [`HostPart`], returns the error [`InvalidHostPart`].
    pub fn parse(s: &str) -> Result<Self, InvalidHostPart> {
        match s {
            "Host"         => Ok(Self::Host),
            "NormalHost"   => Ok(Self::NormalHost),
            "Domain"       => Ok(Self::Domain),
            "DomainPrefix" => Ok(Self::DomainPrefix),
            "DomainMiddle" => Ok(Self::DomainMiddle),
            "DomainSuffix" => Ok(Self::DomainSuffix),
            "DomainLabels" => Ok(Self::DomainLabels),
            "DomainOrigin" => Ok(Self::DomainOrigin),
            "DomainNormal" => Ok(Self::DomainNormal),
            _ => Err(InvalidHostPart)
        }
    }

    /// Get this as a string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Host         => "Host",
            Self::NormalHost   => "NormalHost",
            Self::Domain       => "Domain",
            Self::DomainPrefix => "DomainPrefix",
            Self::DomainMiddle => "DomainMiddle",
            Self::DomainSuffix => "DomainSuffix",
            Self::DomainLabels => "DomainLabels",
            Self::DomainOrigin => "DomainOrigin",
            Self::DomainNormal => "DomainNormal",
        }
    }

    /// Get the part.
    pub fn get<'a>(&self, host: BetterRefHost<'a>) -> Option<&'a str> {
        Some(match self {
            Self::Host         => host.as_str(),
            Self::NormalHost   => host.normal(),
            Self::Domain       => host.domain()?.as_str() ,
            Self::DomainSuffix => host.domain()?.origin()?,
            Self::DomainPrefix => host.domain()?.prefix()?,
            Self::DomainMiddle => host.domain()?.middle()?,
            Self::DomainLabels => host.domain()?.labels() ,
            Self::DomainOrigin => host.domain()?.suffix() ,
            Self::DomainNormal => host.domain()?.normal() ,
        })
    }
}

/// Returned when trying to parse an invalid [`HostPart`].
#[derive(Debug, Error)]
#[error("Attempted to parse an invalid HostPart.")]
pub struct InvalidHostPart;

impl FromStr for HostPart {
    type Err = InvalidHostPart;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for HostPart {
    type Error = InvalidHostPart;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl std::fmt::Display for HostPart {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}
