//! [`BetterRefQuery`].

use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::prelude::*;

mod get;
mod remove;

/// A borrowed query string.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BetterRefQuery<'a>(pub &'a str);

impl<'a> BetterRefQuery<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(query: T) -> Self {
        query.into()
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &'a str {
        self.0
    }

    /// A [`DoubleEndedIterator`] over the [`RawQuerySegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = RawQuerySegment<'a>> {
        self.0.split('&').map(Into::into)
    }

    /// The `index`th [`Self::splits`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let query = BetterRefQuery::new("0&1&2&3");
    ///
    /// assert_eq!(query.split(0), Some((None,            Some("0&1&2&3"))), "a 0");
    /// assert_eq!(query.split(1), Some((Some("0"      ), Some("1&2&3"  ))), "a 1");
    /// assert_eq!(query.split(2), Some((Some("0&1"    ), Some("2&3"    ))), "a 2");
    /// assert_eq!(query.split(3), Some((Some("0&1&2"  ), Some("3"      ))), "a 3");
    /// assert_eq!(query.split(4), Some((Some("0&1&2&3"), None           )), "a 4");
    /// assert_eq!(query.split(5), None                                    , "a 5");
    ///
    /// assert_eq!(query.split(-1), Some((Some("0&1&2&3"), None           )), "a -1");
    /// assert_eq!(query.split(-2), Some((Some("0&1&2"  ), Some("3"      ))), "a -2");
    /// assert_eq!(query.split(-3), Some((Some("0&1"    ), Some("2&3"    ))), "a -3");
    /// assert_eq!(query.split(-4), Some((Some("0"      ), Some("1&2&3"  ))), "a -4");
    /// assert_eq!(query.split(-5), Some((None,            Some("0&1&2&3"))), "a -5");
    /// assert_eq!(query.split(-6), None                                    , "a -6");
    ///
    /// let query = BetterRefQuery::new("0");
    ///
    /// assert_eq!(query.split( 0), Some((None     , Some("0"))), "b 0");
    /// assert_eq!(query.split( 1), Some((Some("0"), None     )), "b 1");
    /// assert_eq!(query.split( 2), None                        , "b 2");
    ///
    /// assert_eq!(query.split(-1), Some((Some("0"), None     )), "b -1");
    /// assert_eq!(query.split(-2), Some((None     , Some("0"))), "b -2");
    /// assert_eq!(query.split(-3), None                        , "b -3");
    ///
    /// let query = BetterRefQuery::new("");
    ///
    /// assert_eq!(query.split( 0), Some((None    , Some(""))), "c 0");
    /// assert_eq!(query.split( 1), Some((Some(""), None    )), "c 1");
    /// assert_eq!(query.split( 2), None                      , "c 2");
    ///
    /// assert_eq!(query.split(-1), Some((Some(""), None    )), "c -1");
    /// assert_eq!(query.split(-2), Some((None    , Some(""))), "c -2");
    /// assert_eq!(query.split(-3), None                      , "c -3");
    /// ```
    pub fn split(&self, index: isize) -> Option<(Option<&'a str>, Option<&'a str>)> {
        self.splits().neg_nth(index)
    }

    /// A [`DoubleEndedIterator`] of every "before" and "after of a place where a segment can be inserted.
    pub fn splits(&self) -> impl DoubleEndedIterator<Item = (Option<&'a str>, Option<&'a str>)> {
        self.iter().map(|segment| {
            let offset = segment.0.addr() - self.0.addr();

            if offset == 0 {
                (None, Some(&self.0[offset..]))
            } else {
                (Some(&self.0[..offset - 1]), Some(&self.0[offset..]))
            }
        }).chain(std::iter::once((Some(self.as_str()), None)))
    }
}

impl<'a> From<&'a str            > for BetterRefQuery<'a> {fn from(value: &'a str            ) -> Self {Self(value)}}
impl<'a> From<RawQuerySegment<'a>> for BetterRefQuery<'a> {fn from(value: RawQuerySegment<'a>) -> Self {value.0.into()}}

impl PartialEq<&str        > for BetterRefQuery<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_str() == *other}}
impl PartialEq<Cow<'_, str>> for BetterRefQuery<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_str() == *other}}
impl PartialEq<String      > for BetterRefQuery<'_> {fn eq(&self, other: &String      ) -> bool {self.as_str() == *other}}

impl PartialEq<Option<&str>        > for BetterRefQuery<'_> {fn eq(&self, other: &Option<&str        >) -> bool {Some(self.as_str()) == other.as_deref()}}
impl PartialEq<Option<String>      > for BetterRefQuery<'_> {fn eq(&self, other: &Option<String      >) -> bool {Some(self.as_str()) == other.as_deref()}}
impl PartialEq<Option<Cow<'_, str>>> for BetterRefQuery<'_> {fn eq(&self, other: &Option<Cow<'_, str>>) -> bool {Some(self.as_str()) == other.as_deref()}}

impl PartialEq<RawQuerySegment    <'_>> for BetterRefQuery<'_> {fn eq(&self, other: &RawQuerySegment    <'_>) -> bool {     self.as_str()  == other.as_str()}}
impl PartialEq<BetterQuery        <'_>> for BetterRefQuery<'_> {fn eq(&self, other: &BetterQuery        <'_>) -> bool {     self.as_str()  == other.as_str()}}
impl PartialEq<BetterMaybeRefQuery<'_>> for BetterRefQuery<'_> {fn eq(&self, other: &BetterMaybeRefQuery<'_>) -> bool {Some(self.as_str()) == other.as_option_str()}}
impl PartialEq<BetterMaybeQuery   <'_>> for BetterRefQuery<'_> {fn eq(&self, other: &BetterMaybeQuery   <'_>) -> bool {Some(self.as_str()) == other.as_option_str()}}

impl std::fmt::Display for BetterRefQuery<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
