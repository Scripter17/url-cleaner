//! General utility functions.

use std::ops::Bound;

use thiserror::Error;

mod macros;
pub(crate) use macros::*;
mod suitability;
pub(crate) use suitability::*;

/// Serde helper function that returns true if `x` is `T`'s [`Default::default`] value.
pub(crate) fn is_default<T: Default + PartialEq>(x: &T) -> bool {x == &T::default()}

/// Simulates Python's indexing to allow using `-1` to mean the last element.
pub(crate) const fn neg_index(index: isize, len: usize) -> Option<usize> {
    if index < 0 {
        len.checked_add_signed(index)
    } else if (index as usize) < len {
        Some(index as usize)
    } else {
        None
    }
}

/// [`neg_index`] but if the index is equal to the length, doesn't return [`None`].
///
/// Useful for [`Vec::insert`] type functions.
pub(crate) const fn neg_range_boundary(index: isize, len: usize) -> Option<usize> {
    if index as usize == len {
        Some(len)
    } else {
        neg_index(index, len)
    }
}

/// Returns [`true`] if [`Ord::clamp`] returns `x`.
fn is_inside_range<T: Ord + Copy>(x: T, min: T, max: T) -> bool {
    x.clamp(min, max) == x
}

/// [`neg_range_boundary`] but shifts its result while keeping it in bounds.
#[expect(clippy::arithmetic_side_effects, reason = "Shouldn't ever occur.")]
pub(crate) fn neg_shifted_range_boundary(index: isize, len: usize, shift: isize) -> Option<usize> {
    neg_range_boundary(index, len)?.checked_add_signed(shift).filter(|x| is_inside_range(*x, 0, len))
}

/// Gets the [`neg_index`] item of `iter`.
pub(crate) fn neg_nth<I: IntoIterator>(iter: I, i: isize) -> Option<I::Item> {
    if i<0 {
        let temp=iter.into_iter().collect::<Vec<_>>();
        let fixed_i=neg_index(i, temp.len())?;
        temp.into_iter().nth(fixed_i)
    } else {
        iter.into_iter().nth(i as usize)
    }
}

/// Return the range between `start` (inclusive) and `end` (exclusive) if the range is within `0..=len` and the resulting range is in ascending order.
pub(crate) fn neg_range(start: isize, end: Option<isize>, len: usize) -> Option<(Bound<usize>, Bound<usize>)> {
    let ret = (
        Bound::Included(neg_range_boundary(start, len)?),
        match end {
            Some(end) => Bound::Excluded(neg_range_boundary(end, len)?),
            None => Bound::Unbounded
        }
    );

    // If the resulting range is "backwards", return None.
    if matches!(ret, (Bound::Included(start), Bound::Excluded(end)) if end < start) {return None;}

    Some(ret)
}

/// Gets the range of elements form `iter`.
///
/// Technically the things this is used for shouldn't be using this at all, but for now it works.
pub(crate) fn neg_vec_keep<I: IntoIterator>(iter: I, start: isize, end: Option<isize>) -> Option<Vec<I::Item>> {
    let mut ret=iter.into_iter().collect::<Vec<_>>();
    Some(ret.drain(neg_range(start, end, ret.len())?).collect())
}

/// Serde helper function.
pub(crate) const fn get_false() -> bool {false}
/// Serde helper function.
pub(crate) const fn is_false(x: &bool) -> bool {!*x}
/// Serde helper function.
pub(crate) const fn get_true() -> bool {true}
/// Serde helper function.
pub(crate) const fn is_true(x: &bool) -> bool {*x}

/// Converts an `end` bound to a [`Bound`].
///
/// Specifically, if `i` is [`Some`], return [`Bound::Excluded`] or [`Bound::Unbounded`] if it's [`None`].
pub(crate) fn exorub(i: Option<usize>) -> Bound<usize> {
    match i {
        Some(i) => Bound::Excluded(i),
        None => Bound::Unbounded
    }
}

/// Returned by [`set_segment_range`] when trying to construct an invalid range.
#[derive(Debug, Error)]
#[error("Tried to construct an invalid range.")]
pub struct InvalidRange;

/// Removes the range from `x` then, if `to` is [`Some`], insert it at where the range used to be.
pub(crate) fn set_segment_range<T>(x: &mut Vec<T>, start: isize, end: Option<isize>, to: Option<T>) -> Result<(), InvalidRange> {
    let fixed_start = neg_index(start, x.len()).ok_or(InvalidRange)?;
    let range = neg_range(start, end, x.len()).ok_or(InvalidRange)?;
    x.drain(range).for_each(drop);
    if let Some(to) = to {
        x.insert(fixed_start, to);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Bound;

    #[test]
    fn neg_index_test() {
        assert_eq!(neg_index(-4, 3), None   );
        assert_eq!(neg_index(-3, 3), Some(0));
        assert_eq!(neg_index(-2, 3), Some(1));
        assert_eq!(neg_index(-1, 3), Some(2));
        assert_eq!(neg_index( 0, 3), Some(0));
        assert_eq!(neg_index( 1, 3), Some(1));
        assert_eq!(neg_index( 2, 3), Some(2));
        assert_eq!(neg_index( 3, 3), None   );
        assert_eq!(neg_index( 4, 3), None   );
    }

    #[test]
    fn neg_nth_test() {
        assert_eq!(neg_nth([1,2,3], -4), None   );
        assert_eq!(neg_nth([1,2,3], -3), Some(1));
        assert_eq!(neg_nth([1,2,3], -2), Some(2));
        assert_eq!(neg_nth([1,2,3], -1), Some(3));
        assert_eq!(neg_nth([1,2,3],  0), Some(1));
        assert_eq!(neg_nth([1,2,3],  1), Some(2));
        assert_eq!(neg_nth([1,2,3],  2), Some(3));
        assert_eq!(neg_nth([1,2,3],  3), None   );
    }

    #[test]
    fn neg_range_test() {
        assert_eq!(neg_range(-3, Some(-3), 2), None);
        assert_eq!(neg_range(-2, Some(-3), 2), None);
        assert_eq!(neg_range(-1, Some(-3), 2), None);
        assert_eq!(neg_range( 0, Some(-3), 2), None);
        assert_eq!(neg_range( 1, Some(-3), 2), None);
        assert_eq!(neg_range( 2, Some(-3), 2), None);
        assert_eq!(neg_range( 3, Some(-3), 2), None);

        assert_eq!(neg_range(-3, Some(-2), 2), None);
        assert_eq!(neg_range(-2, Some(-2), 2), Some((Bound::Included(0), Bound::Excluded(0))));
        assert_eq!(neg_range(-1, Some(-2), 2), None);
        assert_eq!(neg_range( 0, Some(-2), 2), Some((Bound::Included(0), Bound::Excluded(0))));
        assert_eq!(neg_range( 1, Some(-2), 2), None);
        assert_eq!(neg_range( 2, Some(-2), 2), None);
        assert_eq!(neg_range( 3, Some(-2), 2), None);

        assert_eq!(neg_range(-3, Some(-1), 2), None);
        assert_eq!(neg_range(-2, Some(-1), 2), Some((Bound::Included(0), Bound::Excluded(1))));
        assert_eq!(neg_range(-1, Some(-1), 2), Some((Bound::Included(1), Bound::Excluded(1))));
        assert_eq!(neg_range( 0, Some(-1), 2), Some((Bound::Included(0), Bound::Excluded(1))));
        assert_eq!(neg_range( 1, Some(-1), 2), Some((Bound::Included(1), Bound::Excluded(1))));
        assert_eq!(neg_range( 2, Some(-1), 2), None);
        assert_eq!(neg_range( 3, Some(-1), 2), None);

        assert_eq!(neg_range(-3, Some( 0), 2), None);
        assert_eq!(neg_range(-2, Some( 0), 2), Some((Bound::Included(0), Bound::Excluded(0))));
        assert_eq!(neg_range(-1, Some( 0), 2), None);
        assert_eq!(neg_range( 0, Some( 0), 2), Some((Bound::Included(0), Bound::Excluded(0))));
        assert_eq!(neg_range( 1, Some( 0), 2), None);
        assert_eq!(neg_range( 2, Some( 0), 2), None);
        assert_eq!(neg_range( 3, Some( 0), 2), None);

        assert_eq!(neg_range(-3, Some( 1), 2), None);
        assert_eq!(neg_range(-2, Some( 1), 2), Some((Bound::Included(0), Bound::Excluded(1))));
        assert_eq!(neg_range(-1, Some( 1), 2), Some((Bound::Included(1), Bound::Excluded(1))));
        assert_eq!(neg_range( 0, Some( 1), 2), Some((Bound::Included(0), Bound::Excluded(1))));
        assert_eq!(neg_range( 1, Some( 1), 2), Some((Bound::Included(1), Bound::Excluded(1))));
        assert_eq!(neg_range( 2, Some( 1), 2), None);
        assert_eq!(neg_range( 3, Some( 1), 2), None);

        assert_eq!(neg_range(-3, Some( 2), 2), None);
        assert_eq!(neg_range(-2, Some( 2), 2), Some((Bound::Included(0), Bound::Excluded(2))));
        assert_eq!(neg_range(-1, Some( 2), 2), Some((Bound::Included(1), Bound::Excluded(2))));
        assert_eq!(neg_range( 0, Some( 2), 2), Some((Bound::Included(0), Bound::Excluded(2))));
        assert_eq!(neg_range( 1, Some( 2), 2), Some((Bound::Included(1), Bound::Excluded(2))));
        assert_eq!(neg_range( 2, Some( 2), 2), Some((Bound::Included(2), Bound::Excluded(2))));
        assert_eq!(neg_range( 3, Some( 2), 2), None);

        assert_eq!(neg_range(-3, Some( 3), 2), None);
        assert_eq!(neg_range(-2, Some( 3), 2), None);
        assert_eq!(neg_range(-1, Some( 3), 2), None);
        assert_eq!(neg_range( 0, Some( 3), 2), None);
        assert_eq!(neg_range( 1, Some( 3), 2), None);
        assert_eq!(neg_range( 2, Some( 3), 2), None);
        assert_eq!(neg_range( 3, Some( 3), 2), None);
    }
}
