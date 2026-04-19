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
    pub(crate) ps: usize,
}

impl<'a> Userinfo<'a> {
    /// Make a new [`Self`] without checking for validity.
    pub(crate) fn new_unchecked<T: Into<Cow<'a, str>>>(userinfo: T) -> Self {
        let raw = userinfo.into();

        Self {
            ps: match raw.split_once(':') {
                Some((x, _)) => x.len() + 1,
                None => 0
            },
            raw
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
        let mut raw = PartTranscoder::UserinfoPart.encode(value);

        match raw.split_once("%3A") {
            Some((x, _)) => {
                let ps = x.len() + 1;
                raw.replace_range(x.len()..=x.len()+2, ":");
                Self {raw, ps}
            },
            None => Self {raw, ps: 0}
        }
    }
}
