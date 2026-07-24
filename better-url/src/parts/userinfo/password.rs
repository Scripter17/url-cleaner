//! [`Password`].

use crate::prelude::*;

/// A password.
#[derive(Debug, Clone)]
pub struct Password<'a>(pub(crate) Cow<'a, str>);

impl<'a> Password<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        Self(value.into())
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }



    /// [`lossy_decode_password`].
    pub fn lossy_decode(self) -> Cow<'a, str> {
        let (_, value) = lossy_decode_password(self.0);

        value
    }

    /// [`try_decode_password`].
    /// # Errors
    /// If the call to [`try_decode_password`] returns an error, that error is returned.
    pub fn try_decode(self) -> Result<Cow<'a, str>, Cow<'a, [u8]>> {
        match try_decode_password(self.0) {
            Ok ((_, value)) => Ok (value),
            Err((_, value)) => Err(value),
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Password<'_> {
        Password(Cow::Borrowed(&self.0))
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Password<'static> {
        Password(self.0.into_owned().into())
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> From<Cow<'a, str>> for Password<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, value) = encode_password(value);

        unsafe {
            Self::new_unchecked(value)
        }
    }
}
