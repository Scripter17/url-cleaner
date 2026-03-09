//! [`BetterQuery`].

use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::prelude::*;

mod get;
mod set;
mod remove;
mod extend;

/// A query string.
#[repr(transparent)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BetterQuery<'a>(pub Cow<'a, str>);

impl BetterQuery<'_> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(query: T) -> Self {
        query.into()
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Make a [`BetterRefQuery`].
    pub fn as_ref_query(&self) -> BetterRefQuery<'_> {
        self.as_str().into()
    }

    /// Convert into a [`String`].
    pub fn into_string(self) -> String {
        self.0.into_owned()
    }

    /// Convert into an owned [`Self`].
    pub fn into_owned(self) -> BetterQuery<'static> {
        self.into_string().into()
    }

    /// A [`DoubleEndedIterator`] over the [`RawQuerySegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = RawQuerySegment<'_>> {
        self.0.split('&').map(Into::into)
    }

    /// The `index`th [`Self::splits`].
    pub fn split(&self, index: isize) -> Option<(Option<&str>, Option<&str>)> {
        self.splits().neg_nth(index)
    }

    /// A [`DoubleEndedIterator`] of every "before" and "after of a place where a segment can be inserted.
    pub fn splits(&self) -> impl DoubleEndedIterator<Item = (Option<&str>, Option<&str>)> {
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

impl<'a> From<&'a str     > for BetterQuery<'a> {fn from(value: &'a str     ) -> Self {Self(value.into())}}
impl<'a> From<String      > for BetterQuery<'a> {fn from(value: String      ) -> Self {Self(value.into())}}
impl<'a> From<Cow<'a, str>> for BetterQuery<'a> {fn from(value: Cow<'a, str>) -> Self {Self(value       )}}

impl<'a> From<RawQuerySegment<'a>> for BetterQuery<'a> {fn from(value: RawQuerySegment<'a>) -> Self {value.0.into()}}
impl<'a> From<BetterRefQuery <'a>> for BetterQuery<'a> {fn from(value: BetterRefQuery <'a>) -> Self {value.0.into()}}

impl PartialEq<&str        > for BetterQuery<'_> {fn eq(&self, other: &&str        ) -> bool {&self.0 == other}}
impl PartialEq<String      > for BetterQuery<'_> {fn eq(&self, other: &String      ) -> bool {&self.0 == other}}
impl PartialEq<Cow<'_, str>> for BetterQuery<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {&self.0 == other}}

impl PartialEq<Option<&str>        > for BetterQuery<'_> {fn eq(&self, other: &Option<&str        >) -> bool {Some(self.as_str()) == other.as_deref()}}
impl PartialEq<Option<String>      > for BetterQuery<'_> {fn eq(&self, other: &Option<String      >) -> bool {Some(self.as_str()) == other.as_deref()}}
impl PartialEq<Option<Cow<'_, str>>> for BetterQuery<'_> {fn eq(&self, other: &Option<Cow<'_, str>>) -> bool {Some(self.as_str()) == other.as_deref()}}

impl PartialEq<RawQuerySegment    <'_>> for BetterQuery<'_> {fn eq(&self, other: &RawQuerySegment    <'_>) -> bool {     self.as_str()  == other.as_str()}}
impl PartialEq<BetterRefQuery     <'_>> for BetterQuery<'_> {fn eq(&self, other: &BetterRefQuery     <'_>) -> bool {     self.as_str()  == other.as_str()}}
impl PartialEq<BetterMaybeQuery   <'_>> for BetterQuery<'_> {fn eq(&self, other: &BetterMaybeQuery   <'_>) -> bool {Some(self.as_str()) == other.as_option_str()}}
impl PartialEq<BetterMaybeRefQuery<'_>> for BetterQuery<'_> {fn eq(&self, other: &BetterMaybeRefQuery<'_>) -> bool {Some(self.as_str()) == other.as_option_str()}}

impl std::fmt::Display for BetterQuery<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
