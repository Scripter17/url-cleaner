//! [`OpaqueHostDetails`].

use crate::prelude::*;

/// Details for an [`OpaqueHost`].
///
/// Please note that while the [official algorithm](https://url.spec.whatwg.org/#concept-opaque-host-parser) implicitly allows the empty string,
/// the [opaque host type itself](https://url.spec.whatwg.org/#opaque-host) is specified to not be empty.
///
/// Therefore, at least for now, I have chosen to have the empty string return an error.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct OpaqueHostDetails;

impl OpaqueHostDetails {
    /// Parse an [`OpaqueHost`] literal.
    /// # Errors
    /// If the host is an invalid opaque host, returns the error [`InvalidOpaqueHost`].
    ///
    /// Please note that while the [opaque host parser](https://url.spec.whatwg.org/#concept-opaque-host-parser) implicitly allows the empty string,
    /// the [opaque host type itself](https://url.spec.whatwg.org/#opaque-host) is specified to not be empty.
    ///
    /// Therefore, empty opaque hosts are rejected.
    ///
    /// See [whatwg/url#908](https://github.com/whatwg/url/issues/908) for discussion.
    pub fn parse(value: &str) -> Result<Self, InvalidOpaqueHost> {
        if value.is_empty() {
            Err(InvalidOpaqueHost)?;
        }

        if value.bytes().any(|b| FORBIDDEN_HOST_LITERAL.contains(b)) {
            Err(InvalidOpaqueHost)?;
        }

        Ok(Self)
    }
}



impl FromStr for OpaqueHostDetails {
    type Err = InvalidOpaqueHost;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for OpaqueHostDetails {
    type Error = InvalidOpaqueHost;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}



impl TryFrom<HostDetails> for OpaqueHostDetails {
    type Error = HostDetails;

    fn try_from(value: HostDetails) -> Result<Self, Self::Error> {
        match value {
            HostDetails::Opaque(details) => Ok (details),
            details                      => Err(details),
        }
    }
}

impl TryFrom<NonSpecialHostDetails> for OpaqueHostDetails {
    type Error = NonSpecialHostDetails;

    fn try_from(value: NonSpecialHostDetails) -> Result<Self, Self::Error> {
        match value {
            NonSpecialHostDetails::Opaque(details) => Ok (details),
            details                                => Err(details)
        }
    }
}
