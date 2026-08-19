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

    /// Either [`SpecialQuerySegment::value`] or [`NonSpecialQuerySegment::value`].
    pub fn value(&self) -> MaybeQueryValue<'_> {
        match self {
            Self::Special   (x) => x.value().into(),
            Self::NonSpecial(x) => x.value().into(),
        }
    }

    /// Either [`SpecialQuerySegment::into_value`] or [`NonSpecialQuerySegment::into_value`].
    pub fn into_value(self) -> MaybeQueryValue<'a> {
        match self {
            Self::Special   (x) => x.into_value().into(),
            Self::NonSpecial(x) => x.into_value().into(),
        }
    }

    /// Either [`SpecialQuerySegment::set_value`] or [`NonSpecialQuerySegment::set_value`].
    pub fn set_value<'b, T: Into<MaybeSpecialQueryValue<'b>> + Into<MaybeNonSpecialQueryValue<'b>>>(&mut self, value: T) {
        match self {
            Self::Special   (x) => x.set_value(value),
            Self::NonSpecial(x) => x.set_value(value),
        }
    }
}
