//! [`Normalizer`].

use crate::prelude::*;

/// A [`std::fmt::Write`] that, if fed a prefix of its input, does not allocate.
///
/// Used for things like [`Ipv6Host`] where detecting if an input is normalized is super complicated but always allocating is super stupid.
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
/// assert_matches!(normalizer.done(), (true, Cow::Borrowed("ab")));
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
/// assert_matches!(normalizer.done(), (true, Cow::Owned(x)) if x == "[1::]");
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

        unsafe {
            self.x.truncate_unchecked(self.i);
        }

        (changed, self.x)
    }

    /// What [`Self::done`] would return, without consuming `self`.
    pub fn current_output(&self) -> (bool, &str) {
        let changed = self.c || self.i != self.x.len();

        (changed, unsafe {self.x.get_unchecked(..self.i)})
    }
}

impl<'a> From<Cow<'a, str>> for Normalizer<'a     > {fn from(value: Cow<'a, str>) -> Self {Self {x: value       , i: 0, c: false}}}
impl<'a> From<&'a str     > for Normalizer<'a     > {fn from(value: &'a str     ) -> Self {Self {x: value.into(), i: 0, c: false}}}
impl     From<String      > for Normalizer<'static> {fn from(value: String      ) -> Self {Self {x: value.into(), i: 0, c: false}}}

impl<'a> std::fmt::Write for Normalizer<'a> {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        if self.c || !unsafe {self.x.get_unchecked(self.i..)}.starts_with(s) {
            let a = self.x.to_mut();

            unsafe {
                a.as_mut_vec().set_len(self.i);
            }

            a.push_str(s);

            self.c = true;
        }

        self.i += s.len();

        Ok(())
    }
}
