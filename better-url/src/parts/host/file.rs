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
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal and `details` must be its details.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, details: FileHostDetails) -> Self {
        unsafe {
            match details {
                FileHostDetails::Domain(x) => DomainHost::new_unchecked(value, x).into(),
                FileHostDetails::Ipv4  (x) => Ipv4Host  ::new_unchecked(value, x).into(),
                FileHostDetails::Ipv6  (x) => Ipv6Host  ::new_unchecked(value, x).into(),
                FileHostDetails::Empty (x) => EmptyHost ::new_unchecked(value, x).into(),
            }
        }
    }



    /// Make a new [`Self::Domain`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_domain<T: TryInto<DomainHost<'a>>>(value: T) -> Result<Self, T::Error> {
        Ok(value.try_into()?.into())
    }

    /// Make a new [`Self::Ipv4`].
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

    /// Make a new [`Self::Empty`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_empty<T: TryInto<EmptyHost<'a>>>(value: T) -> Result<Self, T::Error> {
        Ok(value.try_into()?.into())
    }



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
    pub fn details(&self) -> FileHostDetails {
        match self {
            Self::Domain(x) => x.details().into(),
            Self::Ipv4  (x) => x.details().into(),
            Self::Ipv6  (x) => x.details().into(),
            Self::Empty (x) => x.details().into(),
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> FileHost<'_> {
        match self {
            Self::Domain(x) => x.borrowed().into(),
            Self::Ipv4  (x) => x.borrowed().into(),
            Self::Ipv6  (x) => x.borrowed().into(),
            Self::Empty (x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> FileHost<'static> {
        match self {
            Self::Domain(x) => x.into_owned().into(),
            Self::Ipv4  (x) => x.into_owned().into(),
            Self::Ipv6  (x) => x.into_owned().into(),
            Self::Empty (x) => x.into_owned().into(),
        }
    }

    /// turn into the inner [`Cow`] and [`HostDetails`].
    pub fn into_parts(self) -> (Cow<'a, str>, FileHostDetails) {
        match self {
            Self::Domain(x) => {let (host, details) = x.into_parts(); (host, details.into())}
            Self::Ipv4  (x) => {let (host, details) = x.into_parts(); (host, details.into())}
            Self::Ipv6  (x) => {let (host, details) = x.into_parts(); (host, details.into())}
            Self::Empty (x) => {let (host, details) = x.into_parts(); (host, details.into())}
        }
    }
}



impl<'a> TryFrom<Cow<'a, str>> for FileHost<'a> {
    type Error = InvalidFileHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(match value.as_bytes() {
            b""        => EmptyHost::default(     ) .into(),
            [b'[', ..] => Ipv6Host ::new    (value)?.into(),
            _ => {
                let (_, value) = try_percent_decode (value).map_err(|_| InvalidFileHost)?;
                let (_, value) = uts46_map_normalize(value);

                match value.as_bytes() {
                    b""          => Err(InvalidFileHost)?,
                    b"localhost" => EmptyHost::default().into(),
                    _ => match ends_in_a_number(&value) {
                        true  =>         Ipv4Host  ::new_normalized(value) ?.into(),
                        false => unsafe {DomainHost::new_normalized(value)}?.into(),
                    }
                }
            }
        })
    }
}



impl<'a> TryFrom<Host<'a>> for FileHost<'a> {
    type Error = Host<'a>;

    fn try_from(value: Host<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            Host::Domain(x) => x.into(),
            Host::Ipv4  (x) => x.into(),
            Host::Ipv6  (x) => x.into(),
            Host::Opaque(x) => x.try_into()?,
            Host::Empty (x) => x.into(),
        })
    }
}

impl<'a> From<SpecialNotFileHost<'a>> for FileHost<'a> {
    fn from(value: SpecialNotFileHost<'a>) -> Self {
        match value {
            SpecialNotFileHost::Domain(x) => x.into(),
            SpecialNotFileHost::Ipv6  (x) => x.into(),
            SpecialNotFileHost::Ipv4  (x) => x.into(),
        }
    }
}

impl<'a> TryFrom<NonSpecialHost<'a>> for FileHost<'a> {
    type Error = NonSpecialHost<'a>;

    fn try_from(value: NonSpecialHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            NonSpecialHost::Ipv6  (x) => x.into(),
            NonSpecialHost::Opaque(x) => x.try_into()?,
            NonSpecialHost::Empty (x) => x.into(),
        })
    }
}

impl<'a> From<DomainHost<'a>> for FileHost<'a> {fn from(value: DomainHost<'a>) -> Self {Self::Domain(value)}}
impl<'a> From<Ipv4Host  <'a>> for FileHost<'a> {fn from(value: Ipv4Host  <'a>) -> Self {Self::Ipv4  (value)}}
impl<'a> From<Ipv6Host  <'a>> for FileHost<'a> {fn from(value: Ipv6Host  <'a>) -> Self {Self::Ipv6  (value)}}
impl<'a> From<EmptyHost <'a>> for FileHost<'a> {fn from(value: EmptyHost <'a>) -> Self {Self::Empty (value)}}

impl<'a> TryFrom<OpaqueHost<'a>> for FileHost<'a> {
    type Error = OpaqueHost<'a>;

    fn try_from(value: OpaqueHost<'a>) -> Result<Self, Self::Error> {
        // TOOD: This is dumb.

        let (host, _) = value.clone().into_parts();

        host.try_into().map_err(|_| value)
    }
}
