use std::io::Error as IoError;
use std::ops::Bound;
use std::string::FromUtf8Error;

use url::ParseError;
use thiserror::Error;

mod url_part;
pub use url_part::*;
mod string_location;
pub use string_location::*;
mod string_modification;
pub use string_modification::*;
mod string_source;
pub use string_source::*;
mod string_matcher;
pub use string_matcher::*;

/// An enum that, if I've done my job properly, contains any possible error that can happen when cleaning a URL.
/// Except for if a [`crate::rules::mappers::Mapper::ExpandShortLink`] response can't be cached. That error is ignored pending a version of [`Result`] that can handle partial errors.
/// Not only is it a recoverable error, it's an error that doesn't need to be recovered from.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum CleaningError {
    /// There was an error getting the config.
    #[error(transparent)]
    GetConfigError(#[from] crate::config::GetConfigError),
    /// There was an error executing a rule.
    #[error(transparent)]
    RuleError(#[from] crate::rules::RuleError),
    /// There was an error parsing the URL.
    #[error(transparent)]
    UrlParseError(#[from] ParseError),
    /// IO error.
    #[error(transparent)]
    IoError(#[from] IoError)
}

#[derive(Debug, Error)]
pub enum StringError {
    /// The requested slice was either not on a UTF-8 boundary or out of bounds.
    #[error("The requested slice was either not on a UTF-8 boundary or out of bounds.")]
    InvalidSlice,
    /// The requested index was either not on a UTF-8 boundary or out of bounds.
    #[error("The requested index was either not on a UTF-8 boundary or out of bounds.")]
    InvalidIndex,
    /// The requested segment was not found.
    #[error("The requested segment was not found.")]
    SegmentNotFound,
    /// The provided string did not start with the requested prefix.
    #[error("The string being modified did not start with the provided prefix. Maybe try `StringModification::StripMaybePrefix`?")]
    PrefixNotFound,
    /// The provided string did not end with the requested prefix.
    #[error("The string being modified did not end with the provided suffix. Maybe try `StringModification::StripMaybeSuffix`?")]
    SuffixNotFound,
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
}

/// Loops negative `index`es around similar to Python.
pub(crate) const fn neg_index(index: isize, len: usize) -> Option<usize> {
    if 0<=index && (index as usize)<=len {
        Some(index as usize)
    } else if index<0 {
        len.checked_sub(index.unsigned_abs())
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

/// `f` but allows for `None` to represent open range ends.
fn neg_maybe_index(index: Option<isize>, len: usize) -> Option<Option<usize>> {
    index.map(|index| neg_index(index, len))
}

/// A range that may or may not have one or both ends open.
pub(crate) fn neg_range(start: Option<isize>, end: Option<isize>, len: usize) -> Option<(Bound<usize>, Bound<usize>)> {
    Some((
        match neg_maybe_index(start, len) {
            Some(Some(start)) => Bound::Included(start),
            Some(None       ) => None?,
            None              => Bound::Unbounded
        },
        match neg_maybe_index(end, len) {
            Some(Some(end)) => Bound::Excluded(end),
            Some(None     ) => None?,
            None            => Bound::Unbounded
        }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neg_index_test() {
        assert_eq!(neg_index( 0, 5), Some(0));
        assert_eq!(neg_index( 4, 5), Some(4));
        assert_eq!(neg_index(-1, 5), Some(4));
        assert_eq!(neg_index( 6, 5), None   );
        assert_eq!(neg_index(-6, 5), None   );
    }

    #[test]
    fn neg_nth_test() {
        assert_eq!(neg_nth([1,2,3],  0), Some(1));
        assert_eq!(neg_nth([1,2,3], -1), Some(3));
        assert_eq!(neg_nth([1,2,3], -4), None);
    }

    #[test]
    fn neg_range_test() {
        assert_eq!(neg_range(Some( 3), None   , 10), Some((Bound::Included( 3), Bound::Unbounded)));
        assert_eq!(neg_range(Some(10), None   , 10), Some((Bound::Included(10), Bound::Unbounded)));
        assert_eq!(neg_range(Some(11), None   , 10), None);
        assert_eq!(neg_range(Some( 3), Some(5), 10), Some((Bound::Included( 3), Bound::Excluded(5))));
    }
}
