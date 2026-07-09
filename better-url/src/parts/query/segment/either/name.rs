//! Name stuff.

use crate::prelude::*;

impl<'a> QuerySegment<'a> {
    /// Either [`SpecialQuerySegment::raw_name`] or [`NonSpecialQuerySegment::raw_name`].
    pub fn raw_name(&self) -> &str {
        match self {
            Self::Special   (x) => x.raw_name(),
            Self::NonSpecial(x) => x.raw_name(),
        }
    }

    /// Either [`SpecialQuerySegment::into_raw_name`] or [`NonSpecialQuerySegment::into_raw_name`].
    pub fn into_raw_name(self) -> Cow<'a, str> {
        match self {
            Self::Special   (x) => x.into_raw_name(),
            Self::NonSpecial(x) => x.into_raw_name(),
        }
    }

    /// Either [`SpecialQuerySegment::name`] or [`NonSpecialQuerySegment::name`].
    pub fn name(&self) -> Cow<'_, str> {
        match self {
            Self::Special   (x) => x.name(),
            Self::NonSpecial(x) => x.name(),
        }
    }

    /// Either [`SpecialQuerySegment::into_name`] or [`NonSpecialQuerySegment::into_name`].
    pub fn into_name(self) -> Cow<'a, str> {
        match self {
            Self::Special   (x) => x.into_name(),
            Self::NonSpecial(x) => x.into_name(),
        }
    }

    /// Either [`SpecialQuerySegment::set_name`] or [`NonSpecialQuerySegment::set_name`].
    pub fn set_name(&mut self, name: &str) {
        match self {
            Self::Special   (x) => x.set_name(name),
            Self::NonSpecial(x) => x.set_name(name),
        }
    }
}
