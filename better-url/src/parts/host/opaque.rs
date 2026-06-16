//! [`OpaqueHost`].

use crate::prelude::*;

/// An opauqe host.
///
/// Please note that while the [opaque host parser](https://url.spec.whatwg.org/#concept-opaque-host-parser) implicitly allows the empty string,
/// the [opaque host type itself](https://url.spec.whatwg.org/#opaque-host) is specified to not be empty.
///
/// Therefore, empty opaque hosts are rejected.
///
/// See [servo/rust-url#1112](https://github.com/servo/rust-url/issues/1112) and [whatwg/url#908](https://github.com/whatwg/url/issues/908) for discussion.
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

    /// Turn into the inner [`Cow`] and [`OpaqueHostDetails`].
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
            host: encode_opaque_host(value).1,
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
