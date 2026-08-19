//! Name.

use crate::prelude::*;

impl<'a> QueryLikeSegment<'a> {
    /// Either [`QuerySegment::name`] or [`FragmentQuerySegment::name`].
    pub fn name(&self) -> QueryLikeName<'_> {
        match self {
            Self::Query   (x) => x.name().into(),
            Self::Fragment(x) => x.name().into(),
        }
    }

    /// Either [`QuerySegment::into_name`] or [`FragmentQuerySegment::into_name`].
    pub fn into_name(self) -> QueryLikeName<'a> {
        match self {
            Self::Query   (x) => x.into_name().into(),
            Self::Fragment(x) => x.into_name().into(),
        }
    }

    /// Either [`QuerySegment::set_name`] or [`FragmentQuerySegment::set_name`].
    pub fn set_name<'b, T: Into<SpecialQueryName<'b>> + Into<NonSpecialQueryName<'b>> + Into<FragmentQueryName<'b>>>(&mut self, name: T) {
        match self {
            Self::Query   (x) => x.set_name(name),
            Self::Fragment(x) => x.set_name(name),
        }
    }
}
