//! [`HostPart`].

use crate::prelude::*;

/// A common API for getting various parts of [`Host`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HostPart {
    /// [`Host::as_str`].
    Host,
    /// [`Host::domain_prefix`] + [`DomainSegments::decode`].
    DomainPrefix,
    /// [`Host::domain_middle`] + [`DomainSegment::decode`].
    DomainMiddle,
    /// [`Host::domain_suffix`] + [`DomainSegments::decode`].
    DomainSuffix,
    /// [`Host::domain_labels`] + [`DomainSegments::decode`].
    DomainLabels,
    /// [`Host::domain_origin`] + [`DomainSegments::decode`].
    DomainOrigin,
    /// [`Host::domain_normal`] + [`DomainSegments::decode`].
    DomainNormal,
}

impl HostPart {
    /// Get the part.
    pub fn get<'a>(self, host: &'a Host<'_>) -> Option<Cow<'a, str>> {
        Some(match self {
            Self::Host         => host.as_str().into(),
            Self::DomainPrefix => host.domain_prefix()?.decode(),
            Self::DomainMiddle => host.domain_middle()?.decode(),
            Self::DomainSuffix => host.domain_suffix()?.decode(),
            Self::DomainLabels => host.domain_labels()?.decode(),
            Self::DomainOrigin => host.domain_origin()?.decode(),
            Self::DomainNormal => host.domain_normal()?.decode(),
        })
    }
}

impl FromStr for HostPart {
    type Err = InvalidHostPart;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
}
