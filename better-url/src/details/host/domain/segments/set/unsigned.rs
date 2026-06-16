use crate::prelude::*;

impl BidiDetails {
    /// Set the `index`th [`BidiDetail`].
    /// # Errors
    /// If [validity](#validity) would be violated, returns the error [`InvalidDomainSegments`].
    pub fn uset(&mut self, index: usize, detail: BidiDetail) -> Result<(), SetBidiDetailsError> {
        unsafe {
            self.0.set_unchecked(0, self.mode_after_uset(index, detail)?);
            match index == self.len() {
                true  => self.0.push         (           detail != BidiDetail::Ltr),
                false => self.0.set_unchecked(index + 1, detail != BidiDetail::Ltr),
            }
        }

        Ok(())
    }
    
    /// Set a range of [`BidiDetail`]s.
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
    /// x.set_range(1..10, &[R, L].try_into().unwrap()).unwrap    (); assert!(x.iter().eq([L, R, L      ]));
    /// x.set_range(2..10, &[I, L].try_into().unwrap()).unwrap_err(); assert!(x.iter().eq([L, R, L      ]));
    /// x.set_range(3..10, &[R, L].try_into().unwrap()).unwrap    (); assert!(x.iter().eq([L, R, L, R, L]));
    /// ```
    pub fn set_urange<B: RangeBounds<usize>>(&mut self, range: B, details: &Self) -> Result<(), SetBidiDetailsError> {
        let replace = urange_intersection(range, self.len()).ok_or(RangeNotFound)?;

        let mode = self.mode_after_set_urange(replace.clone(), details)?;

        unsafe {
            match details.len().cmp(&replace.len()) {
                Ordering::Equal => {},

                Ordering::Greater => {
                    let x = details.len() - replace.len();

                    self.0.resize(self.0.len() + x, false);

                    for i in (replace.end + x .. self.len()).rev() {
                        self.0.set_unchecked(i + 1, self.0.get_unchecked(i + 1 - x));
                    }
                },

                Ordering::Less => {
                    let x = replace.len() - details.len();

                    for i in replace.end - x .. self.len() - x {
                        self.0.set_unchecked(i + 1, self.0.get_unchecked(i + 1 + x));
                    }

                    self.0.truncate(self.0.len() - x);
                },
            }

            self.0.set_unchecked(0, mode);

            for (i, x) in (replace.start..).zip(details.0.iter().skip(1)) {
                self.0.set_unchecked(i + 1, x);
            }
        }

        Ok(())
    }

    /// Insert a new `index`th [`BidiDetail`].
    /// # Errors
    /// See [#validity] for details.
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
    /// x.uinsert(0      , L).unwrap(); assert!(x.iter().eq([L, L, R,    L, R   ]));
    /// x.uinsert(x.len(), L).unwrap(); assert!(x.iter().eq([L, L, R,    L, R, L]));
    /// x.uinsert(3      , R).unwrap(); assert!(x.iter().eq([L, L, R, R, L, R, L]));
    /// ```
    pub fn uinsert(&mut self, index: usize, detail: BidiDetail) -> Result<(), SetBidiDetailsError> {
        unsafe {
            self.0.set_unchecked(0, self.mode_after_uinsert(index, detail)?);

            self.0.push(false);

            for i in (index..self.len()).rev() {
                self.0.set_unchecked(i + 1, self.0.get_unchecked(i));
            }

            self.0.set_unchecked(index + 1, detail != BidiDetail::Ltr);
        }

        Ok(())
    }
}
