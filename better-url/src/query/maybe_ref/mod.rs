//! [`BetterMaybeRefQuery`].

use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::prelude::*;

mod get;
mod remove;

/// A borrowed query string.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BetterMaybeRefQuery<'a>(pub Option<BetterRefQuery<'a>>);

impl<'a> BetterMaybeRefQuery<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(query: T) -> Self {
        query.into()
    }

    /// Borrow as an `Option<&str>`.`
    pub fn as_option_str(&self) -> Option<&'a str> {
        self.0.map(|x| x.as_str())
    }

    /// A [`DoubleEndedIterator`] over the [`RawQuerySegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = RawQuerySegment<'a>> {
        self.0.iter().flat_map(BetterRefQuery::iter)
    }
}

impl<'a> From<&'a str            > for BetterMaybeRefQuery<'a> {fn from(value: &'a str            ) -> Self {Self(Some(value.into()))}}
impl<'a> From<RawQuerySegment<'a>> for BetterMaybeRefQuery<'a> {fn from(value: RawQuerySegment<'a>) -> Self {Self(Some(value.into()))}}

impl<'a> From<Option<&'a str>            > for BetterMaybeRefQuery<'a> {fn from(value: Option<&'a str            >) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<RawQuerySegment<'a>>> for BetterMaybeRefQuery<'a> {fn from(value: Option<RawQuerySegment<'a>>) -> Self {Self(value.map(Into::into))}}

impl PartialEq<&str        > for BetterMaybeRefQuery<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_option_str() == Some(other)}}
impl PartialEq<String      > for BetterMaybeRefQuery<'_> {fn eq(&self, other: &String      ) -> bool {self.as_option_str() == Some(other)}}
impl PartialEq<Cow<'_, str>> for BetterMaybeRefQuery<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_option_str() == Some(other)}}

impl PartialEq<Option<&str>        > for BetterMaybeRefQuery<'_> {fn eq(&self, other: &Option<&str        >) -> bool {self.as_option_str() == other.as_deref()}}
impl PartialEq<Option<String>      > for BetterMaybeRefQuery<'_> {fn eq(&self, other: &Option<String      >) -> bool {self.as_option_str() == other.as_deref()}}
impl PartialEq<Option<Cow<'_, str>>> for BetterMaybeRefQuery<'_> {fn eq(&self, other: &Option<Cow<'_, str>>) -> bool {self.as_option_str() == other.as_deref()}}

impl PartialEq<RawQuerySegment <'_>> for BetterMaybeRefQuery<'_> {fn eq(&self, other: &RawQuerySegment <'_>) -> bool {self.as_option_str() == Some(other.as_str())}}
impl PartialEq<BetterQuery     <'_>> for BetterMaybeRefQuery<'_> {fn eq(&self, other: &BetterQuery     <'_>) -> bool {self.as_option_str() == Some(other.as_str())}}
impl PartialEq<BetterRefQuery  <'_>> for BetterMaybeRefQuery<'_> {fn eq(&self, other: &BetterRefQuery  <'_>) -> bool {self.as_option_str() == Some(other.as_str())}}
impl PartialEq<BetterMaybeQuery<'_>> for BetterMaybeRefQuery<'_> {fn eq(&self, other: &BetterMaybeQuery<'_>) -> bool {self.as_option_str() == other.as_option_str()}}
