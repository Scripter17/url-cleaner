//! [`Userinfo`].

use crate::prelude::*;

mod username;
mod password;

/// A userinfo.
/// # Examples
/// ```
/// use better_url::prelude::*;
/// 
/// assert_eq!(Userinfo::from("abc:def:ghi"), "abc:def%3Aghi");
/// ```
#[derive(Debug, Clone)]
pub struct Userinfo<'a> {
    /// The raw string.
    pub(crate) raw: Cow<'a, str>,
    /// If non-zero, the [`Range::start`] of the password.
    pub(crate) ps: Option<NonZero<usize>>,
}

impl<'a> Userinfo<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal and `ps` must be the index just after the first `:`, if any.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, ps: Option<NonZero<usize>>) -> Self {
        Self {
            raw: value.into(),
            ps
        }
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Userinfo<'static> {
        Userinfo {
            raw: self.raw.into_owned().into(),
            ps: self.ps
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Userinfo<'_> {
        Userinfo {
            raw: Cow::Borrowed(&self.raw),
            ps: self.ps
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.raw
    }
}

impl<'a> From<Cow<'a, str>> for Userinfo<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, raw, ps) = encode_userinfo(value);

        unsafe {
            Self::new_unchecked(raw, ps)
        }
    }
}
