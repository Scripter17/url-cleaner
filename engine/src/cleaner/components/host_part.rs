//! [`HostPart`].

use std::str::FromStr;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;

/// A common API for getting various parts of [`Host`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HostPart {
    /// [`Host::as_str`].
    Host,
    /// [`DomainHost::prefix`].
    DomainPrefix,
    /// [`DomainHost::middle`].
    DomainMiddle,
    /// [`DomainHost::suffix`].
    DomainSuffix,
    /// [`DomainHost::labels`].
    DomainLabels,
    /// [`DomainHost::origin`].
    DomainOrigin,
    /// [`DomainHost::normal`].
    DomainNormal,
}

impl HostPart {
    /// Make a [`HostPart`].
    /// # Errors
    /// If `s` is an invalid [`HostPart`], returns the error [`InvalidHostPart`].
    pub fn parse(s: &str) -> Result<Self, InvalidHostPart> {
        match s {
            "Host"         => Ok(Self::Host),
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
            Self::DomainPrefix => "DomainPrefix",
            Self::DomainMiddle => "DomainMiddle",
            Self::DomainSuffix => "DomainSuffix",
            Self::DomainLabels => "DomainLabels",
            Self::DomainOrigin => "DomainOrigin",
            Self::DomainNormal => "DomainNormal",
        }
    }

    /// Get the part.
    pub fn get<'a>(self, host: &'a Host<'_>) -> Option<&'a str> {
        Some(match self {
            Self::Host         => host.as_str(),
            Self::DomainPrefix => host.domain_prefix()?,
            Self::DomainMiddle => host.domain_middle()?,
            Self::DomainSuffix => host.domain_suffix()?,
            Self::DomainLabels => host.domain_labels()?,
            Self::DomainOrigin => host.domain_origin()?,
            Self::DomainNormal => host.domain_normal()?,
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
