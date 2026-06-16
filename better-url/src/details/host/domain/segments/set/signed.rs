use crate::prelude::*;

impl BidiDetails {
    /// Set the `index`th [`BidiDetail`].
    /// # Errors
    /// If [validity](#validity) would be violated, returns the error [`InvalidDomainSegments`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let L = BidiDetail::Ltr;
    /// let I = BidiDetail::Inv;
    /// let R = BidiDetail::Rtl;
    ///
    /// let mut x = BidiDetails::try_from_iter([L, R, L, R]).unwrap();
    ///
    /// x.set( 0, R).unwrap(); assert!(x.iter().eq([   R, R, L, R   ]));
    /// x.set(-1, L).unwrap(); assert!(x.iter().eq([   R, R, L, L   ]));
    /// x.set( 4, R).unwrap(); assert!(x.iter().eq([   R, R, L, L, R]));
    /// x.set(-6, L).unwrap(); assert!(x.iter().eq([L, R, R, L, L, R]));
    ///
    /// x.set(-8, L).unwrap_err();
    /// x.set( 7, L).unwrap_err();
    ///
    /// let mut x = BidiDetails::try_from_iter([L, L]).unwrap();
    ///
    /// x.set(0, R).expect("1");
    /// x.set(1, I).expect_err("2");
    ///
    /// x.set(0, I).expect("3");
    /// x.set(1, R).expect_err("4");
    /// ```
    pub fn set(&mut self, index: isize, detail: BidiDetail) -> Result<(), SetBidiDetailsError> {
        match index {
            0..                                           => self.uset(index as usize, detail),
            ..0 if self.len() + 1 == index.unsigned_abs() => self.uinsert(0, detail),
            ..0                                           => self.uset(self.len().checked_add_signed(index).ok_or(InsertNotFound)?, detail)
        }
    }

    /// Set the `range` of [`BidiDetail`]s.
    /// # Errors
    /// If the range isn't found, returns the error [`RangeNotFound`].
    ///
    /// If [validity](#validity) would be violated, returns the error [`InvalidDomainSegments`].
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
    /// x.set_range(- 5..  1, &[R, R, R].try_into().unwrap()).unwrap    (); assert!(x.iter().eq([R, R, R, L, R, R]));
    /// x.set_range(-10.. -3, &[L      ].try_into().unwrap()).unwrap    (); assert!(x.iter().eq([L, L, R, R      ]));
    /// x.set_range(  1..= 2, &[R      ].try_into().unwrap()).unwrap    (); assert!(x.iter().eq([L, R, R         ]));
    /// x.set_range(  1..= 1, &[I      ].try_into().unwrap()).unwrap_err(); assert!(x.iter().eq([L, R, R         ]));
    /// x.set_range(  1..= 2, &[I      ].try_into().unwrap()).unwrap    (); assert!(x.iter().eq([L, I            ]));
    /// x.set_range( -3..= 0, &[R      ].try_into().unwrap()).unwrap_err(); assert!(x.iter().eq([L, I            ]));
    /// x.set_range( -3..= 3, &[R      ].try_into().unwrap()).unwrap    (); assert!(x.iter().eq([R               ]));
    /// x.set_range(  1..= 3, &[R      ].try_into().unwrap()).unwrap    (); assert!(x.iter().eq([R, R            ]));
    /// ```
    pub fn set_range<B: RangeBounds<isize>>(&mut self, range: B, details: &Self) -> Result<(), SetBidiDetailsError> {
        self.set_urange(range_intersection(range, self.len()).ok_or(RangeNotFound)?, details)
    }

    /// Insert a new `index`th [`BidiDetail`].
    /// # Errors
    /// If `index` is more negative than [`Self::len`] is positive, returns the error [`InsertNotFound`].
    ///
    /// If the call to [`Self::uinsert`] reutrns an error, that error is returned.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let L = BidiDetail::Ltr;
    /// let I = BidiDetail::Inv;
    /// let R = BidiDetail::Rtl;
    ///
    /// let mut x = BidiDetails::try_from_iter([L, R, L, R]).unwrap();
    ///
    /// x.insert(-5, R).unwrap(); assert!(x.iter().eq([R, L, R, L, R]));
    /// x.insert(-1, L).unwrap(); assert!(x.iter().eq([R, L, R, L, R, L]));
    /// ```
    pub fn insert(&mut self, index: isize, detail: BidiDetail) -> Result<(), SetBidiDetailsError> {
        match index {
            0.. => self.uinsert(index as usize, detail),
            ..0 => self.uinsert(self.len().checked_sub(index.unsigned_abs() - 1).ok_or(InsertNotFound)?, detail)
        }
    }
}
