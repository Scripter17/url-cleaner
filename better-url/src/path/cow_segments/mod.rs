//! [`BetterPathSegments`].

use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::prelude::*;

mod get;
mod set;
mod remove;
mod extend;

/// A maybe owned path segments string.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BetterPathSegments<'a>(pub Cow<'a, str>);

impl BetterPathSegments<'_> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(path: T) -> Self {
        path.into()
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Make a [`String`] without a leading `/`.
    pub fn into_string(self) -> String {
        self.0.into()
    }

    /// Make an owned [`Self`].
    pub fn into_owned(self) -> BetterPathSegments<'static> {
        self.0.into_owned().into()
    }

    /// Make a [`String`] with a leading `/`.
    pub fn to_path_string(&self) -> String {
        format!("/{self}")
    }

    /// Make a [`String`] with a leading `/`.
    pub fn into_path_string(self) -> String {
        match self.0 {
            Cow::Borrowed(    x) => format!("/{x}"),
            Cow::Owned   (mut x) => {x.insert(0, '/'); x}
        }
    }

    /// An iterator over the [`RawPathSegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = RawPathSegment<'_>> {
        self.0.split('/').map(Into::into)
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

impl<'a> From<&'a str     > for BetterPathSegments<'a> {fn from(value: &'a str     ) -> Self {Self(value.into())}}
impl<'a> From<String      > for BetterPathSegments<'a> {fn from(value: String      ) -> Self {Self(value.into())}}
impl<'a> From<Cow<'a, str>> for BetterPathSegments<'a> {fn from(value: Cow<'a, str>) -> Self {Self(value       )}}

impl<'a> From<RawPathSegment<'a>> for BetterPathSegments<'a> {fn from(value: RawPathSegment<'a>) -> Self {value.as_str().into()}}

impl<'a> TryFrom<BetterPath<'a>> for BetterPathSegments<'a> {
    type Error = OpaquePath;

    fn try_from(mut value: BetterPath<'a>) -> Result<Self, Self::Error> {
        if let Some(segments) = value.as_segments_str() {
            value.0.retain_substr(segments);
            Ok(Self(value.0))
        } else {
            Err(OpaquePath)
        }
    }
}

impl<'a> TryFrom<BetterRefPath<'a>> for BetterPathSegments<'a> {
    type Error = OpaquePath;

    fn try_from(value: BetterRefPath<'a>) -> Result<Self, Self::Error> {
        value.as_segments().ok_or(OpaquePath)
    }
}

impl PartialEq<&str        > for BetterPathSegments<'_> {fn eq(&self, other: &&str        ) -> bool {&self.0 == other}}
impl PartialEq<String      > for BetterPathSegments<'_> {fn eq(&self, other: &String      ) -> bool {&self.0 == other}}
impl PartialEq<Cow<'_, str>> for BetterPathSegments<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {&self.0 == other}}

impl PartialEq<BetterPath           <'_>> for BetterPathSegments<'_> {fn eq(&self, other: &BetterPath           <'_>) -> bool {Some(self.as_str()) == other.as_segments_str()}}
impl PartialEq<BetterRefPath        <'_>> for BetterPathSegments<'_> {fn eq(&self, other: &BetterRefPath        <'_>) -> bool {Some(self.as_str()) == other.as_segments_str()}}
impl PartialEq<BetterRefPathSegments<'_>> for BetterPathSegments<'_> {fn eq(&self, other: &BetterRefPathSegments<'_>) -> bool {self.0 == other.0}}

impl std::fmt::Display for BetterPathSegments<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
