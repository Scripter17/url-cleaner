//! General utility functions.

use std::ops::Bound;
use std::borrow::Cow;

mod macros;
pub(crate) use macros::*;
mod suitability;
pub(crate) use suitability::*;
#[cfg(feature = "debug")]
mod debug;
#[cfg(feature = "debug")]
pub(crate) use debug::*;

/// Dud debug macro.
#[cfg(not(feature = "debug"))]
macro_rules! debug {($($_:tt)*) => {}}
#[cfg(not(feature = "debug"))]
pub(crate) use debug;

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

/// Percent encoding helper.
pub(crate) fn peh(s: &str) -> Cow<'_, str> {
    percent_encoding::percent_decode_str(s).decode_utf8_lossy()
}

/// Helper method.
/// # Errors
/// If the call to [`neg_index`] returns [`None`], returns the error provided in `segment_not_found`.
pub(crate) fn set_segment<'a, E>(part: &'a str, index: isize, value: Option<&'a str>, segment_not_found: E, split: char) -> Result<Vec<&'a str>, E> {
    let mut segments = part.split(split).collect::<Vec<_>>();
    let index = neg_index(index, segments.len()).ok_or(segment_not_found)?;
    match value {
        #[expect(clippy::indexing_slicing, reason = "Can't happen.")]
        Some(value) => segments[index] = value,
        None => {segments.remove(index);}
    }
    Ok(segments)
}

/// Helper method.
///
/// Assumes `split` is one byte but this is only called with `.` and `/` so who cares.
/// # Errors
/// If the call to [`neg_index`] returns [`None`], returns the error provided in `segment_not_found`.
pub(crate) fn insert_segment_at<E>(part: &str, index: isize, value: &str, segment_not_found: E, split: char, join: &str) -> Result<String, E> {
    use std::ops::Bound;
    #[expect(clippy::arithmetic_side_effects, reason = "Can't happen.")]
    let start_of_first_shifted_segment = (match index {
        0.. => part.split(split).nth(index as usize),
        ..0 => part.split(split).nth_back((-index) as usize)
    }.ok_or(segment_not_found)?.as_ptr() as usize) - (part.as_ptr() as usize);
    Ok(format!(
        "{}{value}{join}{}",
        part.get((Bound::Unbounded, Bound::Excluded(start_of_first_shifted_segment))).expect("This to be written right."),
        part.get((Bound::Included(start_of_first_shifted_segment), Bound::Unbounded)).expect("This to be written right.")
    ))
}

/// Helper method.
/// # Errors
/// If the call to [`neg_index`] returns [`None`], returns the error provided in `segment_not_found`.
pub(crate) fn insert_segment_after<E>(part: &str, index: isize, value: &str, segment_not_found: E, split: char, join: &str) -> Result<String, E> {
    let mut segments = part.split(split).collect::<Vec<_>>();
    #[expect(clippy::arithmetic_side_effects, reason = "Can't happen.")]
    segments.insert(neg_index(index, segments.len()).ok_or(segment_not_found)? + 1, value);
    Ok(segments.join(join))
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
