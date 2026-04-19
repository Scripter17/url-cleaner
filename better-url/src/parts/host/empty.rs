//! [`EmptyHost`].

use crate::prelude::*;

/// The empty host.
#[derive(Debug, Clone, Copy, Default)]
pub struct EmptyHost<'a> {
    /// Fake having a host string.
    pub(crate) phantom: std::marker::PhantomData<&'a ()>,
    /// The [`EmptyHostDetails`].
    pub(crate) details: EmptyHostDetails
}

impl<'a> EmptyHost<'a> {
    /// The host as a [`str`].
    pub fn as_str(&self) -> &str {
        ""
    }

    /// The [`OpaqueHostDetails`].
    pub fn details(&self) -> EmptyHostDetails {
        self.details
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (Cow<'a, str>, EmptyHostDetails) {
        (Cow::Borrowed(""), self.details)
    }



    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> EmptyHost<'static> {
        EmptyHost::default()
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> EmptyHost<'_> {
        EmptyHost::default()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for EmptyHost<'a> {
    type Error = InvalidEmptyHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(Self {
            details: value.parse()?,
            phantom: Default::default()
        })
    }
}

impl<'a> TryFrom<Host<'a>> for EmptyHost<'a> {
    type Error = Host<'a>;

    fn try_from(value: Host<'a>) -> Result<Self, Self::Error> {
        match value {
            Host::Empty(x) => Ok(x),
            x => Err(x),
        }
    }
}
