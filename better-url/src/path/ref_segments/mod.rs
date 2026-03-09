//! [`BetterRefPathSegments`].

use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::prelude::*;

mod get;
mod remove;

/// A borrowed path segments string.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BetterRefPathSegments<'a>(pub &'a str);

impl<'a> BetterRefPathSegments<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(path: T) -> Self {
        path.into()
    }

    /// Borrow as a [`str`].
    pub fn as_str(self) -> &'a str {
        self.0
    }

    /// Make a [`String`] with a leading `/`.
    pub fn to_path_string(self) -> String {
        format!("/{self}")
    }

    /// An iterator over [`RawPathSegment`]s.
    pub fn iter(self) -> impl DoubleEndedIterator<Item = RawPathSegment<'a>> {
        self.0.split('/').map(Into::into)
    }

    /// The `index`th [`Self::splits`].
    pub fn split(self, index: isize) -> Option<(Option<&'a str>, Option<&'a str>)> {
        self.splits().neg_nth(index)
    }

    /// A [`DoubleEndedIterator`] of every "before" and "after of a place where a segment can be inserted.
    pub fn splits(self) -> impl DoubleEndedIterator<Item = (Option<&'a str>, Option<&'a str>)> {
        self.iter().map(move |segment| {
            let offset = segment.0.addr() - self.0.addr();

            if offset == 0 {
                (None, Some(&self.0[offset..]))
            } else {
                (Some(&self.0[..offset - 1]), Some(&self.0[offset..]))
            }
        }).chain(std::iter::once((Some(self.as_str()), None)))
    }
}

impl<'a> From<&'a str> for BetterRefPathSegments<'a> {fn from(value: &'a str) -> Self {Self(value)}}

impl<'a> From<RawPathSegment<'a>> for BetterRefPathSegments<'a> {fn from(value: RawPathSegment<'a>) -> Self {value.as_str().into()}}

impl<'a> TryFrom<BetterRefPath<'a>> for BetterRefPathSegments<'a> {
    type Error = OpaquePath;

    fn try_from(value: BetterRefPath<'a>) -> Result<Self, Self::Error> {
        value.as_ref_segments().ok_or(OpaquePath)
    }
}

impl PartialEq<&str        > for BetterRefPathSegments<'_> {fn eq(&self, other: &&str        ) -> bool {self.0 == *other}}
impl PartialEq<String      > for BetterRefPathSegments<'_> {fn eq(&self, other: &String      ) -> bool {self.0 ==  other}}
impl PartialEq<Cow<'_, str>> for BetterRefPathSegments<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.0 ==  other}}


impl PartialEq<BetterPath        <'_>> for BetterRefPathSegments<'_> {fn eq(&self, other: &BetterPath        <'_>) -> bool {Some(self.as_str()) == other.as_segments_str()}}
impl PartialEq<BetterRefPath     <'_>> for BetterRefPathSegments<'_> {fn eq(&self, other: &BetterRefPath     <'_>) -> bool {Some(self.as_str()) == other.as_segments_str()}}
impl PartialEq<BetterPathSegments<'_>> for BetterRefPathSegments<'_> {fn eq(&self, other: &BetterPathSegments<'_>) -> bool {self.0 == other.0}}

impl std::fmt::Display for BetterRefPathSegments<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
