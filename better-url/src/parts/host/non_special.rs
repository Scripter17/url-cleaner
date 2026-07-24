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
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal and `details` must be its details.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, details: NonSpecialHostDetails) -> Self {
        unsafe {
            match details {
                NonSpecialHostDetails::Ipv6  (x) => Ipv6Host  ::new_unchecked(value, x).into(),
                NonSpecialHostDetails::Opaque(x) => OpaqueHost::new_unchecked(value, x).into(),
                NonSpecialHostDetails::Empty (x) => EmptyHost ::new_unchecked(value, x).into(),
            }
        }
    }

    /// Make a new [`Self::Opaque`] from a [`DomainHost`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_domain<T: TryInto<DomainHost<'a>>>(value: T) -> Result<Self, T::Error> {
        Ok(value.try_into()?.into())
    }

    /// Make a new [`Self::Opaque`] from an [`Ipv4Host`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_ipv4<T: TryInto<Ipv4Host<'a>>>(value: T) -> Result<Self, T::Error> {
        Ok(value.try_into()?.into())
    }

    /// Make a new [`Self::Ipv6`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_ipv6<T: TryInto<Ipv6Host<'a>>>(value: T) -> Result<Self, T::Error> {
        Ok(value.try_into()?.into())
    }

    /// Make a new [`Self::Opaque`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_opaque<T: TryInto<OpaqueHost<'a>>>(value: T) -> Result<Self, T::Error> {
        Ok(value.try_into()?.into())
    }

    /// Make a new [`Self::Empty`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_empty<T: TryInto<EmptyHost<'a>>>(value: T) -> Result<Self, T::Error> {
        Ok(value.try_into()?.into())
    }



    /// The host.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ipv6  (x) => x.as_str(),
            Self::Opaque(x) => x.as_str(),
            Self::Empty (x) => x.as_str(),
        }
    }

    /// The [`HostDetails`].
    pub fn details(&self) -> NonSpecialHostDetails {
        match self {
            Self::Ipv6  (x) => x.details().into(),
            Self::Opaque(x) => x.details().into(),
            Self::Empty (x) => x.details().into(),
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> NonSpecialHost<'_> {
        match self {
            Self::Ipv6  (x) => x.borrowed().into(),
            Self::Opaque(x) => x.borrowed().into(),
            Self::Empty (x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> NonSpecialHost<'static> {
        match self {
            Self::Ipv6  (x) => x.into_owned().into(),
            Self::Opaque(x) => x.into_owned().into(),
            Self::Empty (x) => x.into_owned().into(),
        }
    }

    /// turn into the inner [`Cow`] and [`HostDetails`].
    pub fn into_parts(self) -> (Cow<'a, str>, NonSpecialHostDetails) {
        match self {
            Self::Ipv6  (x) => {let (host, details) = x.into_parts(); (host, details.into())}
            Self::Opaque(x) => {let (host, details) = x.into_parts(); (host, details.into())}
            Self::Empty (x) => {let (host, details) = x.into_parts(); (host, details.into())}
        }
    }
}



impl<'a> TryFrom<Cow<'a, str>> for NonSpecialHost<'a> {
    type Error = InvalidNonSpecialHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(match value.as_bytes() {
            b""        => EmptyHost ::default(     ) .into(),
            [b'[', ..] => Ipv6Host  ::new    (value)?.into(),
            _          => OpaqueHost::new    (value)?.into(),
        })
    }
}



impl<'a> From<Host<'a>> for NonSpecialHost<'a> {
    fn from(value: Host<'a>) -> Self {
        match value {
            Host::Domain(x) => x.into(),
            Host::Ipv4  (x) => x.into(),
            Host::Ipv6  (x) => x.into(),
            Host::Opaque(x) => x.into(),
            Host::Empty (x) => x.into(),
        }
    }
}

impl<'a> From<FileHost<'a>> for NonSpecialHost<'a> {
    fn from(value: FileHost<'a>) -> Self {
        match value {
            FileHost::Domain(x) => x.into(),
            FileHost::Ipv4  (x) => x.into(),
            FileHost::Ipv6  (x) => x.into(),
            FileHost::Empty (x) => x.into(),
        }
    }
}

impl<'a> From<SpecialNotFileHost<'a>> for NonSpecialHost<'a> {
    fn from(value: SpecialNotFileHost<'a>) -> Self {
        match value {
            SpecialNotFileHost::Domain(x) => x.into(),
            SpecialNotFileHost::Ipv4  (x) => x.into(),
            SpecialNotFileHost::Ipv6  (x) => x.into(),
        }
    }
}

impl<'a> From<DomainHost<'a>> for NonSpecialHost<'a> {fn from(value: DomainHost<'a>) -> Self {Self::Opaque(value.into())}}
impl<'a> From<Ipv4Host  <'a>> for NonSpecialHost<'a> {fn from(value: Ipv4Host  <'a>) -> Self {Self::Opaque(value.into())}}
impl<'a> From<Ipv6Host  <'a>> for NonSpecialHost<'a> {fn from(value: Ipv6Host  <'a>) -> Self {Self::Ipv6  (value)}}
impl<'a> From<OpaqueHost<'a>> for NonSpecialHost<'a> {fn from(value: OpaqueHost<'a>) -> Self {Self::Opaque(value)}}
impl<'a> From<EmptyHost <'a>> for NonSpecialHost<'a> {fn from(value: EmptyHost <'a>) -> Self {Self::Empty (value)}}
