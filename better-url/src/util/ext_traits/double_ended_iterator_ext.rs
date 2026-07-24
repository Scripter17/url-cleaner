//! [`DoubleEndedIteratorExt`].

use crate::prelude::*;

/// [`DoubleEndedIterator`] extension trait.
pub(crate) trait DoubleEndedIteratorExt: DoubleEndedIterator {
    /// Index from either the front or the back.
    fn neg_nth(&mut self, n: isize) -> Option<Self::Item>;

    /// [`IteratorExt::try_nth`] but backwards.
    fn try_nth_back(&mut self, n: usize) -> Result<Self::Item, usize>;

    /// [`IteratorExt::try_nth`] using [`Self::neg_nth`].
    fn try_neg_nth(&mut self, n: isize) -> Result<Self::Item, usize>;
}

impl<I: DoubleEndedIterator> DoubleEndedIteratorExt for I {
    fn neg_nth(&mut self, n: isize) -> Option<Self::Item> {
        match n {
            0.. => self.nth     (n as usize),
            ..0 => self.nth_back(n.unsigned_abs() - 1),
        }
    }

    fn try_nth_back(&mut self, n: usize) -> Result<Self::Item, usize> {
        self.rev().try_nth(n)
    }

    fn try_neg_nth(&mut self, n: isize) -> Result<Self::Item, usize> {
        match n {
            0.. => self.try_nth     (n as usize),
            ..0 => self.try_nth_back(n.unsigned_abs() - 1),
        }
    }
}
