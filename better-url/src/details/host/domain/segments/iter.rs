//! [`BidiDetailsIter`].

use crate::prelude::*;

/// A borrowing [`Iterator`] over a [`BidiDetails`]'s [`BidiDetail`]s.
#[derive(Debug, Clone)]
pub struct BidiDetailsIter<'a> {
    /// The [`BidiDetails`].
    pub(crate) details: &'a BidiDetails,
    /// The [`Range`].
    pub(crate) range: Range<usize>,
}

impl BidiDetailsIter<'_> {
    /// The number of non-[`BidiDetail::Ltr`] segments in the range.
    pub(crate) fn count_ones(&self) -> usize {
        self.details.0.range(self.range.start + 1 .. self.range.end + 1).iter().filter(|&x| x).count()
    }

    /// The number of [`BidiDetail::Ltr`] segments in the range.
    pub fn count_ltr(&self) -> usize {
        self.details.0.range(self.range.start + 1 .. self.range.end + 1).iter().filter(|&x| !x).count()
    }

    /// The number of [`BidiDetail::Rtl`] segments in the range.
    pub fn count_rtl(&self) -> usize {
        match self.details.is_bidi() {
            false => 0,
            true  => self.count_ones(),
        }
    }

    /// The number of [`BidiDetail::Inv`] segments in the range.
    pub fn count_inv(&self) -> usize {
        match self.details.is_bidi() {
            false => self.count_ones(),
            true  => 0,
        }
    }

    /// Get a subrange.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let L = BidiDetail::Ltr;
    /// let R = BidiDetail::Rtl;
    /// let I = BidiDetail::Inv;
    ///
    /// let bidi_details = BidiDetails::try_from_iter([L, L, R, R, L, R]).unwrap();
    ///
    /// assert!(bidi_details.range(1..-1).unwrap().subrange(2.. ).unwrap().eq([R, L]));
    /// assert!(bidi_details.range(1..-1).unwrap().subrange(2..4).unwrap().eq([R, L]));
    ///
    /// assert!(bidi_details.range(1..-1).unwrap().subrange(2..5).is_none());
    /// ```
    pub fn subrange<B: RangeBounds<isize>>(&self, range: B) -> Option<Self> {
        let mut subrange = normalize_irange(range, self.len())?;

        subrange.start += self.range.start;
        subrange.end   += self.range.start;

        Some(Self {
            details: self.details,
            range  : subrange,
        })
    }

    /// Get a subrange.
    pub fn usubrange<B: RangeBounds<usize>>(&self, range: B) -> Option<Self> {
        let mut subrange = normalize_urange(range, self.len())?;

        subrange.start += self.range.start;
        subrange.end   += self.range.start;

        Some(Self {
            details: self.details,
            range  : subrange,
        })
    }

    pub(crate) fn set_urange(&self, index: isize) -> Option<Range<usize>> {
        let mut ret = Thing1::set_urange(index, self.len())?;

        ret.start += self.range.start;
        ret.end   += self.range.start;

        Some(ret)
    }

    pub(crate) fn insert_urange(&self, index: isize) -> Option<Range<usize>> {
        let mut ret = Thing2::insert_urange(index, self.len())?;

        ret.start += self.range.start;
        ret.end   += self.range.start;

        Some(ret)
    }
}

impl ExactSizeIterator for BidiDetailsIter<'_> {
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl Iterator for BidiDetailsIter<'_> {
    type Item = BidiDetail;

    fn next(&mut self) -> Option<Self::Item> {
        self.details.uget(self.range.next()?)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.details.uget(self.range.nth(n)?)
    }
}

impl DoubleEndedIterator for BidiDetailsIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.details.uget(self.range.next_back()?)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.details.uget(self.range.nth_back(n)?)
    }
}

impl From<BidiDetailsIter<'_>> for BidiDetails {
    fn from(value: BidiDetailsIter<'_>) -> Self {
        let mut ret: SmallBitVec = std::iter::once(false).chain(value.details.0.range(value.range.start + 1 .. value.range.end + 1).iter()).collect();

        if value.details.is_bidi() && !ret.all_false() {
            ret.set(0, true);
        }

        Self(ret)
    }
}

impl<'a> IntoIterator for &'a BidiDetails {
    type IntoIter = BidiDetailsIter<'a>;
    type Item = BidiDetail;

    fn into_iter(self) -> Self::IntoIter {
        BidiDetailsIter {
            details: self,
            range  : 0..self.len()
        }
    }
}
