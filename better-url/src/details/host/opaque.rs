//! [`OpaqueHostDetails`].

use crate::prelude::*;

/// Details for an [`OpaqueHost`].
///
/// Please note that while the [official algorithm](https://url.spec.whatwg.org/#concept-opaque-host-parser) implicitly allows the empty string,
/// the [opaque host type itself](https://url.spec.whatwg.org/#opaque-host) is specified to not be empty.
///
/// Therefore, at least for now, I have chosen to have the empty string return an error.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OpaqueHostDetails;

impl OpaqueHostDetails {
    /// Parse a raw opaque host.
    /// # Errors
    /// If the host is an invalid opaque host, returns the error [`InvalidOpaqueHost`].
    ///
    /// Please note that while the [opaque host parser](https://url.spec.whatwg.org/#concept-opaque-host-parser) implicitly allows the empty string,
    /// the [opaque host type itself](https://url.spec.whatwg.org/#opaque-host) is specified to not be empty.
    ///
    /// Therefore, empty opaque hosts are rejected.
    ///
    /// See [servo/rust-url#1112](https://github.com/servo/rust-url/issues/1112) and [whatwg/url#908](https://github.com/whatwg/url/issues/908) for discussion.
    pub fn from_raw(s: &str) -> Result<Self, InvalidOpaqueHost> {
        if s.is_empty() {
            Err(InvalidOpaqueHost)?;
        }

        if s.bytes().any(|b| b.is_ascii() && FORBIDDEN_HOST.contains(b)) {
            Err(InvalidOpaqueHost)?;
        }

        Ok(Self)
    }
}

impl FromStr for OpaqueHostDetails {
    type Err = InvalidOpaqueHost;

    /// [`Self::from_raw`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_raw(s)
    }
}

impl TryFrom<&str> for OpaqueHostDetails {
    type Error = InvalidOpaqueHost;

    /// [`Self::from_raw`].
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_raw(value)
    }
}

impl TryFrom<HostDetails> for OpaqueHostDetails {
    type Error = HostDetails;

    fn try_from(value: HostDetails) -> Result<Self, Self::Error> {
        match value {
            HostDetails::Opaque(details) => Ok(details),
            details => Err(details)
        }
    }
}
