//! Name stuff.

use crate::prelude::*;

impl<'a> QuerySegment<'a> {
    /// Either [`SpecialQuerySegment::name`] or [`NonSpecialQuerySegment::name`].
    pub fn name(&self) -> QueryName<'_> {
        match self {
            Self::Special   (x) => x.name().into(),
            Self::NonSpecial(x) => x.name().into(),
        }
    }

    /// Either [`SpecialQuerySegment::into_name`] or [`NonSpecialQuerySegment::into_name`].
    pub fn into_name(self) -> QueryName<'a> {
        match self {
            Self::Special   (x) => x.into_name().into(),
            Self::NonSpecial(x) => x.into_name().into(),
        }
    }

    /// Either [`SpecialQuerySegment::set_name`] or [`NonSpecialQuerySegment::set_name`].
    pub fn set_name<'b, T: Into<SpecialQueryName<'b>> + Into<NonSpecialQueryName<'b>>>(&mut self, name: T) {
        match self {
            Self::Special   (x) => x.set_name(name),
            Self::NonSpecial(x) => x.set_name(name),
        }
    }
}
