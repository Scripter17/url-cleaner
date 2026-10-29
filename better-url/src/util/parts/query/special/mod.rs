//! Special queries.

use crate::prelude::*;

mod iter;

pub use iter::*;



/// Make a [`MaybeSpecialQuery`].
pub fn make_maybe_special_query<'a, T: Into<Cow<'a, str>>>(value: Option<T>) -> (bool, Option<Cow<'a, str>>) {
    match value {
        None => (false, None),
        Some(value) => {
            let (changed, value) = make_special_query(value);

            (changed, Some(value))
        }
    }
}

/// Make a [`MaybeSpecialQuery`] from bytes.
pub fn make_maybe_special_query_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: Option<T>) -> (bool, Option<Cow<'a, str>>) {
    match value {
        None => (false, None),
        Some(value) => {
            let (changed, value) = make_special_query_bytes(value);

            (changed, Some(value))
        }
    }
}



/// Make a [`SpecialQuery`].
pub fn make_special_query<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(value, &SPECIAL_QUERY)
}

/// Make a [`SpecialQuerySegment`].
pub fn make_special_query_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    let (changed, value) = percent_encode(value, &SPECIAL_QUERY_SEGMENT);

    let vs = value.memchr(b'=').and_then(|x| NonZero::new(x + 1));

    (changed, value, vs)
}



/// Make a [`SpecialQuery`].
pub fn make_special_query_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode_bytes(value, &SPECIAL_QUERY)
}

/// Make a [`SpecialQuerySegment`].
pub fn make_special_query_segment_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    let (changed, value) = percent_encode_bytes(value, &SPECIAL_QUERY_SEGMENT);

    let vs = value.memchr(b'=').and_then(|x| NonZero::new(x + 1));

    (changed, value, vs)
}



/// Turn a [`NonSpecialQuery`] into a [`SpecialQuery`].
pub fn non_special_query_to_special_query<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    unsafe {
        percent_encode_one(value, b'`')
    }
}

/// Turn a [`Fragment`]/[`FragmentQuery`] into a [`SpecialQuery`].
pub fn fragment_to_special_query<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    unsafe {
        percent_encode_one(value, b'#')
    }
}
