//! [`MaybePort`].

use crate::prelude::*;

/// A maybe port.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MaybePort<'a>(pub Option<Port<'a>>);

impl<'a> MaybePort<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_ref().map(Port::as_str)
    }

    /// Get it as a [`u16`].
    pub fn as_u16(&self) -> Option<u16> {
        self.0.as_ref().map(Port::as_u16)
    }
}

impl<'a> TryFrom<Option<Cow<'a, str>>> for MaybePort<'a> {
    type Error = InvalidPort;

    fn try_from(value: Option<Cow<'a, str>>) -> Result<Self, Self::Error> {
        value.map(TryInto::try_into).transpose().map(Self)
    }
}
