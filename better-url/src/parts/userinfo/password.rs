//! [`Password`].

use crate::prelude::*;

/// A password.
#[derive(Debug, Clone)]
pub struct Password<'a>(pub(crate) Cow<'a, str>);

impl<'a> Password<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(username: T) -> Self {
        Self(username.into())
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Password<'static> {
        Password(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Password<'_> {
        Password(Cow::Borrowed(&self.0))
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> From<Cow<'a, str>> for Password<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(encode_password(value).1)
    }
}
