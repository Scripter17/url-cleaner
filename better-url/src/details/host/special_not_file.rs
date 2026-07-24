//! [`SpecialNotFileHostDetails`].

use crate::prelude::*;

/// Details for a [`SpecialNotFileHost`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialNotFileHostDetails {
    /** [`DomainHostDetails`]. **/ Domain(DomainHostDetails),
    /** [`Ipv4HostDetails`].   **/ Ipv4  (Ipv4HostDetails  ),
    /** [`Ipv6HostDetails`].   **/ Ipv6  (Ipv6HostDetails  ),
}

impl SpecialNotFileHostDetails {
    /// Parse from a [`SpecialNotFileHost`] literal.
    /// # Errors
    /// If `value` is not a valid [`SpecialNotFileHost`] literal, returns the error [`InvalidSpecialNotFileHost`].
    pub fn parse(value: &str) -> Result<Self, InvalidSpecialNotFileHost> {
        Ok(match value.as_bytes() {
            [b'[', ..] => Ipv6HostDetails::parse(value)?.into(),
            _ => match ends_in_a_number(value) {
                true  => Ipv4HostDetails  ::parse(value)?.into(),
                false => DomainHostDetails::parse(value)?.into(),
            }
        })
    }
}

impl From<DomainHostDetails> for SpecialNotFileHostDetails {fn from(value: DomainHostDetails) -> Self {Self::Domain(value)}}
impl From<Ipv4HostDetails  > for SpecialNotFileHostDetails {fn from(value: Ipv4HostDetails  ) -> Self {Self::Ipv4  (value)}}
impl From<Ipv6HostDetails  > for SpecialNotFileHostDetails {fn from(value: Ipv6HostDetails  ) -> Self {Self::Ipv6  (value)}}
