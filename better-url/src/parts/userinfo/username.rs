//! [`Username`].

use crate::prelude::*;

/// A username.
#[derive(Debug, Clone)]
pub struct Username<'a>(pub(crate) Cow<'a, str>);

impl<'a> Username<'a> {
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



    /// [`lossy_decode_username`].
    pub fn lossy_decode(self) -> Cow<'a, str> {
        let (_, value) = lossy_decode_username(self.0);

        value
    }

    /// [`try_decode_username`].
    /// # Errors
    /// If the call to [`try_decode_username`] returns an error, that error is returned.
    pub fn try_decode(self) -> Result<Cow<'a, str>, Cow<'a, [u8]>> {
        match try_decode_username(self.0) {
            Ok ((_, value)) => Ok (value),
            Err((_, value)) => Err(value),
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Username<'_> {
        Username(Cow::Borrowed(&self.0))
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Username<'static> {
        Username(self.0.into_owned().into())
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> From<Cow<'a, str>> for Username<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, value) = encode_username(value);

        unsafe {
            Self::new_unchecked(value)
        }
    }
}
