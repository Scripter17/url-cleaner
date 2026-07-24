//! Name.

use crate::prelude::*;

impl<'a> QueryLikeSegment<'a> {
    /// Either [`QuerySegment::raw_name`] or [`FragmentQuerySegment::raw_name`].
    pub fn raw_name(&self) -> &str {
        match self {
            Self::Query   (x) => x.raw_name(),
            Self::Fragment(x) => x.raw_name(),
        }
    }

    /// Either [`QuerySegment::name`] or [`FragmentQuerySegment::name`].
    pub fn name(&self) -> Cow<'_, str> {
        match self {
            Self::Query   (x) => x.name(),
            Self::Fragment(x) => x.name(),
        }
    }

    /// Either [`QuerySegment::into_raw_name`] or [`FragmentQuerySegment::into_raw_name`].
    pub fn into_raw_name(self) -> Cow<'a, str> {
        match self {
            Self::Query   (x) => x.into_raw_name(),
            Self::Fragment(x) => x.into_raw_name(),
        }
    }

    /// Either [`QuerySegment::into_name`] or [`FragmentQuerySegment::into_name`].
    pub fn into_name(self) -> Cow<'a, str> {
        match self {
            Self::Query   (x) => x.into_name(),
            Self::Fragment(x) => x.into_name(),
        }
    }

    /// Either [`QuerySegment::set_name`] or [`FragmentQuerySegment::set_name`].
    pub fn set_name(&mut self, name: &str) {
        match self {
            Self::Query   (x) => x.set_name(name),
            Self::Fragment(x) => x.set_name(name),
        }
    }
}
