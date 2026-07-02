//! [`FileHost`].

use crate::prelude::*;

/// The host of a [`SchemeType::File`] URL.
#[derive(Debug, Clone)]
pub enum FileHost<'a> {
    /** [`DomainHost`]. **/ Domain(DomainHost<'a>),
    /** [`Ipv4Host`].   **/ Ipv4  (Ipv4Host  <'a>),
    /** [`Ipv6Host`].   **/ Ipv6  (Ipv6Host  <'a>),
    /** [`EmptyHost`].  **/ Empty (EmptyHost <'a>),
}

impl<'a> FileHost<'a> {
    /// The host.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Domain(x) => x.as_str(),
            Self::Ipv4  (x) => x.as_str(),
            Self::Ipv6  (x) => x.as_str(),
            Self::Empty (x) => x.as_str(),
        }
    }

    /// The [`HostDetails`].
    pub fn details(&self) -> HostDetails {
        match self {
            Self::Domain(x) => x.details().into(),
            Self::Ipv4  (x) => x.details().into(),
            Self::Ipv6  (x) => x.details().into(),
            Self::Empty (x) => x.details().into(),
        }
    }
}

impl<'a> TryFrom<Cow<'a, str>> for FileHost<'a> {
    type Error = InvalidHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(if value.is_empty() {
            EmptyHost::default().into()
        } else if value.starts_with('[') {
            Ipv6Host::new(value)?.into()
        } else {
            let value = lossy_percent_decode(value).1;
            let value = uts46_map_normalize(value).1;
            if value.is_empty() {
                Err(InvalidDomainHost)?
            } else if value == "localhost" {
                EmptyHost::default().into()
            } else if ends_in_a_number(&value) {
                Ipv4Host  ::new_normalized(value)?.into()
            } else {
                DomainHost::new_normalized(value)?.into()
            }
        })
    }
}

impl<'a> From<DomainHost<'a>> for FileHost<'a> {fn from(value: DomainHost<'a>) -> Self {Self::Domain(value)}}
impl<'a> From<Ipv4Host  <'a>> for FileHost<'a> {fn from(value: Ipv4Host  <'a>) -> Self {Self::Ipv4  (value)}}
impl<'a> From<Ipv6Host  <'a>> for FileHost<'a> {fn from(value: Ipv6Host  <'a>) -> Self {Self::Ipv6  (value)}}
impl<'a> From<EmptyHost <'a>> for FileHost<'a> {fn from(value: EmptyHost <'a>) -> Self {Self::Empty (value)}}
