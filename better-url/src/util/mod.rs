//! General utility functions.

use crate::prelude::*;

mod parts;
mod percent;
mod ascii_set;
mod macros;
mod ext_traits;

pub use parts::*;
pub use percent::*;
pub use ascii_set::*;

pub(crate) use macros::*;
pub(crate) use ext_traits::*;

/// A UTS46 normalizer.
pub(crate) static UTS46: icu_normalizer::uts46::Uts46MapperBorrowed = icu_normalizer::uts46::Uts46MapperBorrowed::new();

#[derive(Debug)]
pub(crate) enum Thing1 {
    Prepend,
    Append,
    New,
    Set(usize),
}

impl Thing1 {
    pub(crate) fn set_urange(index: isize, len: usize) -> Option<Range<usize>> {
        Some(match thing1(index, len)? {
            Self::Prepend | Self::New =>   0..  0,
            Self::Append              => len..len,
            Self::Set(i)              =>   i..i+1,
        })
    }
}

pub(crate) fn thing1(index: isize, len: usize) -> Option<Thing1> {
    match index {
        0 if len == 0 => Some(Thing1::New),

        0.. if index as usize >  len => None,
        0.. if index as usize == len => Some(Thing1::Append),
        0..                          => Some(Thing1::Set(index as usize)),

        ..0 if index.unsigned_abs() - 1 >  len => None,
        ..0 if index.unsigned_abs() - 1 == len => Some(Thing1::Prepend),
        ..0                                    => Some(Thing1::Set(len - index.unsigned_abs())),
    }
}

#[derive(Debug)]
pub(crate) enum Thing2 {
    Prepend,
    Append,
    New,
    Insert(usize),
}

impl Thing2 {
    pub(crate) fn insert_urange(index: isize, len: usize) -> Option<Range<usize>> {
        Some(match thing2(index, len)? {
            Self::Prepend | Self::New =>   0..  0,
            Self::Append              => len..len,
            Self::Insert(i)           =>   i..  i,
        })
    }
}

pub(crate) fn thing2(index: isize, len: usize) -> Option<Thing2> {
    match index {
        0 if len == 0 => Some(Thing2::New),

        0.. if index as usize >  len => None,
        0.. if index as usize == len => Some(Thing2::Append),
        0..                          => Some(Thing2::Insert(index as usize)),

        ..0 if index.unsigned_abs() - 1 >  len => None,
        ..0 if index.unsigned_abs() - 1 == len => Some(Thing2::Prepend),
        ..0                                    => Some(Thing2::Insert(len - index.unsigned_abs() + 1)),
    }
}

pub(crate) fn range_intersection<B: RangeBounds<isize>>(range: B, len: usize) -> Option<Range<usize>> {
    let start = match range.start_bound() {
        Bound::Unbounded => 0,

        Bound::Included(&x @ 0..) if x as usize > len => None?,
        Bound::Included(&x @ 0..) => x as usize,
        Bound::Included(&x @ ..0) => len.saturating_add_signed(x),

        Bound::Excluded(&x @ 0..) if x as usize >= len => None?,
        Bound::Excluded(&x @ 0..) => x as usize - 1,
        Bound::Excluded(&x @ ..0) => len.saturating_add_signed(x + 1),
    };

    let after = match range.end_bound() {
        Bound::Unbounded => len,

        Bound::Included(&x @ 0..) => (x as usize + 1).min(len),
        Bound::Included(&x @ ..0) if x.unsigned_abs() > len => None?,
        Bound::Included(&x @ ..0) => len.saturating_sub_signed(x + 1),

        Bound::Excluded(&x @ 0..) => (x as usize).min(len),
        Bound::Excluded(&x @ ..0) if x.unsigned_abs() >= len => None?,
        Bound::Excluded(&x @ ..0) => len.saturating_add_signed(x)
    };

    Some(start..after)
}

pub(crate) fn urange_intersection<B: RangeBounds<usize>>(range: B, len: usize) -> Option<Range<usize>> {
    let start = match range.start_bound() {
        Bound::Unbounded    => 0,

        Bound::Excluded(&x) if x >= len => None?,
        Bound::Excluded(&x) => x + 1,

        Bound::Included(&x) if x >  len => None?,
        Bound::Included(&x) => x
    };

    let after = match range.end_bound() {
        Bound::Unbounded    => len,
        Bound::Excluded(&x) => len.min(x),
        Bound::Included(&x) => len.min(x + 1),
    };

    Some(start..after)
}

/// Normalize an [`isize`] index into a [`usize`] index.
pub(crate) fn normalize_index(index: isize, len: usize) -> Option<usize> {
    Some(match index {
        0.. if index as usize > len => None?,
        0.. => index as usize,
        ..0 => len.checked_add_signed(index)?
    })
}

/// Normalize a [`RangeBounds`] of [`isize`] into a [`Range`] of [`usize`].
pub(crate) fn normalize_irange<B: RangeBounds<isize>>(range: B, len: usize) -> Option<Range<usize>> {
    let start = match range.start_bound() {
        Bound::Unbounded => 0,

        Bound::Excluded(&x @ 0..) if x as usize >= len => None?,
        Bound::Included(&x @ 0..) if x as usize >  len => None?,

        Bound::Excluded(&x @ 0..) => x as usize + 1,
        Bound::Included(&x @ 0..) => x as usize,

        Bound::Excluded(&x @ ..0) => len.checked_sub(x.unsigned_abs() + 1)?,
        Bound::Included(&x @ ..0) => len.checked_sub(x.unsigned_abs()    )?,
    };

    let after = match range.end_bound() {
        Bound::Unbounded => len,

        Bound::Excluded(&x @ 0..) if x as usize >  len => None?,
        Bound::Included(&x @ 0..) if x as usize >= len => None?,

        Bound::Excluded(&x @ 0..) => x as usize,
        Bound::Included(&x @ 0..) => x as usize + 1,

        Bound::Excluded(&x @ ..0) => len.checked_sub(x.unsigned_abs()    )?,
        Bound::Included(&x @ ..0) => len.checked_sub(x.unsigned_abs() - 1)?,
    };

    Some(start..after)
}

/// Normalize a [`RangeBounds`] of [`usize`] into a [`Range`] of [`usize`].
pub(crate) fn normalize_urange<B: RangeBounds<usize>>(range: B, len: usize) -> Option<Range<usize>> {
    let start = match range.start_bound() {
        Bound::Unbounded    => 0,

        Bound::Excluded(&x) if x >= len => None?,
        Bound::Included(&x) if x >  len => None?,

        Bound::Excluded(&x) => x + 1,
        Bound::Included(&x) => x,
    };

    let after = match range.end_bound() {
        Bound::Unbounded    => len,

        Bound::Excluded(&x) if x >  len => None?,
        Bound::Included(&x) if x >= len => None?,

        Bound::Excluded(&x) => x,
        Bound::Included(&x) => x + 1,
    };

    Some(start..after)
}

/// Get the `range` of `split`-delimited segments of `whole`.
pub(crate) fn segments_range_thing<B: RangeBounds<isize>>(whole: &str, split: char, range: B) -> Option<&str> {
    let start = match range.start_bound() {
        Bound::Unbounded    => whole.split(split).neg_nth(0)?,
        Bound::Excluded(-1) => None?,
        Bound::Excluded(&x) => whole.split(split).neg_nth(x + 1)?,
        Bound::Included(&x) => whole.split(split).neg_nth(x)?,
    }.addr() - whole.addr();

    let after = match range.end_bound() {
        Bound::Unbounded    => whole.split(split).neg_nth(-1)?,
        Bound::Excluded(&0) => None?,
        Bound::Excluded(&x) => whole.split(split).neg_nth(x - 1)?,
        Bound::Included(&x) => whole.split(split).neg_nth(x)?,
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
