//! [`Ipv6Host`].

use std::net::Ipv6Addr;

use crate::prelude::*;

/// An IPv6 host.
#[derive(Debug, Clone)]
pub struct Ipv6Host<'a> {
    /// The host string.
    pub(crate) host: Cow<'a, str>,
    /// The [`Ipv6HostDetails`].
    pub(crate) details: Ipv6HostDetails
}

impl<'a> Ipv6Host<'a> {
    /// Make a new [`Self`] with zero validity checks.
    /// # Safety
    /// `value` must be a valid IPv6 host literal and `details` must be its [`Ipv6HostDetails`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, details: Ipv6HostDetails) -> Self {
        Self {
            host: value.into(),
            details,
        }
    }

    /// The host as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.host
    }

    /// The [`Ipv6HostDetails`].
    pub fn details(&self) -> Ipv6HostDetails {
        self.details
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (Cow<'a, str>, Ipv6HostDetails) {
        (self.host, self.details)
    }



    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Ipv6Host<'static> {
        Ipv6Host {
            host: self.host.into_owned().into(),
            details: self.details
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Ipv6Host<'_> {
        Ipv6Host {
            host: Cow::Borrowed(&self.host),
            details: self.details
        }
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Ipv6Host<'a> {
    type Error = InvalidIpv6Host;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let (_, addr, host) = make_ipv6_host(value)?;

        Ok(Self {
            host,
            details: addr.into(),
        })
    }
}

impl<'a> TryFrom<Host<'a>> for Ipv6Host<'a> {
    type Error = Host<'a>;

    fn try_from(value: Host<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            Host::Ipv6(x) => x,
            x             => Err(x)?,
        })
    }
}

impl<'a> TryFrom<FileHost<'a>> for Ipv6Host<'a> {
    type Error = FileHost<'a>;

    fn try_from(value: FileHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            FileHost::Ipv6(x) => x,
            x                 => Err(x)?,
        })
    }
}

impl<'a> TryFrom<SpecialNotFileHost<'a>> for Ipv6Host<'a> {
    type Error = SpecialNotFileHost<'a>;

    fn try_from(value: SpecialNotFileHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            SpecialNotFileHost::Ipv6(x) => x,
            x                           => Err(x)?,
        })
    }
}

impl<'a> TryFrom<NonSpecialHost<'a>> for Ipv6Host<'a> {
    type Error = NonSpecialHost<'a>;

    fn try_from(value: NonSpecialHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            NonSpecialHost::Ipv6(x) => x,
            x                       => Err(x)?,
        })
    }
}

impl From<Ipv6Addr> for Ipv6Host<'static> {
    fn from(value: Ipv6Addr) -> Self {
        Self {
            host: format!("[{value}]").into(),
            details: value.into()
        }
    }
}
