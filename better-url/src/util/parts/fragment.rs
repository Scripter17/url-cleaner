//! Fragment stuff.

use crate::prelude::*;

/// Encode a [`Fragment`]/[`FragmentQuery`].
pub fn encode_fragment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    encode_fragment_bytes(cow_str_to_bytes(value))
}

/// Encode a [`FragmentQuerySegment`].
pub fn encode_fragment_query_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    encode_fragment_query_segment_bytes(cow_str_to_bytes(value))
}



/// Encode a [`Fragment`]/[`FragmentQuery`] from bytes.
pub fn encode_fragment_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode_bytes(value, FRAGMENT)
}

/// Encode a [`FragmentQuerySegment`] from bytes.
pub fn encode_fragment_query_segment_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
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
