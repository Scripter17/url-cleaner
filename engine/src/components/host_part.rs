//! [`HostPart`].

use crate::prelude::*;

/// A common API for getting various parts of [`Host`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub enum HostPart {
    /** [`Host::as_str`]. **/ Host,

    /** [`Host::domain_prefix_str`]. **/ DomainPrefix,
    /** [`Host::domain_middle_str`]. **/ DomainMiddle,
    /** [`Host::domain_suffix_str`]. **/ DomainSuffix,
    /** [`Host::domain_labels_str`]. **/ DomainLabels,
    /** [`Host::domain_origin_str`]. **/ DomainOrigin,
    /** [`Host::domain_normal_str`]. **/ DomainNormal,

    /** [`Host::domain_prefix`] + [`DomainSegments::decode`]. **/ DecodedDomainPrefix,
    /** [`Host::domain_middle`] + [`DomainSegment::decode`].  **/ DecodedDomainMiddle,
    /** [`Host::domain_suffix`] + [`DomainSegments::decode`]. **/ DecodedDomainSuffix,
    /** [`Host::domain_labels`] + [`DomainSegments::decode`]. **/ DecodedDomainLabels,
    /** [`Host::domain_origin`] + [`DomainSegments::decode`]. **/ DecodedDomainOrigin,
    /** [`Host::domain_normal`] + [`DomainSegments::decode`]. **/ DecodedDomainNormal,
}

impl HostPart {
    /// Get the part.
    pub fn get<'a>(self, host: &'a Host<'_>) -> Option<Cow<'a, str>> {
        Some(match self {
            Self::Host         => host.as_str().into(),

            Self::DomainPrefix => host.domain_prefix_str()?.into(),
            Self::DomainMiddle => host.domain_middle_str()?.into(),
            Self::DomainSuffix => host.domain_suffix_str()?.into(),
            Self::DomainLabels => host.domain_labels_str()?.into(),
            Self::DomainOrigin => host.domain_origin_str()?.into(),
            Self::DomainNormal => host.domain_normal_str()?.into(),

            Self::DecodedDomainPrefix => host.domain_prefix()?.decode(),
            Self::DecodedDomainMiddle => host.domain_middle()?.decode(),
            Self::DecodedDomainSuffix => host.domain_suffix()?.decode(),
            Self::DecodedDomainLabels => host.domain_labels()?.decode(),
            Self::DecodedDomainOrigin => host.domain_origin()?.decode(),
            Self::DecodedDomainNormal => host.domain_normal()?.decode(),
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

            "DecodedDomainPrefix" => Ok(Self::DecodedDomainPrefix),
            "DecodedDomainMiddle" => Ok(Self::DecodedDomainMiddle),
            "DecodedDomainSuffix" => Ok(Self::DecodedDomainSuffix),
            "DecodedDomainLabels" => Ok(Self::DecodedDomainLabels),
            "DecodedDomainOrigin" => Ok(Self::DecodedDomainOrigin),
            "DecodedDomainNormal" => Ok(Self::DecodedDomainNormal),

            _ => Err(InvalidHostPart),
        }
    }
}
