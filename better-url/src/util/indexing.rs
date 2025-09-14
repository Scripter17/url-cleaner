//! Indexing utilities.

/// Simulates Python's indexing to allow using `-1` to mean the last element.
pub const fn neg_index(index: isize, len: usize) -> Option<usize> {
    if index < 0 {
        len.checked_add_signed(index)
    } else if (index as usize) < len {
        Some(index as usize)
    } else {
        None
    }
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
}

