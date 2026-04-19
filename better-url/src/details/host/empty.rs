//! [`EmptyHostDetails`].

use crate::prelude::*;

/// Details for the [`EmptyHost`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmptyHostDetails;

impl EmptyHostDetails {
    /// Parse the empty host.
    /// # Errors
    /// If the host isn't empty, returns the error [`InvalidEmptyHost`].
    pub fn parse(s: &str) -> Result<Self, InvalidEmptyHost> {
        match s {
            "" => Ok(Self),
            _  => Err(InvalidEmptyHost),
        }
    }
}

impl FromStr for EmptyHostDetails {
    type Err = InvalidEmptyHost;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for EmptyHostDetails {
    type Error = InvalidEmptyHost;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<HostDetails> for EmptyHostDetails {
    type Error = HostDetails;

    fn try_from(value: HostDetails) -> Result<Self, Self::Error> {
        match value {
            HostDetails::Empty(details) => Ok(details),
            details => Err(details)
        }
    }
}
