//! Segment utilities.

use crate::prelude::*;

/// Gets the range of elements form `iter`.
///
/// Technically the things this is used for shouldn't be using this at all, but for now it works.
pub(crate) fn neg_vec_keep<I: IntoIterator>(iter: I, start: isize, end: Option<isize>) -> Option<Vec<I::Item>> {
    let mut ret=iter.into_iter().collect::<Vec<_>>();
    Some(ret.drain(neg_range(start, end, ret.len())?).collect())
}
