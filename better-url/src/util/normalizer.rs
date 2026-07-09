//! [`Normalizer`].

use crate::prelude::*;

/// A [`std::fmt::Write`] that, if fed a prefix of its input, does not allocate.
/// # Examples
/// ```
/// use std::fmt::Write;
/// use std::assert_matches;
/// use std::borrow::Cow;
///
/// use better_url::util::*;
///
/// // Prefixes don't get allocated.
///
/// let mut normalizer = Normalizer::new("abc");
///
/// write!(normalizer, "a").unwrap();
/// write!(normalizer, "b").unwrap();
///
/// let (changed, value) = normalizer.done();
///
/// assert!(changed);
/// assert_matches!(value, Cow::Borrowed("ab"));
///
/// // But changes do.
///
/// let mut normalizer = Normalizer::new("[1::0]");
///
/// write!(normalizer, "[" ).unwrap();
/// write!(normalizer, "1" ).unwrap();
/// write!(normalizer, "::").unwrap();
/// write!(normalizer, "]" ).unwrap();
///
/// assert_eq!(normalizer.done(), (true, "[1::]".into()));
/// ```
#[derive(Debug, Clone)]
pub struct Normalizer<'a> {
    /// The serialization.
    x: Cow<'a, str>,
    /// The index to write to.
    i: usize,
    /// If [`Self::x`] has been changed.
    c: bool,
}

impl<'a> Normalizer<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(value: T) -> Self {
        value.into()
    }

    /// Truncate if needed and return the final result.
    pub fn done(mut self) -> (bool, Cow<'a, str>) {
        let changed = self.c || self.i != self.x.len();
        match &mut self.x {
            Cow::Owned   (x) => unsafe {x.as_mut_vec().set_len(self.i);},
            Cow::Borrowed(x) => *x = unsafe {x.get_unchecked(..self.i)}
        }
        (changed, self.x)
    }
}

impl<'a> From<Cow<'a, str>> for Normalizer<'a     > {fn from(value: Cow<'a, str>) -> Self {Self {x: value, i: 0, c: false}}}
impl<'a> From<&'a str     > for Normalizer<'a     > {fn from(value: &'a str     ) -> Self {Cow::from(value).into()}}
impl     From<String      > for Normalizer<'static> {fn from(value: String      ) -> Self {Cow::from(value).into()}}

impl<'a> std::fmt::Write for Normalizer<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        if self.i + s.len() > self.x.len() || unsafe {self.x.get_unchecked(self.i..self.i+s.len())}.bytes().ne(s.bytes()) {
            self.x.to_mut().truncate(self.i);
            write!(self.x.to_mut(), "{s}")?;
            self.c = true;
        }
        self.i += s.len();

        Ok(())
    }
}
