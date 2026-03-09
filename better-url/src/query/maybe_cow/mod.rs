//! [`BetterMaybeQuery`].

use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::prelude::*;

mod get;
mod set;
mod remove;
mod extend;
mod filter;

/// A [`BetterQuery`] that might be [`None`].
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BetterMaybeQuery<'a>(pub Option<BetterQuery<'a>>);

impl BetterMaybeQuery<'_> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(query: T) -> Self {
        query.into()
    }

    /// Borrow as an `Option<&str>`.
    pub fn as_option_str(&self) -> Option<&str> {
        self.0.as_ref().map(BetterQuery::as_str)
    }

    /// Convert into an `Option<String>`.
    pub fn into_option_string(self) -> Option<String> {
        self.0.map(BetterQuery::into_string)
    }

    /// Convert into an owned [`Self`].
    pub fn into_owned(self) -> BetterMaybeQuery<'static> {
        self.into_option_string().into()
    }

    /// If [`Self::0`] is [`Some`], [`BetterQuery::iter`].
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = RawQuerySegment<'_>> {
        self.0.iter().flat_map(BetterQuery::iter)
    }
}

impl<'a> From<&'a str            > for BetterMaybeQuery<'a> {fn from(value: &'a str            ) -> Self {Self(Some(value.into()))}}
impl<'a> From<Cow<'a, str>       > for BetterMaybeQuery<'a> {fn from(value: Cow<'a, str>       ) -> Self {Self(Some(value.into()))}}
impl<'a> From<String             > for BetterMaybeQuery<'a> {fn from(value: String             ) -> Self {Self(Some(value.into()))}}
impl<'a> From<RawQuerySegment<'a>> for BetterMaybeQuery<'a> {fn from(value: RawQuerySegment<'a>) -> Self {Self(Some(value.into()))}}

impl<'a> From<Option<&'a str            >> for BetterMaybeQuery<'a> {fn from(value: Option<&'a str            >) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<Cow<'a, str>       >> for BetterMaybeQuery<'a> {fn from(value: Option<Cow<'a, str>       >) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<String             >> for BetterMaybeQuery<'a> {fn from(value: Option<String             >) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<RawQuerySegment<'a>>> for BetterMaybeQuery<'a> {fn from(value: Option<RawQuerySegment<'a>>) -> Self {Self(value.map(Into::into))}}

impl PartialEq<&str        > for BetterMaybeQuery<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_option_str() == Some(other)}}
impl PartialEq<String      > for BetterMaybeQuery<'_> {fn eq(&self, other: &String      ) -> bool {self.as_option_str() == Some(other)}}
impl PartialEq<Cow<'_, str>> for BetterMaybeQuery<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_option_str() == Some(other)}}

impl PartialEq<Option<&str>        > for BetterMaybeQuery<'_> {fn eq(&self, other: &Option<&str        >) -> bool {self.as_option_str() == other.as_deref()}}
impl PartialEq<Option<String>      > for BetterMaybeQuery<'_> {fn eq(&self, other: &Option<String      >) -> bool {self.as_option_str() == other.as_deref()}}
impl PartialEq<Option<Cow<'_, str>>> for BetterMaybeQuery<'_> {fn eq(&self, other: &Option<Cow<'_, str>>) -> bool {self.as_option_str() == other.as_deref()}}

impl PartialEq<RawQuerySegment    <'_>> for BetterMaybeQuery<'_> {fn eq(&self, other: &RawQuerySegment    <'_>) -> bool {self.as_option_str() == Some(other.as_str())}}
impl PartialEq<BetterQuery        <'_>> for BetterMaybeQuery<'_> {fn eq(&self, other: &BetterQuery        <'_>) -> bool {self.as_option_str() == Some(other.as_str())}}
impl PartialEq<BetterRefQuery     <'_>> for BetterMaybeQuery<'_> {fn eq(&self, other: &BetterRefQuery     <'_>) -> bool {self.as_option_str() == Some(other.as_str())}}
impl PartialEq<BetterMaybeRefQuery<'_>> for BetterMaybeQuery<'_> {fn eq(&self, other: &BetterMaybeRefQuery<'_>) -> bool {self.as_option_str() == other.as_option_str()}}
