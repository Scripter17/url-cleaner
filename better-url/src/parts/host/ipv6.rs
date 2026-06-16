//! [`Ipv6Host`].

use std::net::Ipv6Addr;

use crate::prelude::*;

/// An IPv6 host.
#[derive(Debug, Clone)]
pub struct Ipv6Host<'a> {
    /// The host string.
    pub(crate) host: Cow<'a, str>,
    /// The [`Ipv6Details`].
    pub(crate) details: Ipv6Details
}

impl<'a> Ipv6Host<'a> {
    /// The host as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.host
    }

    /// The [`Ipv6Details`].
    pub fn details(&self) -> Ipv6Details {
        self.details
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (Cow<'a, str>, Ipv6Details) {
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
        Ok(Self {
            details: value.parse()?,
            host: value,
        })
    }
}

impl<'a> TryFrom<Host<'a>> for Ipv6Host<'a> {
    type Error = Host<'a>;

    fn try_from(value: Host<'a>) -> Result<Self, Self::Error> {
        match value {
            Host::Ipv6(x) => Ok(x),
            x => Err(x)
        }
    }
}

impl<'a> TryFrom<IpHost<'a>> for Ipv6Host<'a> {
    type Error = Ipv4Host<'a>;

    fn try_from(value: IpHost<'a>) -> Result<Self, Self::Error> {
        match value {
            IpHost::V4(x) => Err(x),
            IpHost::V6(x) => Ok (x),
        }
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
