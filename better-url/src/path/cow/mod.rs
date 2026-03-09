//! [`BetterPath`].

use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// A maybe owned path string.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BetterPath<'a>(pub Cow<'a, str>);

impl BetterPath<'_> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(path: T) -> Self {
        path.into()
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Make a [`String`].
    pub fn into_string(self) -> String {
        self.0.into_owned()
    }

    /// Make an owned [`Self`].
    pub fn into_owned(self) -> BetterPath<'static> {
        self.0.into_owned().into()
    }

    /// Borrow segments as a [`str`].
    pub fn as_segments_str(&self) -> Option<&str> {
        self.0.strip_prefix("/")
    }

    /// Borrow segments as a [`BetterPathSegments`].
    pub fn as_segments(&self) -> Option<BetterPathSegments<'_>> {
        self.0.strip_prefix("/").map(Into::into)
    }

    /// Borrow segments as a [`BetterRefPathSegments`].
    pub fn as_ref_segments(&self) -> Option<BetterRefPathSegments<'_>> {
        self.0.strip_prefix("/").map(Into::into)
    }
}

impl<'a> From<&'a str     > for BetterPath<'a> {fn from(value: &'a str     ) -> Self {Self(value.into())}}
impl<'a> From<String      > for BetterPath<'a> {fn from(value: String      ) -> Self {Self(value.into())}}
impl<'a> From<Cow<'a, str>> for BetterPath<'a> {fn from(value: Cow<'a, str>) -> Self {Self(value       )}}

impl<'a> From<BetterPathSegments   <'_>> for BetterPath<'a> {fn from(value: BetterPathSegments   <'_>) -> Self {value.into_path_string().into()}}
impl<'a> From<BetterRefPathSegments<'_>> for BetterPath<'a> {fn from(value: BetterRefPathSegments<'_>) -> Self {value.to_path_string().into()}}

impl PartialEq<&str        > for BetterPath<'_> {fn eq(&self, other: &&str        ) -> bool {&self.0 == other}}
impl PartialEq<String      > for BetterPath<'_> {fn eq(&self, other: &String      ) -> bool {&self.0 == other}}
impl PartialEq<Cow<'_, str>> for BetterPath<'_> {fn eq(&self, other: &Cow<'_, str>) -> bool {&self.0 == other}}

impl PartialEq<BetterPathSegments   <'_>> for BetterPath<'_> {fn eq(&self, other: &BetterPathSegments   <'_>) -> bool {self.as_segments_str() == Some(other.as_str())}}
impl PartialEq<BetterRefPathSegments<'_>> for BetterPath<'_> {fn eq(&self, other: &BetterRefPathSegments<'_>) -> bool {self.as_segments_str() == Some(other.as_str())}}
impl PartialEq<BetterRefPath        <'_>> for BetterPath<'_> {fn eq(&self, other: &BetterRefPath        <'_>) -> bool {self.0 == other.0}}

impl std::fmt::Display for BetterPath<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
