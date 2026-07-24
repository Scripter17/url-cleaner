//! [`NonSpecialHostDetails`].

use crate::prelude::*;

/// Details for a [`NonSpecialHost`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonSpecialHostDetails {
    /** [`Ipv6HostDetails`].   **/ Ipv6  (Ipv6HostDetails  ),
    /** [`OpaqueHostDetails`]. **/ Opaque(OpaqueHostDetails),
    /** [`EmptyHostDetails`].  **/ Empty (EmptyHostDetails ),
}

impl NonSpecialHostDetails {
    /// Parse from a [`NonSpecialHost`] literal.
    /// # Errors
    /// If `value` is not a valid [`NonSpecialHost`] literal, returns the error [`InvalidNonSpecialHost`].
    pub fn parse(value: &str) -> Result<Self, InvalidNonSpecialHost> {
        Ok(match value.as_bytes() {
            b""        => EmptyHostDetails                  .into(),
            [b'[', ..] => Ipv6HostDetails  ::parse  (value)?.into(),
            _          => OpaqueHostDetails::parse  (value)?.into(),
        })
    }
}

impl From<Ipv6HostDetails  > for NonSpecialHostDetails {fn from(value: Ipv6HostDetails  ) -> Self {Self::Ipv6  (value)}}
impl From<OpaqueHostDetails> for NonSpecialHostDetails {fn from(value: OpaqueHostDetails) -> Self {Self::Opaque(value)}}
impl From<EmptyHostDetails > for NonSpecialHostDetails {fn from(value: EmptyHostDetails ) -> Self {Self::Empty (value)}}
