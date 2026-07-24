//! [`Userinfo`].

use crate::prelude::*;

mod username;
mod password;

/// A userinfo.
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// let userinfo = Userinfo::from("username:password:also_password");
///
/// assert_eq!(userinfo, "username:password%3Aalso_password");
///
/// assert_eq!(userinfo.username(), "username");
///
/// assert_eq!(userinfo.password(), "password%3Aalso_password");
/// ```
#[derive(Debug, Clone)]
pub struct Userinfo<'a> {
    /// The userinfo string.
    pub(crate) userinfo: Cow<'a, str>,
    /// The [`Range::start`] of the password.
    pub(crate) password_start: Option<NonZero<usize>>,
}

impl<'a> Userinfo<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal and `password_start` must be the index just after the first `:`, if any.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, password_start: Option<NonZero<usize>>) -> Self {
        Self {
            userinfo: value.into(),
            password_start
        }
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.userinfo
    }



    /// If it's visible in a URL.
    pub fn is_visible(&self) -> bool {
        !self.is_empty()
    }



    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Userinfo<'static> {
        Userinfo {
            userinfo: self.userinfo.into_owned().into(),
            password_start: self.password_start
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Userinfo<'_> {
        Userinfo {
            userinfo: Cow::Borrowed(&self.userinfo),
            password_start: self.password_start
        }
    }

    /// Turn into the inner [`Cow`] and [`NonZero`] [`usize`].
    pub fn into_parts(self) -> (Cow<'a, str>, Option<NonZero<usize>>) {
        (self.userinfo, self.password_start)
    }
}

impl<'a> From<Cow<'a, str>> for Userinfo<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        let (_, userinfo, password_start) = encode_userinfo(value);

        unsafe {
            Self::new_unchecked(userinfo, password_start)
        }
    }
}
