//! [`BetterRefPath`].

use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::prelude::*;

/// A borrowed path string.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BetterRefPath<'a>(pub &'a str);

impl<'a> BetterRefPath<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(path: T) -> Self {
        path.into()
    }

    /// Borrow as a [`str`].
    pub fn as_str(self) -> &'a str {
        self.0
    }

    /// Borrow segments as a [`str`].
    pub fn as_segments_str(self) -> Option<&'a str> {
        self.0.strip_prefix("/")
    }

    /// Borrow segments as a [`BetterPathSegments`].
    pub fn as_segments(self) -> Option<BetterPathSegments<'a>> {
        self.0.strip_prefix("/").map(Into::into)
    }

    /// Borrow segments as a [`BetterRefPathSegments`].
    pub fn as_ref_segments(self) -> Option<BetterRefPathSegments<'a>> {
        self.0.strip_prefix("/").map(Into::into)
    }
}

impl<'a> From<&'a str> for BetterRefPath<'a> {fn from(value: &'a str) -> Self {Self(value)}}

impl PartialEq<&str        > for BetterRefPath<'_> {fn eq(&self, other: &&str        ) -> bool {self.as_str() == *other}}
impl PartialEq<String      > for BetterRefPath<'_> {fn eq(&self, other: &String      ) -> bool {self.as_str() == *other}}
impl PartialEq<Cow<'_, str>> for BetterRefPath<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {self.as_str() == *other}}

impl PartialEq<BetterPathSegments   <'_>> for BetterRefPath<'_> {fn eq(&self, other: &BetterPathSegments   <'_>) -> bool {self.as_segments_str() == Some(other.as_str())}}
impl PartialEq<BetterRefPathSegments<'_>> for BetterRefPath<'_> {fn eq(&self, other: &BetterRefPathSegments<'_>) -> bool {self.as_segments_str() == Some(other.as_str())}}
impl PartialEq<BetterPath           <'_>> for BetterRefPath<'_> {fn eq(&self, other: &BetterPath           <'_>) -> bool {self.0 == other.0}}

impl std::fmt::Display for BetterRefPath<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
