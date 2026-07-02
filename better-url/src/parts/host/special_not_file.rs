//! [`SpecialNotFileHost`].

use crate::prelude::*;

/// A host for a [`SchemeType::SpecialNotFile`] URL.
#[derive(Debug, Clone)]
pub enum SpecialNotFileHost<'a> {
    /** [`DomainHost`]. **/ Domain(DomainHost<'a>),
    /** [`Ipv4Host`].   **/ Ipv4  (Ipv4Host  <'a>),
    /** [`Ipv6Host`].   **/ Ipv6  (Ipv6Host  <'a>),
}

impl<'a> SpecialNotFileHost<'a> {
    /// The host.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Domain(x) => x.as_str(),
            Self::Ipv4  (x) => x.as_str(),
            Self::Ipv6  (x) => x.as_str(),
        }
    }

    /// The [`HostDetails`].
    pub fn details(&self) -> HostDetails {
        match self {
            Self::Domain(x) => x.details().into(),
            Self::Ipv4  (x) => x.details().into(),
            Self::Ipv6  (x) => x.details().into(),
        }
    }
}

impl<'a> TryFrom<Cow<'a, str>> for SpecialNotFileHost<'a> {
    type Error = InvalidHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(if value.starts_with('[') {
            Ipv6Host::new(value)?.into()
        } else {
            let value = lossy_percent_decode(value).1;
            match ends_in_a_number(&value) {
                true  => Ipv4Host  ::new_percent_decoded(value)?.into(),
                false => DomainHost::new_percent_decoded(value)?.into(),
            }
        })
    }
}

impl<'a> From<DomainHost<'a>> for SpecialNotFileHost<'a> {fn from(value: DomainHost<'a>) -> Self {Self::Domain(value)}}
impl<'a> From<Ipv4Host  <'a>> for SpecialNotFileHost<'a> {fn from(value: Ipv4Host  <'a>) -> Self {Self::Ipv4  (value)}}
impl<'a> From<Ipv6Host  <'a>> for SpecialNotFileHost<'a> {fn from(value: Ipv6Host  <'a>) -> Self {Self::Ipv6  (value)}}
