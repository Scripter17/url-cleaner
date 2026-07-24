//! [`FileHostDetails`].

use crate::prelude::*;

/// Details for a [`FileHost`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileHostDetails {
    /** [`DomainHostDetails`]. **/ Domain(DomainHostDetails),
    /** [`Ipv4HostDetails`].   **/ Ipv4  (Ipv4HostDetails  ),
    /** [`Ipv6HostDetails`].   **/ Ipv6  (Ipv6HostDetails  ),
    /** [`EmptyHostDetails`].  **/ Empty (EmptyHostDetails ),
}

impl FileHostDetails {
    /// Parse from a [`FileHost`] literal.
    /// # Errors
    /// If `value` is not a valid [`FileHost`] literal, returns the error [`InvalidFileHost`].
    pub fn parse(value: &str) -> Result<Self, InvalidFileHost> {
        Ok(match value.as_bytes() {
            b""        => EmptyHostDetails                 .into(),
            [b'[', ..] => Ipv6HostDetails ::parse  (value)?.into(),
            _ => match ends_in_a_number(value) {
                true  => Ipv4HostDetails  ::parse(value)?.into(),
                false => DomainHostDetails::parse(value)?.into(),
            }
        })
    }
}

impl From<DomainHostDetails> for FileHostDetails {fn from(value: DomainHostDetails) -> Self {Self::Domain(value)}}
impl From<Ipv4HostDetails  > for FileHostDetails {fn from(value: Ipv4HostDetails  ) -> Self {Self::Ipv4  (value)}}
impl From<Ipv6HostDetails  > for FileHostDetails {fn from(value: Ipv6HostDetails  ) -> Self {Self::Ipv6  (value)}}
impl From<EmptyHostDetails > for FileHostDetails {fn from(value: EmptyHostDetails ) -> Self {Self::Empty (value)}}
