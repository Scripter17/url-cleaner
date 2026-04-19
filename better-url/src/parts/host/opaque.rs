//! [`OpaqueHost`].

use crate::prelude::*;

/// An opauqe host.
///
/// Please note that while the [official algorithm](https://url.spec.whatwg.org/#concept-opaque-host-parser) implicitly allows the empty string,
/// the [opaque host type itself](https://url.spec.whatwg.org/#opaque-host) is specified to not be empty.
///
/// Therefore, at least for now, I have chosen to have the empty string return an error.
#[derive(Debug, Clone)]
pub struct OpaqueHost<'a> {
    /// The host.
    pub(crate) host: Cow<'a, str>,
    /// The [`OpaqueHostDetails`].
    pub(crate) details: OpaqueHostDetails
}

impl<'a> OpaqueHost<'a> {
    /// The host as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.host
    }

    /// The [`OpaqueHostDetails`].
    pub fn details(&self) -> OpaqueHostDetails {
        self.details
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (Cow<'a, str>, OpaqueHostDetails) {
        (self.host, self.details)
    }



    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> OpaqueHost<'static> {
        OpaqueHost {
            host: self.host.into_owned().into(),
            details: self.details
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> OpaqueHost<'_> {
        OpaqueHost {
            host: Cow::Borrowed(&self.host),
            details: self.details
        }
    }
}

impl<'a> TryFrom<Cow<'a, str>> for OpaqueHost<'a> {
    type Error = InvalidOpaqueHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(Self {
            details: value.parse()?,
            host: PartTranscoder::OpaqueHost.encode(value),
        })
    }
}

impl<'a> TryFrom<Host<'a>> for OpaqueHost<'a> {
    type Error = Host<'a>;

    fn try_from(value: Host<'a>) -> Result<Self, Self::Error> {
        match value {
            Host::Opaque(x) => Ok(x),
            x => Err(x),
        }
    }
}
