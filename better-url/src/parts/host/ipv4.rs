//! [`Ipv4Host`].

use std::net::Ipv4Addr;

use crate::prelude::*;

/// An IPv4 host.
#[derive(Debug, Clone)]
pub struct Ipv4Host<'a> {
    /// The host string.
    pub(crate) host: Cow<'a, str>,
    /// The [`Ipv4Details`].
    pub(crate) details: Ipv4Details
}

impl<'a> Ipv4Host<'a> {
    /// The host as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.host
    }

    /// The [`Ipv4Details`].
    pub fn details(&self) -> Ipv4Details {
        self.details
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (Cow<'a, str>, Ipv4Details) {
        (self.host, self.details)
    }



    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Ipv4Host<'static> {
        Ipv4Host {
            host: self.host.into_owned().into(),
            details: self.details
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Ipv4Host<'_> {
        Ipv4Host {
            host: Cow::Borrowed(&self.host),
            details: self.details
        }
    }
}

impl From<Ipv4Addr> for Ipv4Host<'static> {
    fn from(value: Ipv4Addr) -> Self {
        Self {
            host: value.to_string().into(),
            details: value.into()
        }
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Ipv4Host<'a> {
    type Error = InvalidIpv4Host;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if let Ok(x) = value.parse::<Ipv4Addr>() {
            Ok(Self {
                details: x.into(),
                host: value
            })
        } else {
            let details = value.parse()?;
            Ok(Self {
                details,
                host: details.parsed.to_string().into(),
            })
        }
    }
}

impl<'a> TryFrom<IpHost<'a>> for Ipv4Host<'a> {
    type Error = Ipv6Host<'a>;

    fn try_from(value: IpHost<'a>) -> Result<Self, Self::Error> {
        match value {
            IpHost::V4(x) => Ok(x),
            IpHost::V6(x) => Err(x),
        }
    }
}

impl<'a> TryFrom<Host<'a>> for Ipv4Host<'a> {
    type Error = Host<'a>;

    fn try_from(value: Host<'a>) -> Result<Self, Self::Error> {
        match value {
            Host::Ipv4(x) => Ok(x),
            x => Err(x)
        }
    }
}
