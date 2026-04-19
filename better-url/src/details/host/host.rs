//! [`HostDetails`].

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::prelude::*;

/// Details for a [`Host`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostDetails {
    /// [`DomainDetails`].
    Domain(DomainDetails),
    /// [`Ipv4Details`].
    Ipv4(Ipv4Details),
    /// [`Ipv6Details`].
    Ipv6(Ipv6Details),
    /// [`OpaqueHostDetails`].
    Opaque(OpaqueHostDetails),
    /// [`EmptyHostDetails`].
    Empty(EmptyHostDetails),
}

impl HostDetails {
    /// Parse a non-opaque host.
    ///
    /// - If `s` is empty, returns [`EmptyHostDetails`].
    ///
    /// - If `s` starts with `[`, attempts to parse as a [`Self::Ipv6`].
    ///
    /// - If `s` ends in a number, attempts to parse as a [`Self::Ipv4`].
    ///
    /// - Otherwise, attempts to parse as a [`Self::Domain`].
    /// # Errors
    /// If the call to [`Ipv6Details::parse`] returns an error, that error is returned.
    ///
    /// If the call to [`Ipv4Details::parse`] returns an error, that error is returned.
    ///
    /// If the call to [`DomainDetails::parse`] returns an error, that error is returned.
    pub fn parse(s: &str) -> Result<Self, InvalidHost> {
        Ok(if s.is_empty() {
            Self::Empty(EmptyHostDetails)
        } else if s.starts_with("[") {
            Self::Ipv6(s.parse()?)
        } else if ends_in_a_number(s) {
            Self::Ipv4(s.parse()?)
        } else {
            Self::Domain(DomainDetails::parse_not_eian(s)?)
        })
    }

    /// Make from [`url::Url`], allowing for skipping some checks.
    pub fn from_url(url: &url::Url) -> Option<Self> {
        Some(match url.host()? {
            url::Host::Domain(x) => match url.is_special() {
                true  => DomainDetails::parse_unchecked(x).into(),
                false => OpaqueHostDetails.into()
            },
            url::Host::Ipv4(x) => x.into(),
            url::Host::Ipv6(x) => x.into(),
        })
    }

    /// If it's [`Self::Domain`].
    pub fn is_domain(self) -> bool {
        matches!(self, Self::Domain(_))
    }

    /// If it's [`Self::Ipv4`].
    pub fn is_ipv4(self) -> bool {
        matches!(self, Self::Ipv4(_))
    }

    /// If it's [`Self::ipv6`].
    pub fn is_ipv6(self) -> bool {
        matches!(self, Self::Ipv6(_))
    }

    /// If it's [`Self::Ipv4`] or [`Self::Ipv6`].
    pub fn is_ip(self) -> bool {
        matches!(self, Self::Ipv4(_) | Self::Ipv6(_))
    }

    /// If it's [`Self::Opaque`].
    pub fn is_opaque(self) -> bool {
        matches!(self, Self::Opaque(_))
    }

    /// If it's [`Self::Empty`].
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty(_))
    }

    /// The [`DomainDetails`].
    pub fn domain(self) -> Option<DomainDetails> {
        self.try_into().ok()
    }

    /// The [`Ipv4Details`].
    pub fn ipv4(self) -> Option<Ipv4Details> {
        self.try_into().ok()
    }

    /// The [`Ipv6Details`].
    pub fn ipv6(self) -> Option<Ipv6Details> {
        self.try_into().ok()
    }

    /// The [`IpDetails`].
    pub fn ip(self) -> Option<IpDetails> {
        self.try_into().ok()
    }

    /// The [`OpaqueHostDetails`].
    pub fn opaque(self) -> Option<OpaqueHostDetails> {
        self.try_into().ok()
    }

    /// The [`EmptyHostDetails`].
    pub fn empty(self) -> Option<EmptyHostDetails> {
        self.try_into().ok()
    }
}

impl From<DomainDetails    > for HostDetails {fn from(value: DomainDetails    ) -> Self {Self::Domain(value)}}
impl From<Ipv4Details      > for HostDetails {fn from(value: Ipv4Details      ) -> Self {Self::Ipv4  (value)}}
impl From<Ipv6Details      > for HostDetails {fn from(value: Ipv6Details      ) -> Self {Self::Ipv6  (value)}}
impl From<OpaqueHostDetails> for HostDetails {fn from(value: OpaqueHostDetails) -> Self {Self::Opaque(value)}}
impl From<EmptyHostDetails > for HostDetails {fn from(value: EmptyHostDetails ) -> Self {Self::Empty (value)}}

impl From<IpDetails> for HostDetails {
    fn from(value: IpDetails) -> Self {
        match value {
            IpDetails::V4(x) => x.into(),
            IpDetails::V6(x) => x.into(),
        }
    }
}

impl From<Ipv4Addr> for HostDetails {fn from(value: Ipv4Addr) -> Self {Self::Ipv4(value.into())}}
impl From<Ipv6Addr> for HostDetails {fn from(value: Ipv6Addr) -> Self {Self::Ipv6(value.into())}}

impl From<IpAddr> for HostDetails {
    fn from(value: IpAddr) -> Self {
        match value {
            IpAddr::V4(addr) => addr.into(),
            IpAddr::V6(addr) => addr.into(),
        }
    }
}

impl TryFrom<&str> for HostDetails {
    type Error = InvalidHost;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl FromStr for HostDetails {
    type Err = InvalidHost;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}
