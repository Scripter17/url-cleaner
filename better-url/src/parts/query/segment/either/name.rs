//! Name stuff.

use crate::prelude::*;

impl<'a> QuerySegment<'a> {
    /// Either [`SpecialQuerySegment::raw_name`], [`NonSpecialQuerySegment::raw_name`], or [`FragmentQuerySegment::raw_name`].
    pub fn raw_name(&self) -> &str {
        match self {
            Self::Special   (x) => x.raw_name(),
            Self::NonSpecial(x) => x.raw_name(),
            Self::Fragment  (x) => x.raw_name(),
        }
    }

    /// Either [`SpecialQuerySegment::into_raw_name`], [`NonSpecialQuerySegment::into_raw_name`], or [`FragmentQuerySegment::into_raw_name`].
    pub fn into_raw_name(self) -> Cow<'a, str> {
        match self {
            Self::Special   (x) => x.into_raw_name(),
            Self::NonSpecial(x) => x.into_raw_name(),
            Self::Fragment  (x) => x.into_raw_name(),
        }
    }

    /// Either [`SpecialQuerySegment::name`], [`NonSpecialQuerySegment::name`], or [`FragmentQuerySegment::name`].
    pub fn name(&self) -> Cow<'_, str> {
        match self {
            Self::Special   (x) => x.name(),
            Self::NonSpecial(x) => x.name(),
            Self::Fragment  (x) => x.name(),
        }
    }

    /// Either [`SpecialQuerySegment::into_name`], [`NonSpecialQuerySegment::into_name`], or [`FragmentQuerySegment::into_name`].
    pub fn into_name(self) -> Cow<'a, str> {
        match self {
            Self::Special   (x) => x.into_name(),
            Self::NonSpecial(x) => x.into_name(),
            Self::Fragment  (x) => x.into_name(),
        }
    }

    /// Either [`SpecialQuerySegment::set_name`], [`NonSpecialQuerySegment::set_name`], or [`FragmentQuerySegment::set_name`].
    pub fn set_name(&mut self, name: &str) {
        match self {
            Self::Special   (x) => x.set_name(name),
            Self::NonSpecial(x) => x.set_name(name),
            Self::Fragment  (x) => x.set_name(name),
        }
    }
}
