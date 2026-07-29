//! Special queries.

use crate::prelude::*;

mod iter;
pub use iter::*;

/// Encode a [`SpecialQuery`].
pub fn encode_special_query<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(value, SPECIAL_QUERY)
}

/// Encode a [`SpecialQuerySegment`].
pub fn encode_special_query_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    let (changed, value) = percent_encode(value, SPECIAL_QUERY_SEGMENT);

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
