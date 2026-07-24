//! [`HostDetails`].

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::prelude::*;

/// Details for a [`Host`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HostDetails {
    /** [`DomainHostDetails`]. **/ Domain(DomainHostDetails),
    /** [`Ipv4HostDetails`].   **/ Ipv4  (Ipv4HostDetails  ),
    /** [`Ipv6HostDetails`].   **/ Ipv6  (Ipv6HostDetails  ),
    /** [`OpaqueHostDetails`]. **/ Opaque(OpaqueHostDetails),
    /** [`EmptyHostDetails`].  **/ Empty (EmptyHostDetails ),
}

impl HostDetails {
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



    /// The [`DomainHostDetails`].
    pub fn domain(self) -> Option<DomainHostDetails> {
        self.try_into().ok()
    }

    /// The [`Ipv4HostDetails`].
    pub fn ipv4(self) -> Option<Ipv4HostDetails> {
        self.try_into().ok()
    }

    /// The [`Ipv6HostDetails`].
    pub fn ipv6(self) -> Option<Ipv6HostDetails> {
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

impl From<FileHostDetails> for HostDetails {
    fn from(value: FileHostDetails) -> Self {
        match value {
            FileHostDetails::Domain(x) => x.into(),
            FileHostDetails::Ipv4  (x) => x.into(),
            FileHostDetails::Ipv6  (x) => x.into(),
            FileHostDetails::Empty (x) => x.into(),
        }
    }
}

impl From<SpecialNotFileHostDetails> for HostDetails {
    fn from(value: SpecialNotFileHostDetails) -> Self {
        match value {
            SpecialNotFileHostDetails::Domain(x) => x.into(),
            SpecialNotFileHostDetails::Ipv4  (x) => x.into(),
            SpecialNotFileHostDetails::Ipv6  (x) => x.into(),
        }
    }
}

impl From<NonSpecialHostDetails> for HostDetails {
    fn from(value: NonSpecialHostDetails) -> Self {
        match value {
            NonSpecialHostDetails::Ipv6  (x) => x.into(),
            NonSpecialHostDetails::Opaque(x) => x.into(),
            NonSpecialHostDetails::Empty (x) => x.into(),
        }
    }
}

impl From<DomainHostDetails> for HostDetails {fn from(value: DomainHostDetails) -> Self {Self::Domain(value)}}
impl From<Ipv4HostDetails  > for HostDetails {fn from(value: Ipv4HostDetails  ) -> Self {Self::Ipv4  (value)}}
impl From<Ipv6HostDetails  > for HostDetails {fn from(value: Ipv6HostDetails  ) -> Self {Self::Ipv6  (value)}}
impl From<OpaqueHostDetails> for HostDetails {fn from(value: OpaqueHostDetails) -> Self {Self::Opaque(value)}}
impl From<EmptyHostDetails > for HostDetails {fn from(value: EmptyHostDetails ) -> Self {Self::Empty (value)}}

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
