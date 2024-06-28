//! Various helper functions and macros.
//! Very weird, very jank.
//! May or may not be made pub if useful.
//! Which is to say probably not.

use std::ops::Bound;

mod macros;
pub(crate) use macros::*;

/// For use with [`#[serde(default, skip_serializing_if = "...")]`](https://serde.rs/field-attrs.html#skip_serializing_if).
pub(crate) fn is_default<T: Default + PartialEq>(t: &T) -> bool {t == &T::default()}

/// Loops negative `index`es around similar to Python.
pub(crate) const fn neg_index(index: isize, len: usize) -> Option<usize> {
    if 0<=index && (index as usize)<len {
        Some(index as usize)
    } else if index<0 {
        len.checked_sub(index.unsigned_abs())
    } else {
        None
    }
}

/// [`neg_index`] but doesn't [`None`] when `index == len`.
pub(crate) const fn neg_range_boundary(index: isize, len: usize) -> Option<usize> {
    if index >= 0 && index as usize == len {
        Some(index as usize)
    } else {
        neg_index(index, len)
    }
}

/// Used for inserting after segments.
#[allow(clippy::arithmetic_side_effects)]
pub(crate) const fn neg_shifted_range_boundary(index: isize, len: usize, shift: isize) -> Option<usize> {
    if let Some(x) = neg_range_boundary(index, len) {
        if x as isize + shift <= 0 || x as isize + shift > len as isize {
            None
        } else {
            Some((x as isize + shift) as usize)
        }
    } else {
        None
    }
}

/// Equivalent to how python handles indexing. Minus the panicking, of course.
pub(crate) fn neg_nth<I: IntoIterator>(iter: I, i: isize) -> Option<I::Item> {
    if i<0 {
        let temp=iter.into_iter().collect::<Vec<_>>();
        let fixed_i=neg_index(i, temp.len())?;
        temp.into_iter().nth(fixed_i)
    } else {
        iter.into_iter().nth(i as usize)
    }
}

/// [`neg_maybe_index`] but doesn't [`None`] when `index == len`.
fn neg_maybe_range_boundary(index: Option<isize>, len: usize) -> Option<Option<usize>> {
    index.map(|index| neg_index(index, len))
}

/// A range that may or may not have one or both ends open.
pub(crate) fn neg_range(start: Option<isize>, end: Option<isize>, len: usize) -> Option<(Bound<usize>, Bound<usize>)> {
    match (start, end) {
        (Some(start), Some(end)) if neg_index(end, len)? < neg_index(start, len)? => None,
        _ => Some((
            match neg_maybe_range_boundary(start, len) {
                Some(Some(start)) => Bound::Included(start),
                Some(None       ) => None?,
                None              => Bound::Unbounded
            },
            match neg_maybe_range_boundary(end, len) {
                Some(Some(end)) => Bound::Excluded(end),
                Some(None     ) => None?,
                None            => Bound::Unbounded
            }
        ))
    }
}

/// Makes a [`Vec`] from `iter` then keeps only elements specified by [`neg_range`]ing `start` and `end`.
pub(crate) fn neg_vec_keep<I: IntoIterator>(iter: I, start: Option<isize>, end: Option<isize>) -> Option<Vec<I::Item>> {
    let mut ret=iter.into_iter().collect::<Vec<_>>();
    Some(ret.drain(neg_range(start, end, ret.len())?).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
