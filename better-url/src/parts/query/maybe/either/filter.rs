//! Filters.

use crate::prelude::*;

impl MaybeQuery<'_> {
    /// Either [`MaybeSpecialQuery::filtered`] or [`MaybeNonSpecialQuery::filtered`].
    pub fn filtered<F: FnMut(QuerySegment<'_>) -> bool>(self, mut f: F) -> (bool, Self) {
        match self {
            Self::Special   (x) => {let (c, x) = x.filtered(|x| f(x.into())); (c, x.into())},
            Self::NonSpecial(x) => {let (c, x) = x.filtered(|x| f(x.into())); (c, x.into())},
        }
    }

    /// Either [`MaybeSpecialQuery::try_filtered`] or [`MaybeNonSpecialQuery::try_filtered`].
    /// # Errors
    /// If the call to [`MaybeSpecialQuery::try_filtered`] returns an error, that error is returned.
    ///
    /// If the call to [`MaybeNonSpecialQuery::try_filtered`] returns an error, that error is returned.
    pub fn try_filtered<F: FnMut(QuerySegment<'_>) -> Result<bool, E>, E>(self, mut f: F) -> Result<(bool, Self), E> {
        match self {
            Self::Special   (x) => {let (c, x) = x.try_filtered(|x| f(x.into()))?; Ok((c, x.into()))},
            Self::NonSpecial(x) => {let (c, x) = x.try_filtered(|x| f(x.into()))?; Ok((c, x.into()))},
        }
    }

    /// Either [`MaybeSpecialQuery::filter`] or [`MaybeNonSpecialQuery::filter`].
    pub fn filter<F: FnMut(QuerySegment<'_>) -> bool>(&mut self, mut f: F) -> bool {
        match self {
            Self::Special   (x) => x.filter(|x| f(x.into())),
            Self::NonSpecial(x) => x.filter(|x| f(x.into())),
        }
    }

    /// Either [`MaybeSpecialQuery::try_filter`] or [`MaybeNonSpecialQuery::try_filter`].
    /// # Errors
    /// If the call to [`MaybeSpecialQuery::try_filter`] returns an error, that error is returned.
    ///
    /// If the call to [`MaybeNonSpecialQuery::try_filter`] returns an error, that error is returned.
    pub fn try_filter<F: FnMut(QuerySegment<'_>) -> Result<bool, E>, E>(&mut self, mut f: F) -> Result<bool, E> {
        match self {
            Self::Special   (x) => x.try_filter(|x| f(x.into())),
            Self::NonSpecial(x) => x.try_filter(|x| f(x.into())),
        }
    }
}
