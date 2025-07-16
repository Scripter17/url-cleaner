//! Indexing utilities.

use std::ops::Bound;

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
