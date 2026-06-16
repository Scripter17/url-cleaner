//! [`BidiDetails`].

use crate::prelude::*;

mod get;
mod set;
mod iter;
mod into_iter;

pub use iter::*;
pub use into_iter::*;

/// [`BidiDetail`]s about a list of zero or more domain segments.
/// # Validity
/// A domain with a [`BidiDetail::Rtl`] label requires that all of its labels satisfy the [Bidi rule](https://www.rfc-editor.org/info/rfc5893/#section-2).
///
/// That is, a domain can contain [`BidiDetail::Ltr`] and [`BidiDetail::Inv`] segments or it can contain [`BidiDetail::Ltr`] and [`BidiDetail::Inv`] segments, but it cannot contain both [`BidiDetail::Inv`] and [`BidiDetail::Rtl`] segments.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BidiDetails(pub(crate) SmallBitVec);

impl BidiDetails {
    /// Try to make a new [`Self`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new<T: TryInto<Self>>(value: T) -> Result<Self, T::Error> {
        value.try_into()
    }

    /// Try to make a new [`Self`] from an [`Iterator`] of [`BidiDetail`]s.
    /// # Errors
    /// If any call to [`Self::try_push`] returns an error, that error is returned.
    pub fn try_from_iter<I: IntoIterator<Item = BidiDetail>>(iter: I) -> Result<Self, InvalidDomainSegments> {
        let mut ret = Self::default();

        for x in iter {
            ret.try_push(x)?;
        }

        Ok(ret)
    }

    /// The number of segments.
    pub fn len(&self) -> usize {
        self.0.len() - 1
    }

    /// If there's zero elements.
    pub fn is_empty(&self) -> bool {
        self.0.len() == 1
    }

    /// Make a [`BidiDetailsIter`].
    pub fn iter(&self) -> BidiDetailsIter<'_> {
        self.into_iter()
    }

    /// If it's bidi.
    pub fn is_bidi(&self) -> bool {
        unsafe {
            self.0.get_unchecked(0)
        }
    }

    /// The number of non-[`BidiDetail::Ltr`] segments in the range.
    fn count_ones(&self) -> usize {
        self.0.iter().filter(|&x| x).count() - self.is_bidi() as usize
    }

    /// The number of [`BidiDetail::Ltr`] segments.
    pub fn count_ltr(&self) -> usize {
        self.0.iter().filter(|&x| !x).count() - !self.is_bidi() as usize
    }

    /// The number of [`BidiDetail::Rtl`] segments.
    pub fn count_rtl(&self) -> usize {
        match self.is_bidi() {
            false => 0,
            true  => self.count_ones(),
        }
    }

    /// The number of [`BidiDetail::Inv`] segments.
    pub fn count_inv(&self) -> usize {
        match self.is_bidi() {
            false => self.count_ones(),
            true  => 0,
        }
    }

    /// The "mode" after setting `index`.
    ///
    /// - `Ok(false)` => Not Bidi.
    /// - `Ok(true)` => Bidi.
    /// - `Err(_)` => Invalid set.
    /// # Errors
    /// If `index` is greater than [`Self::len`], returns the error [`InsertNotFound`].
    ///
    /// If the set would break [validity](#validity), returns the error [`InvalidDomainSegments`].
    pub fn mode_after_uset(&self, index: usize, detail: BidiDetail) -> Result<bool, SetBidiDetailsError> {
        if index > self.len() {
            Err(InsertNotFound)?;
        }

        Ok(match (self.is_bidi(), detail) {
            (x, BidiDetail::Ltr) => x,

            (false, BidiDetail::Inv) => false,
            (false, BidiDetail::Rtl) => {
                if !self.0.all_false() {
                    Err(InvalidDomainSegments)?;
                }

                true
            },

            (true, BidiDetail::Inv) => match self.count_ones() {
                1 if unsafe {self.0.get_unchecked(index + 1)} => false,
                _ => Err(InvalidDomainSegments)?
            },
            (true, BidiDetail::Rtl) => true,
            
        })
    }

    /// The "mode" after setting `range`.
    ///
    /// - `Ok(false)` => Not Bidi.
    /// - `Ok(true)` => Bidi.
    /// - `Err(_)` => Invalid set.
    /// # Errors
    /// If the call to [`Self::urange`] reutrns [`None`], returns the error [`RangeNotFound`].
    ///
    /// If the set would break [validity](#validity), returns the error [`InvalidDomainSegments`].
    pub fn mode_after_set_urange<B: RangeBounds<usize>>(&self, range: B, with: &Self) -> Result<bool, SetBidiDetailsError> {
        Ok(match (self.is_bidi(), with.is_bidi()) {
            (false, false) => false,
            (true , true ) => true ,

            (false, true) => {
                if !self.0.all_false() {
                    let x = self.count_ones();
                    let y = self.urange(range).ok_or(RangeNotFound)?.count_ones();

                    if x > y {
                        Err(InvalidDomainSegments)?;
                    }
                }

                true
            },

            (true, false) => {
                let x = self.count_ones();
                let y = self.urange(range).ok_or(RangeNotFound)?.count_ones();

                if x > y && !with.0.all_false() {
                    Err(InvalidDomainSegments)?;
                }

                x > y
            }
        })
    }

    /// The "mode" after inserting `detail`.
    ///
    /// - `Ok(false)` => Not Bidi.
    /// - `Ok(true)` => Bidi.
    /// - `Err(_)` => Invalid insert.
    /// # Errors
    /// If the insert would violate [validity](#validity), returns the error [`InvalidDomainSegments`].
    pub fn mode_after_uinsert(&self, index: usize, detail: BidiDetail) -> Result<bool, SetBidiDetailsError> {
        if index > self.len() {
            Err(InsertNotFound)?;
        }

        Ok(match (self.is_bidi(), detail) {
            (x, BidiDetail::Ltr) => x,

            (false, BidiDetail::Inv) => false,
            (false, BidiDetail::Rtl) => {
                if !self.0.all_false() {
                    Err(InvalidDomainSegments)?;
                }
                true
            },

            (true, BidiDetail::Inv) => Err(InvalidDomainSegments)?,
            (true, BidiDetail::Rtl) => true
        })
    }
}



impl Default for BidiDetails {
    fn default() -> Self {
        Self(SmallBitVec::from_elem(1, false))
    }
}



impl From<BidiDetail> for BidiDetails {
    fn from(value: BidiDetail) -> Self {
        Self(match value {
            BidiDetail::Ltr => [false, false],
            BidiDetail::Rtl => [true , true ],
            BidiDetail::Inv => [false, true ],
        }.into_iter().collect())
    }
}



impl TryFrom<&[BidiDetail]> for BidiDetails {
    type Error = InvalidDomainSegments;

    fn try_from(value: &[BidiDetail]) -> Result<Self, Self::Error> {
        let mode = value.contains(&BidiDetail::Rtl);

        if mode && value.contains(&BidiDetail::Inv) {
            Err(InvalidDomainSegments)?;
        }

        Ok(Self(SmallBitVec::from_iter(std::iter::once(mode).chain(value.iter().map(|x| match x {
            BidiDetail::Ltr => false,
            BidiDetail::Inv => true,
            BidiDetail::Rtl => true,
        })))))
    }
}

impl<const N: usize> TryFrom<[BidiDetail; N]> for BidiDetails {
    type Error = InvalidDomainSegments;

    fn try_from(value: [BidiDetail; N]) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl<const N: usize> TryFrom<&[BidiDetail; N]> for BidiDetails {
    type Error = InvalidDomainSegments;

    fn try_from(value: &[BidiDetail; N]) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}
