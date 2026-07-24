//! Filters.

use crate::prelude::*;

impl<'a> MaybeQueryLike<'a> {
    /// Either [`MaybeQuery::filtered`] or [`MaybeFragmentQuery::filtered`].
    pub fn filtered<F: FnMut(QueryLikeSegment<'_>) -> bool>(self, mut f: F) -> (bool, Self) {
        match self {
            Self::Query   (x) => {let (c, x) = x.filtered(|x| f(x.into())); (c, x.into())},
            Self::Fragment(x) => {let (c, x) = x.filtered(|x| f(x.into())); (c, x.into())},
        }
    }

    /// Either [`MaybeQuery::try_filtered`] or [`MaybeFragmentQuery::try_filtered`].
    /// # Errors
    /// If the call to [`MaybeQuery::try_filtered`] returns an error, that error is returned.
    ///
    /// If the call to [`MaybeFragmentQuery::try_filtered`] returns an error, that error is returned.
    pub fn try_filtered<F: FnMut(QueryLikeSegment<'_>) -> Result<bool, E>, E>(self, mut f: F) -> Result<(bool, Self), E> {
        match self {
            Self::Query   (x) => {let (c, x) = x.try_filtered(|x| f(x.into()))?; Ok((c, x.into()))},
            Self::Fragment(x) => {let (c, x) = x.try_filtered(|x| f(x.into()))?; Ok((c, x.into()))},
        }
    }

    /// Either [`MaybeQuery::filter`] or [`MaybeFragmentQuery::filter`].
    pub fn filter<F: FnMut(QueryLikeSegment<'_>) -> bool>(&mut self, mut f: F) -> bool {
        match self {
            Self::Query   (x) => x.filter(|x| f(x.into())),
            Self::Fragment(x) => x.filter(|x| f(x.into())),
        }
    }

    /// Either [`MaybeQuery::try_filter`] or [`MaybeFragmentQuery::try_filter`].
    /// # Errors
    /// If the call to [`MaybeQuery::try_filter`] returns an error, that error is returned.
    ///
    /// If the call to [`MaybeFragmentQuery::try_filter`] returns an error, that error is returned.
    pub fn try_filter<F: FnMut(QueryLikeSegment<'_>) -> Result<bool, E>, E>(&mut self, mut f: F) -> Result<bool, E> {
        match self {
            Self::Query   (x) => x.try_filter(|x| f(x.into())),
            Self::Fragment(x) => x.try_filter(|x| f(x.into())),
        }
    }
}
