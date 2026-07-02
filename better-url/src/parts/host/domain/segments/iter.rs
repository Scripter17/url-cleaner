//! [`DomainSegmentsIter`].

use crate::prelude::*;

/// An [`Iterator`] over the [`DomainSegment`]s of a [`DomainSegments`].
#[derive(Debug, Clone)]
pub struct DomainSegmentsIter<'a>(pub(crate) SplitDots<'a>);

impl<'a> TryFrom<DomainSegmentsIter<'a>> for DomainSegments<'a> {
    type Error = CantBeEmpty;

    fn try_from(value: DomainSegmentsIter<'a>) -> Result<Self, Self::Error> {
        Ok(DomainSegments(value.0.0.ok_or(CantBeEmpty)?.into()))
    }
}

impl<'a> IntoIterator for &'a DomainSegments<'_> {
    type IntoIter = DomainSegmentsIter<'a>;
    type Item = DomainSegment<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DomainSegmentsIter(SplitDots(Some(&self.0)))
    }
}

impl<'a> Iterator for DomainSegmentsIter<'a> {
    type Item = DomainSegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| DomainSegment(x.into()))
    }
}

impl<'a> DoubleEndedIterator for DomainSegmentsIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| DomainSegment(x.into()))
    }
}
