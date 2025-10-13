//! Details of an IP host.

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

mod v4;
pub use v4::*;
mod v6;
pub use v6::*;

use super::*;

/// Either an [`Ipv4Details`] or an [`Ipv6Details`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub enum IpDetails {
    /// [`Ipv4Details`].
    V4(Ipv4Details),
    /// [`Ipv6Details`].
    V6(Ipv6Details)
}

impl IpDetails {
    /// If `self` is [`Self::V4`], return the [`Ipv4Details`].
    pub fn ipv4_details(&self) -> Option<&Ipv4Details> {
        match self {
            Self::V4(details) => Some(details),
            _ => None
        }
    }

    /// If `self` is [`Self::V6`], return the [`Ipv6Details`].
    pub fn ipv6_details(&self) -> Option<&Ipv6Details> {
        match self {
            Self::V6(details) => Some(details),
            _ => None
        }
    }

    /// Return [`true`] if `self` is [`Self::V4`].
    pub fn is_ipv4(&self) -> bool {
        matches!(self, Self::V4(_))
    }

    /// Return [`true`] if `self` is [`Self::V6`].
    pub fn is_ipv6(&self) -> bool {
        matches!(self, Self::V6(_))
    }

    // /// [`Ipv4Details::is_benchmarking`] or [`Ipv6Details::is_benchmarking`].
    // pub fn is_benchmarking(&self) -> bool {
    //     match self {
    //         Self::V4(details) => details.is_benchmarking(),
    //         Self::V6(details) => details.is_benchmarking()
    //     }
    // }

    // /// [`Ipv4Details::is_documentation`] or [`Ipv6Details::is_documentation`].
    // pub fn is_documentation(&self) -> bool {
    //     match self {
    //         Self::V4(details) => details.is_documentation(),
    //         Self::V6(details) => details.is_documentation()
    //     }
    // }

    // /// [`Ipv4Details::is_global`] or [`Ipv6Details::is_global`].
    // pub fn is_global(&self) -> bool {
    //     match self {
    //         Self::V4(details) => details.is_global(),
    //         Self::V6(details) => details.is_global()
    //     }
    // }

    /// [`Ipv4Details::is_loopback`] or [`Ipv6Details::is_loopback`].
    pub fn is_loopback(&self) -> bool {
        match self {
            Self::V4(details) => details.is_loopback(),
            Self::V6(details) => details.is_loopback()
        }
    }

    /// [`Ipv4Details::is_multicast`] or [`Ipv6Details::is_multicast`].
    pub fn is_multicast(&self) -> bool {
        match self {
            Self::V4(details) => details.is_multicast(),
            Self::V6(details) => details.is_multicast()
        }
    }

    /// [`Ipv4Details::is_unspecified`] or [`Ipv6Details::is_unspecified`].
    pub fn is_unspecified(&self) -> bool {
        match self {
            Self::V4(details) => details.is_unspecified(),
            Self::V6(details) => details.is_unspecified()
        }
    }
}

impl From<Ipv4Details> for IpDetails {
    fn from(value: Ipv4Details) -> IpDetails {
        Self::V4(value)
    }
}

impl From<Ipv6Details> for IpDetails {
    fn from(value: Ipv6Details) -> IpDetails {
        Self::V6(value)
    }
}

impl TryFrom<IpDetails> for Ipv4Details {
    type Error = HostIsNotIpv4;

    fn try_from(value: IpDetails) -> Result<Self, Self::Error> {
        match value {
            IpDetails::V4(details) => Ok(details),
            _ => Err(HostIsNotIpv4)
        }
    }
}
impl TryFrom<IpDetails> for Ipv6Details {
    type Error = HostIsNotIpv6;

    fn try_from(value: IpDetails) -> Result<Self, Self::Error> {
        match value {
            IpDetails::V6(details) => Ok(details),
            _ => Err(HostIsNotIpv6)
        }
    }
}
