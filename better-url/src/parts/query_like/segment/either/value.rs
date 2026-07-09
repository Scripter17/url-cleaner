//! Value.

use crate::prelude::*;

impl<'a> QueryLikeSegment<'a> {
    /// Either [`QuerySegment::raw_value`] or [`FragmentQuerySegment::raw_value`].
    pub fn raw_value(&self) -> Option<&str> {
        match self {
            Self::Query   (x) => x.raw_value(),
            Self::Fragment(x) => x.raw_value(),
        }
    }

    /// Either [`QuerySegment::value`] or [`FragmentQuerySegment::value`].
    pub fn value(&self) -> Option<Cow<'_, str>> {
        match self {
            Self::Query   (x) => x.value(),
            Self::Fragment(x) => x.value(),
        }
    }

    /// Either [`QuerySegment::into_raw_value`] or [`FragmentQuerySegment::into_raw_value`].
    pub fn into_raw_value(self) -> Option<Cow<'a, str>> {
        match self {
            Self::Query   (x) => x.into_raw_value(),
            Self::Fragment(x) => x.into_raw_value(),
        }
    }

    /// Either [`QuerySegment::into_value`] or [`FragmentQuerySegment::into_value`].
    pub fn into_value(self) -> Option<Cow<'a, str>> {
        match self {
            Self::Query   (x) => x.into_value(),
            Self::Fragment(x) => x.into_value(),
        }
    }

    /// Either [`QuerySegment::set_value`] or [`FragmentQuerySegment::set_value`].
    pub fn set_value(&mut self, value: Option<&str>) {
        match self {
            Self::Query   (x) => x.set_value(value),
            Self::Fragment(x) => x.set_value(value),
        }
    }
}
