//! Value stuff.

use crate::prelude::*;

impl<'a> QuerySegment<'a> {
    /// Either [`SpecialQuerySegment::has_value`] or [`NonSpecialQuerySegment::has_value`].
    pub fn has_value(&self) -> bool {
        match self {
            Self::Special   (x) => x.has_value(),
            Self::NonSpecial(x) => x.has_value(),
        }
    }
    
    /// Either [`SpecialQuerySegment::value_start`] or [`NonSpecialQuerySegment::value_start`].
    pub fn value_start(&self) -> Option<usize> {
        match self {
            Self::Special   (x) => x.value_start(),
            Self::NonSpecial(x) => x.value_start(),
        }
    }

    /// Either [`SpecialQuerySegment::value_after`] or [`NonSpecialQuerySegment::value_after`].
    pub fn value_after(&self) -> Option<usize> {
        match self {
            Self::Special   (x) => x.value_after(),
            Self::NonSpecial(x) => x.value_after(),
        }
    }

    /// Either [`SpecialQuerySegment::value_range`] or [`NonSpecialQuerySegment::value_range`].
    pub fn value_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Special   (x) => x.value_range(),
            Self::NonSpecial(x) => x.value_range(),
        }
    }

    /// Either [`SpecialQuerySegment::raw_value`] or [`NonSpecialQuerySegment::raw_value`].
    pub fn raw_value(&self) -> Option<&str> {
        match self {
            Self::Special   (x) => x.raw_value(),
            Self::NonSpecial(x) => x.raw_value(),
        }
    }

    /// Either [`SpecialQuerySegment::into_raw_value`] or [`NonSpecialQuerySegment::into_raw_value`].
    pub fn into_raw_value(self) -> Option<Cow<'a, str>> {
        match self {
            Self::Special   (x) => x.into_raw_value(),
            Self::NonSpecial(x) => x.into_raw_value(),
        }
    }

    /// Either [`SpecialQuerySegment::value`] or [`NonSpecialQuerySegment::value`].
    pub fn value(&self) -> Option<Cow<'_, str>> {
        match self {
            Self::Special   (x) => x.value(),
            Self::NonSpecial(x) => x.value(),
        }
    }

    /// Either [`SpecialQuerySegment::into_value`] or [`NonSpecialQuerySegment::into_value`].
    pub fn into_value(self) -> Option<Cow<'a, str>> {
        match self {
            Self::Special   (x) => x.into_value(),
            Self::NonSpecial(x) => x.into_value(),
        }
    }

    /// Either [`SpecialQuerySegment::set_value`] or [`NonSpecialQuerySegment::set_value`].
    pub fn set_value(&mut self, value: &str) {
        match self {
            Self::Special   (x) => x.set_value(value),
            Self::NonSpecial(x) => x.set_value(value),
        }
    }
}
