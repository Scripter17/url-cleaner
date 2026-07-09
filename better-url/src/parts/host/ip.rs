//! [`IpHost`].

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::prelude::*;

/// Either a [`Ipv4Host`] or a [`Ipv6Host`].
#[derive(Debug, Clone)]
pub enum IpHost<'a> {
    /// [`Ipv4Host`].
    V4(Ipv4Host<'a>),
    /// [`Ipv6Host`].
    V6(Ipv6Host<'a>),
}

impl<'a> IpHost<'a> {
    /// Make a new [`Self::V4`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_v4<T: TryInto<Ipv4Host<'a>>>(value: T) -> Result<Self, T::Error> {
        Ok(value.try_into()?.into())
    }

    /// Make a new [`Self::V6`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_v6<T: TryInto<Ipv6Host<'a>>>(value: T) -> Result<Self, T::Error> {
        Ok(value.try_into()?.into())
    }



    /// The host as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::V4(x) => x.as_str(),
            Self::V6(x) => x.as_str(),
        }
    }

    /// The [`IpDetails`].
    pub fn details(&self) -> IpDetails {
        match self {
            Self::V4(x) => x.details().into(),
            Self::V6(x) => x.details().into(),
        }
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (Cow<'a, str>, IpDetails) {
        match self {
            Self::V4(x) => {let (host, details) = x.into_parts(); (host, details.into())},
            Self::V6(x) => {let (host, details) = x.into_parts(); (host, details.into())},
        }
    }



    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> IpHost<'static> {
        match self {
            Self::V4(x) => x.into_owned().into(),
            Self::V6(x) => x.into_owned().into(),
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> IpHost<'_> {
        match self {
            Self::V4(x) => x.borrowed().into(),
            Self::V6(x) => x.borrowed().into(),
        }
    }



    /// Borrow as an [`Ipv4Host`].
    pub fn as_ipv4(&self) -> Option<Ipv4Host<'_>> {
        self.borrowed().ipv4()
    }

    /// Borrow as an [`Ipv6Host`].
    pub fn as_ipv6(&self) -> Option<Ipv6Host<'_>> {
        self.borrowed().ipv6()
    }



    /// Turn into an [`Ipv4Host`].
    pub fn ipv4(self) -> Option<Ipv4Host<'a>> {
        self.try_into().ok()
    }

    /// Turn into an [`Ipv6Host`].
    pub fn ipv6(self) -> Option<Ipv6Host<'a>> {
        self.try_into().ok()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for IpHost<'a> {
    type Error = InvalidIpHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(match value.as_bytes() {
            [b'[', ..] => Self::new_v6(value)?,
            _          => Self::new_v4(value)?,
        })
    }
}

impl From<Ipv4Addr> for IpHost<'static> {fn from(value: Ipv4Addr) -> Self {Self::V4(value.into())}}
impl From<Ipv6Addr> for IpHost<'static> {fn from(value: Ipv6Addr) -> Self {Self::V6(value.into())}}

impl From<IpAddr> for IpHost<'static> {
    fn from(value: IpAddr) -> Self {
        match value {
            IpAddr::V4(x) => x.into(),
            IpAddr::V6(x) => x.into(),
        }
    }
}

impl<'a> From<Ipv4Host<'a>> for IpHost<'a> {fn from(value: Ipv4Host<'a>) -> Self {Self::V4(value)}}
impl<'a> From<Ipv6Host<'a>> for IpHost<'a> {fn from(value: Ipv6Host<'a>) -> Self {Self::V6(value)}}

impl<'a> TryFrom<Host<'a>> for IpHost<'a> {
    type Error = Host<'a>;

    fn try_from(value: Host<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            Host::Ipv4(x) => x.into(),
            Host::Ipv6(x) => x.into(),
            x             => Err(x)?,
        })
    }
}

impl<'a> TryFrom<FileHost<'a>> for IpHost<'a> {
    type Error = FileHost<'a>;

    fn try_from(value: FileHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            FileHost::Ipv4(x) => x.into(),
            FileHost::Ipv6(x) => x.into(),
            x                 => Err(x)?,
        })
    }
}

impl<'a> TryFrom<SpecialNotFileHost<'a>> for IpHost<'a> {
    type Error = SpecialNotFileHost<'a>;

    fn try_from(value: SpecialNotFileHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            SpecialNotFileHost::Ipv4(x) => x.into(),
            SpecialNotFileHost::Ipv6(x) => x.into(),
            x                           => Err(x)?,
        })
    }
}

impl<'a> TryFrom<NonSpecialHost<'a>> for IpHost<'a> {
    type Error = NonSpecialHost<'a>;

    fn try_from(value: NonSpecialHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            NonSpecialHost::Opaque(x) => x.try_into()?,
            NonSpecialHost::Ipv6  (x) => x.into(),
            x                         => Err(x)?,
        })
    }
}

impl<'a> TryFrom<OpaqueHost<'a>> for IpHost<'a> {
    type Error = OpaqueHost<'a>;

    fn try_from(value: OpaqueHost<'a>) -> Result<Self, Self::Error> {
        Self::new_v4(value)
    }
}
