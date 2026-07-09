//! General utility functions.

use crate::prelude::*;

mod split;
mod parts;
mod percent;
mod ascii_set;
mod normalizer;
mod macros;
mod ext_traits;

pub use split::*;
pub use parts::*;
pub use percent::*;
pub use ascii_set::*;
pub use normalizer::*;

pub(crate) use macros::*;
pub(crate) use ext_traits::*;

/// A UTS46 normalizer.
pub(crate) static UTS46: icu_normalizer::uts46::Uts46MapperBorrowed = icu_normalizer::uts46::Uts46MapperBorrowed::new();

/// Get the `range` of domain segments.
pub(crate) fn domain_range_thing<B: RangeBounds<isize>>(whole: &str, range: B) -> Option<&str> {
    let mut split = SplitDots(Some(whole));

    let start = match range.start_bound() {
        Bound::Unbounded    => split.clone().neg_nth(0)?,
        Bound::Excluded(-1) => None?,
        Bound::Excluded(&x) => split.clone().neg_nth(x + 1)?,
        Bound::Included(&x) => split.clone().neg_nth(x)?,
    }.addr() - whole.addr();

    let after = match range.end_bound() {
        Bound::Unbounded    => split.neg_nth(-1)?,
        Bound::Excluded(&0) => None?,
        Bound::Excluded(&x) => split.neg_nth(x - 1)?,
        Bound::Included(&x) => split.neg_nth(x)?,
    }.end_addr() - whole.addr();

    whole.get(start .. after)
}

/// Get the `range` of path segments.
pub(crate) fn path_segments_range_thing<B: RangeBounds<isize>>(whole: &str, range: B) -> Option<&str> {
    let mut split = SplitSlashes(Some(whole));

    let start = match range.start_bound() {
        Bound::Unbounded    => split.clone().neg_nth(0)?,
        Bound::Excluded(-1) => None?,
        Bound::Excluded(&x) => split.clone().neg_nth(x + 1)?,
        Bound::Included(&x) => split.clone().neg_nth(x)?,
    }.addr() - whole.addr();

    let after = match range.end_bound() {
        Bound::Unbounded    => split.neg_nth(-1)?,
        Bound::Excluded(&0) => None?,
        Bound::Excluded(&x) => split.neg_nth(x - 1)?,
        Bound::Included(&x) => split.neg_nth(x)?,
    }.end_addr() - whole.addr();

    whole.get(start .. after)
}

/// Convert a `Cow<'_, str>` into a `Cow<'_, [u8]>`.
pub fn cow_str_to_bytes(value: Cow<'_, str>) -> Cow<'_, [u8]> {
    match value {
        Cow::Owned   (x) => Cow::Owned   (x.into    ()),
        Cow::Borrowed(x) => Cow::Borrowed(x.as_bytes())
    }
}

/// Try to convert a `Cow<'_, [u8]>` into a `Cow<'_, str>`.
/// # Errors
/// If the call to [`str::from_utf8`] returns an error, that error is returned.
pub fn try_cow_bytes_to_str(value: Cow<'_, [u8]>) -> Result<Cow<'_, str>, std::str::Utf8Error> {
    str::from_utf8(&value)?;
    Ok(unsafe {cow_bytes_to_str(value)})
}

/// Convert a `Cow<'_, [u8]>` into a `Cow<'_, str>` without checking for validity.
/// # Safety
/// `value` must be valid UTF-8.
pub unsafe fn cow_bytes_to_str(value: Cow<'_, [u8]>) -> Cow<'_, str> {
    match value {
        Cow::Borrowed(x) => unsafe {str   ::from_utf8_unchecked(x)}.into(),
        Cow::Owned   (x) => unsafe {String::from_utf8_unchecked(x)}.into()
    }
}

/// Lossily decode a `Cow<'_, [u8]>`, ideally in-place.
pub fn decode_utf8_cow_lossy(value: Cow<'_, [u8]>) -> Cow<'_, str> {
    match String::from_utf8_lossy(&value) {
        // SAFETY: Can only be borrowed if `value` is UTF-8.
        Cow::Borrowed(_) => unsafe {cow_bytes_to_str(value)},
        Cow::Owned(x) => Cow::Owned(x)
    }
}
