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
    /// Make a new [`Self`] with zero validity checks.
    /// # Safety
    /// `value` must be a valid empty host literal and `details` must be its [`OpaqueHostDetails`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, details: EmptyHostDetails) -> Self {
        let _ = value.into();

        Self {
            details,
            ..Self::default()
        }
    }

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
        Ok(match value {
            Host::Empty(x) => x,
            x              => Err(x)?,
        })
    }
}

impl<'a> TryFrom<FileHost<'a>> for EmptyHost<'a> {
    type Error = FileHost<'a>;

    fn try_from(value: FileHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            FileHost::Empty(x) => x,
            e                  => Err(e)?,
        })
    }
}

impl<'a> TryFrom<NonSpecialHost<'a>> for EmptyHost<'a> {
    type Error = NonSpecialHost<'a>;

    fn try_from(value: NonSpecialHost<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            NonSpecialHost::Empty(x) => x,
            e                        => Err(e)?,
        })
    }
}
