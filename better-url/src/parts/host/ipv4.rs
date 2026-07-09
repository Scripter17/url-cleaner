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
    /// Make a new [`Self`] with zero validity checks.
    /// # Safety
    /// `value` must be a valid IPv4 host literal and `details` must be its [`Ipv4Details`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, details: Ipv4Details) -> Self {
        Self {
            host: value.into(),
            details,
        }
    }

    /// Make a new [`Self`] from a percent decoded value.
    /// # Errors
    /// If the call to [`Self::new_normalized`] returns an error, that error is returned.
    pub fn new_percent_decoded<T: Into<Cow<'a, str>>>(value: T) -> Result<Self, InvalidIpv4Host> {
        let (_, value) = uts46_map_normalize(value);
        Self::new_normalized(value)
    }

    /// Make a new [`Self`] from a percent decoded and UTS46 normalized value.
    /// # Errors
    /// If the call to [`parse_ipv4_host`] returns an error, that error is returned.
    pub fn new_normalized<T: Into<Cow<'a, str>>>(value: T) -> Result<Self, InvalidIpv4Host> {
        let (_, addr, host) = make_ipv4_host(value)?;

        Ok(Self {
            host,
            details: addr.into(),
        })
    }

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

impl<'a> TryFrom<Cow<'a, str>> for Ipv4Host<'a> {
    type Error = InvalidIpv4Host;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let (_, value) = try_percent_decode(value).map_err(|_| InvalidIpv4Host)?; // TODO: Fix.

        Self::new_percent_decoded(value)
    }
}

impl<'a> TryFrom<Host<'a>> for Ipv4Host<'a> {
    type Error = Host<'a>;

    fn try_from(value: Host<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            Host::Ipv4(x) => x,
            x             => Err(x)?,
        })
    }
}

impl<'a> TryFrom<FileHost<'a>> for Ipv4Host<'a> {
    type Error = FileHost<'a>;

    fn try_from(value: FileHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            FileHost::Ipv4(x) => x,
            x                 => Err(x)?,
        })
    }
}

impl<'a> TryFrom<SpecialNotFileHost<'a>> for Ipv4Host<'a> {
    type Error = SpecialNotFileHost<'a>;

    fn try_from(value: SpecialNotFileHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            SpecialNotFileHost::Ipv4(x) => x,
            x                           => Err(x)?,
        })
    }
}

impl<'a> TryFrom<NonSpecialHost<'a>> for Ipv4Host<'a> {
    type Error = NonSpecialHost<'a>;

    fn try_from(value: NonSpecialHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            NonSpecialHost::Opaque(x) => x.try_into()?,
            x                         => Err(x)?,
        })
    }
}

impl<'a> TryFrom<IpHost<'a>> for Ipv4Host<'a> {
    type Error = Ipv6Host<'a>;

    fn try_from(value: IpHost<'a>) -> Result<Self, Self::Error> {
        match value {
            IpHost::V4(x) => Ok (x),
            IpHost::V6(x) => Err(x),
        }
    }
}



impl<'a> TryFrom<OpaqueHost<'a>> for Ipv4Host<'a> {
    type Error = OpaqueHost<'a>;

    fn try_from(value: OpaqueHost<'a>) -> Result<Self, Self::Error> {
        // TODO: This is dumb.

        let (host, _) = value.clone().into_parts();

        host.try_into().map_err(|_| value)
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
