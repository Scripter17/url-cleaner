//! Special queries.

use crate::prelude::*;

/// Encode a [`SpecialQuery`].
pub fn encode_special_query<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value.into()), SPECIAL_QUERY)
}

/// Encode a [`SpecialQuerySegment`].
pub fn encode_special_query_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    let (changed, value) = percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value.into()), SPECIAL_QUERY_SEGMENT);

    let vs = value.find('=').and_then(|x| NonZero::new(x + 1));

    (changed, value, vs)
}

/// Turn a [`NonSpecialQuery`] into a [`SpecialQuery`].
pub fn non_special_query_to_special_query<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value.into()), NON_SPECIAL_QUERY_TO_SPECIAL_QUERY)
}

/// Turn a [`Fragment`]/[`FragmentQuery`] into a [`SpecialQuery`].
pub fn fragment_to_special_query<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value.into()), FRAGMENT_TO_SPECIAL_QUERY)
}
