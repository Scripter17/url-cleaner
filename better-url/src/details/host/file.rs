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

    /** If it's [`Self::Domain`]. **/                 pub fn is_domain(self) -> bool {matches!(self, Self::Domain(_)                )}
    /** If it's [`Self::Ipv4`]. **/                   pub fn is_ipv4  (self) -> bool {matches!(self, Self::Ipv4  (_)                )}
    /** If it's [`Self::ipv6`]. **/                   pub fn is_ipv6  (self) -> bool {matches!(self, Self::Ipv6  (_)                )}
    /** If it's [`Self::Ipv4`] or [`Self::Ipv6`]. **/ pub fn is_ip    (self) -> bool {matches!(self, Self::Ipv4  (_) | Self::Ipv6(_))}
    /** If it's [`Self::Empty`]. **/                  pub fn is_empty (self) -> bool {matches!(self, Self::Empty (_)                )}

    /** The [`DomainHostDetails`]. **/ pub fn domain(self) -> Option<DomainHostDetails> {self.try_into().ok()}
    /** The [`Ipv4HostDetails`].   **/ pub fn ipv4  (self) -> Option<Ipv4HostDetails  > {self.try_into().ok()}
    /** The [`Ipv6HostDetails`].   **/ pub fn ipv6  (self) -> Option<Ipv6HostDetails  > {self.try_into().ok()}
    /** The [`EmptyHostDetails`].  **/ pub fn empty (self) -> Option<EmptyHostDetails > {self.try_into().ok()}
}

impl From<DomainHostDetails> for FileHostDetails {fn from(value: DomainHostDetails) -> Self {Self::Domain(value)}}
impl From<Ipv4HostDetails  > for FileHostDetails {fn from(value: Ipv4HostDetails  ) -> Self {Self::Ipv4  (value)}}
impl From<Ipv6HostDetails  > for FileHostDetails {fn from(value: Ipv6HostDetails  ) -> Self {Self::Ipv6  (value)}}
impl From<EmptyHostDetails > for FileHostDetails {fn from(value: EmptyHostDetails ) -> Self {Self::Empty (value)}}
