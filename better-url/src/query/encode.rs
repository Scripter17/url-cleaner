//! [`QueryPartEncoder`].

use std::borrow::Cow;

use crate::prelude::*;

/// An [`Iterator`] yielding [`str`] slices from encoding [`Self::0`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueryPartEncoder<'a>(pub &'a [u8]);

impl<'a> QueryPartEncoder<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(part: T) -> Self {
        part.into()
    }

    /// Shorthand for [`Self::new`] and [`Iterator::collect`].
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use better_url::prelude::*;
    ///
    /// assert_eq!(QueryPartEncoder::new("abc\u{202e}def").encode(), "abc%E2%80%AEdef");
    /// assert_eq!(QueryPartEncoder::new(b"abc\x00def"   ).encode(), "abc%00def");
    ///
    /// assert!(matches!(QueryPartEncoder::new("abc").encode(), Cow::Borrowed(_)));
    /// ```
    pub fn encode(mut self) -> Cow<'a, str> {
        let mut ret = Cow::Borrowed(self.next().unwrap_or(""));

        for chunk in self {
            ret.to_mut().push_str(chunk);
        }

        ret
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> From<&'a T> for QueryPartEncoder<'a> {
    fn from(value: &'a T) -> Self {
        Self(value.as_ref())
    }
}

impl<T: AsRef<[u8]> + ?Sized> PartialEq<T> for QueryPartEncoder<'_> {
    fn eq(&self, other: &T) -> bool {
        self.flat_map(str::bytes).eq(other.as_ref().iter().copied())
    }
}

/// Returns [`true`] if `b` should be encoded.
fn changed(b: u8) -> bool {
    PercentEncodeSet::FormUrlencoded.matches(b)
}

/// Encodes `b`.
fn change(b: u8) -> &'static str {
    match b {
        b' ' => "+",
        b => percent_encode(b)
    }
}

impl<'a> Iterator for QueryPartEncoder<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.0.split_first()? {
            (&b, rest) if changed(b) => {self.0 = rest; change(b)},
            _ => {
                let split = self.0.iter().copied().position(changed).unwrap_or(self.0.len());
                let (ret, rest) = self.0.split_at(split);
                self.0 = rest;
                // SAFETY:
                // 1. `ret` only contains bytes for which `unchanged` returns false.
                // 2. `unchanged` only returns false for certain ASCII bytes.
                // Therefore, `ret` only contains ASCII bytes.
                unsafe {str::from_utf8_unchecked(ret)}
            }
        })
    }
}

impl std::fmt::Display for QueryPartEncoder<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in *self {
            write!(formatter, "{x}")?;
        }
        Ok(())
    }
}
