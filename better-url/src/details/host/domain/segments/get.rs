use crate::prelude::*;

impl BidiDetails {
    /// The `index`th [`BidiDetail`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let L = BidiDetail::Ltr;
    /// let R = BidiDetail::Rtl;
    /// let I = BidiDetail::Inv;
    ///
    /// let mut x = BidiDetails::try_from_iter([L, L, R, R]).unwrap();
    ///
    /// assert_eq!(x.get(0), Some(L));
    /// assert_eq!(x.get(1), Some(L));
    /// assert_eq!(x.get(2), Some(R));
    /// assert_eq!(x.get(3), Some(R));
    /// assert_eq!(x.get(4), None   );
    ///
    /// assert_eq!(x.get(-1), Some(R));
    /// assert_eq!(x.get(-2), Some(R));
    /// assert_eq!(x.get(-3), Some(L));
    /// assert_eq!(x.get(-4), Some(L));
    /// assert_eq!(x.get(-5), None   );
    /// ```
    pub fn get(&self, index: isize) -> Option<BidiDetail> {
        self.uget(normalize_index(index, self.len())?)
    }

    /// The `index`th [`BidiDetail`].
    pub fn uget(&self, index: usize) -> Option<BidiDetail> {
        Some(match (self.is_bidi(), self.0.get(index + 1)?) {
            (_    , false) => BidiDetail::Ltr,
            (false, true ) => BidiDetail::Inv,
            (true , true ) => BidiDetail::Rtl,
        })
    }

    /// If [`Self::len`] is `1`, return the only [`BidiDetail`].
    pub fn only(&self) -> Option<BidiDetail> {
        match self.len() {
            1 => self.uget(0),
            _ => None
        }
    }
    /// [`BidiDetailsIter::subrange`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<BidiDetailsIter<'_>> {
        self.iter().subrange(range)
    }

    /// [`BidiDetailsIter::usubrange`].
    pub fn urange<B: RangeBounds<usize>>(&self, range: B) -> Option<BidiDetailsIter<'_>> {
        self.iter().usubrange(range)
    }
}
