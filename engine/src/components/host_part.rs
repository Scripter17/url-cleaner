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

    /** [`Host::domain_prefix`] + [`DomainSegments::to_unicode`]. **/ DomainPrefixToUnicode,
    /** [`Host::domain_middle`] + [`DomainSegment::to_unicode`].  **/ DomainMiddleToUnicode,
    /** [`Host::domain_suffix`] + [`DomainSegments::to_unicode`]. **/ DomainSuffixToUnicode,
    /** [`Host::domain_labels`] + [`DomainSegments::to_unicode`]. **/ DomainLabelsToUnicode,
    /** [`Host::domain_origin`] + [`DomainSegments::to_unicode`]. **/ DomainOriginToUnicode,
    /** [`Host::domain_normal`] + [`DomainSegments::to_unicode`]. **/ DomainNormalToUnicode,
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

            Self::DomainPrefixToUnicode => host.domain_prefix()?.to_unicode(),
            Self::DomainMiddleToUnicode => host.domain_middle()?.to_unicode(),
            Self::DomainSuffixToUnicode => host.domain_suffix()?.to_unicode(),
            Self::DomainLabelsToUnicode => host.domain_labels()?.to_unicode(),
            Self::DomainOriginToUnicode => host.domain_origin()?.to_unicode(),
            Self::DomainNormalToUnicode => host.domain_normal()?.to_unicode(),
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

            "DomainPrefixToUnicode" => Ok(Self::DomainPrefixToUnicode),
            "DomainMiddleToUnicode" => Ok(Self::DomainMiddleToUnicode),
            "DomainSuffixToUnicode" => Ok(Self::DomainSuffixToUnicode),
            "DomainLabelsToUnicode" => Ok(Self::DomainLabelsToUnicode),
            "DomainOriginToUnicode" => Ok(Self::DomainOriginToUnicode),
            "DomainNormalToUnicode" => Ok(Self::DomainNormalToUnicode),

            _ => Err(InvalidHostPart),
        }
    }
}
