//! Fragment stuff.

use crate::prelude::*;



/// Make a [`MaybeFragment`].
pub fn make_maybe_fragment<'a, T: Into<Cow<'a, str>>>(value: Option<T>) -> (bool, Option<Cow<'a, str>>) {
    match value {
        None => (false, None),
        Some(value) => {
            let (changed, value) = make_fragment(value);

            (changed, Some(value))
        }
    }
}

/// Make a [`MaybeFragment`] from bytes.
pub fn make_maybe_fragment_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: Option<T>) -> (bool, Option<Cow<'a, str>>) {
    match value {
        None => (false, None),
        Some(value) => {
            let (changed, value) = make_fragment_bytes(value);

            (changed, Some(value))
        }
    }
}



/// Make a [`Fragment`]/[`FragmentQuery`].
pub fn make_fragment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    make_fragment_bytes(cow_str_to_bytes(value))
}

/// Make a [`FragmentQuerySegment`].
pub fn make_fragment_query_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    make_fragment_query_segment_bytes(cow_str_to_bytes(value))
}



/// Make a [`Fragment`]/[`FragmentQuery`] from bytes.
pub fn make_fragment_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode_bytes(value, FRAGMENT)
}

/// Make a [`FragmentQuerySegment`] from bytes.
pub fn make_fragment_query_segment_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    let (changed, value) = percent_encode_bytes(value, FRAGMENT_QUERY_SEGMENT);

    let vs = value.memchr(b'=').and_then(|x| NonZero::new(x + 1));

    (changed, value, vs)
}



/// Turn a [`SpecialQuery`] into a [`Fragment`]/[`FragmentQuery`].
pub fn special_query_to_fragment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    unsafe {
        percent_encode_one(value, b'`')
    }
}

/// Turn a [`NonSpecialQuery`] into a [`Fragment`]/[`FragmentQuery`].
pub fn non_special_query_to_fragment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    unsafe {
        percent_encode_one(value, b'`')
    }
}
