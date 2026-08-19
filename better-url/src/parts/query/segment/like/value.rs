//! Value.

use crate::prelude::*;

impl<'a> QueryLikeSegment<'a> {
    /// Either [`QuerySegment::has_value`] or [`FragmentQuerySegment::has_value`].
    pub fn has_value(&self) -> bool {
        match self {
            Self::Query   (x) => x.has_value(),
            Self::Fragment(x) => x.has_value(),
        }
    }

    /// Either [`QuerySegment::value`] or [`FragmentQuerySegment::value`].
    pub fn value(&self) -> MaybeQueryLikeValue<'_> {
        match self {
            Self::Query   (x) => x.value().into(),
            Self::Fragment(x) => x.value().into(),
        }
    }

    /// Either [`QuerySegment::into_value`] or [`FragmentQuerySegment::into_value`].
    pub fn into_value(self) -> MaybeQueryLikeValue<'a> {
        match self {
            Self::Query   (x) => x.into_value().into(),
            Self::Fragment(x) => x.into_value().into(),
        }
    }

    /// Either [`QuerySegment::set_value`] or [`FragmentQuerySegment::set_value`].
    pub fn set_value<'b, T: Into<MaybeSpecialQueryValue<'b>> + Into<MaybeNonSpecialQueryValue<'b>> + Into<MaybeFragmentQueryValue<'b>>>(&mut self, value: T) {
        match self {
            Self::Query   (x) => x.set_value(value),
            Self::Fragment(x) => x.set_value(value),
        }
    }
}
