//! [`Username`].

use crate::prelude::*;

/// A username.
#[derive(Debug, Clone)]
pub struct Username<'a>(pub(crate) Cow<'a, str>);

impl<'a> Username<'a> {
    /// Make a new [`Self`] without checking for validity.
    pub(crate) fn new_unchecked<T: Into<Cow<'a, str>>>(username: T) -> Self {
        Self(username.into())
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Username<'static> {
        Username(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Username<'_> {
        Username(Cow::Borrowed(&self.0))
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> From<Cow<'a, str>> for Username<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(encode_username(value).1)
    }
}
