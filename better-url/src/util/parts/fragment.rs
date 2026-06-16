//! Fragment stuff.

use crate::prelude::*;

/// Encode a [`Fragment`]/[`FragmentQuery`].
pub fn encode_fragment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(cow_str_to_bytes(value.into()), false, false, false, FRAGMENT)
}

/// Encode a [`FragmentQuerySegment`].
pub fn encode_fragment_query_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    let (changed, value) = percent_encode(cow_str_to_bytes(value.into()), false, false, false, FRAGMENT_QUERY_SEGMENT);

    let vs = value.find('=').and_then(|x| NonZero::new(x + 1));

    (changed, value, vs)
}

/// Turn a [`SpecialQuery`] into a [`Fragment`]/[`FragmentQuery`].
pub fn special_query_to_fragment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(cow_str_to_bytes(value.into()), false, false, false, SPECIAL_QUERY_TO_FRAGMENT)
}

/// Turn a [`NonSpecialQuery`] into a [`Fragment`]/[`FragmentQuery`].
pub fn non_special_query_to_fragment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode(cow_str_to_bytes(value.into()), false, false, false, NON_SPECIAL_QUERY_TO_FRAGMENT)
}
