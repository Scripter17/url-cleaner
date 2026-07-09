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
    /// Make a new [`Self`] with zero validity checks.
    /// # Safety
    /// `value` must be a valid opaque host literal and `details` must be its [`OpaqueHostDetails`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, details: OpaqueHostDetails) -> Self {
        Self {
            host: value.into(),
            details,
        }
    }

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



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> OpaqueHost<'_> {
        OpaqueHost {
            host: Cow::Borrowed(&self.host),
            details: self.details
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> OpaqueHost<'static> {
        OpaqueHost {
            host: self.host.into_owned().into(),
            details: self.details
        }
    }
}



impl<'a> TryFrom<Cow<'a, str>> for OpaqueHost<'a> {
    type Error = InvalidOpaqueHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let (_, host) = encode_opaque_host(value)?;

        Ok(Self {
            host,
            details: Default::default(),
        })
    }
}



impl<'a> TryFrom<Host<'a>> for OpaqueHost<'a> {
    type Error = Host<'a>;

    fn try_from(value: Host<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            Host::Domain(x) => x.into(),
            Host::Ipv4  (x) => x.into(),
            Host::Opaque(x) => x,
            x               => Err(x)?,
        })
    }
}

impl<'a> TryFrom<FileHost<'a>> for OpaqueHost<'a> {
    type Error = FileHost<'a>;

    fn try_from(value: FileHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            FileHost::Domain(x) => x.into(),
            FileHost::Ipv4  (x) => x.into(),
            x                   => Err(x)?,
        })
    }
}

impl<'a> TryFrom<SpecialNotFileHost<'a>> for OpaqueHost<'a> {
    type Error = SpecialNotFileHost<'a>;

    fn try_from(value: SpecialNotFileHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            SpecialNotFileHost::Domain(x) => x.into(),
            SpecialNotFileHost::Ipv4  (x) => x.into(),
            x                             => Err(x)?,
        })
    }
}

impl<'a> TryFrom<NonSpecialHost<'a>> for OpaqueHost<'a> {
    type Error = NonSpecialHost<'a>;

    fn try_from(value: NonSpecialHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            NonSpecialHost::Opaque(x) => x,
            x                         => Err(x)?,
        })
    }
}


impl<'a> From<DomainHost<'a>> for OpaqueHost<'a> {fn from(value: DomainHost<'a>) -> Self {let (host, _) = value.into_parts(); Self {host, details: Default::default()}}}
impl<'a> From<Ipv4Host  <'a>> for OpaqueHost<'a> {fn from(value: Ipv4Host  <'a>) -> Self {let (host, _) = value.into_parts(); Self {host, details: Default::default()}}}
