//! Indexing utilities.

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
}

