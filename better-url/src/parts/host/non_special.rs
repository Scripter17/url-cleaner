//! [`NonSpecialHost`].

use crate::prelude::*;

/// A host for a [`SchemeType::NonSpecial`] URL.
#[derive(Debug, Clone)]
pub enum NonSpecialHost<'a> {
    /** [`Ipv6Host`].   **/ Ipv6  (Ipv6Host  <'a>),
    /** [`OpaqueHost`]. **/ Opaque(OpaqueHost<'a>),
    /** [`EmptyHost`].  **/ Empty (EmptyHost <'a>),
}

impl<'a> NonSpecialHost<'a> {
    /// The host.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ipv6  (x) => x.as_str(),
            Self::Opaque(x) => x.as_str(),
            Self::Empty (x) => x.as_str(),
        }
    }

    /// The [`HostDetails`].
    pub fn details(&self) -> HostDetails {
        match self {
            Self::Ipv6  (x) => x.details().into(),
            Self::Opaque(x) => x.details().into(),
            Self::Empty (x) => x.details().into(),
        }
    }
}

impl<'a> TryFrom<Cow<'a, str>> for NonSpecialHost<'a> {
    type Error = InvalidHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(if value.is_empty() {
            EmptyHost::default().into()
        } else if value.starts_with('[') {
            Ipv6Host::new(value)?.into()
        } else {
            OpaqueHost::new(value)?.into()
        })
    }
}

impl<'a> From<Ipv6Host  <'a>> for NonSpecialHost<'a> {fn from(value: Ipv6Host  <'a>) -> Self {Self::Ipv6  (value)}}
impl<'a> From<OpaqueHost<'a>> for NonSpecialHost<'a> {fn from(value: OpaqueHost<'a>) -> Self {Self::Opaque(value)}}
impl<'a> From<EmptyHost <'a>> for NonSpecialHost<'a> {fn from(value: EmptyHost <'a>) -> Self {Self::Empty (value)}}
