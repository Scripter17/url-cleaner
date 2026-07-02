//! Non-special queries.

use crate::prelude::*;

/// Encode a [`NonSpecialQuery`].
pub fn encode_non_special_query<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value.into()), NON_SPECIAL_QUERY)
}

/// Encode a [`NonSpecialQuerySegment`].
pub fn encode_non_special_query_segment<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>, Option<NonZero<usize>>) {
    let (changed, value) = percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value.into()), NON_SPECIAL_QUERY_SEGMENT);

    let vs = value.find('=').and_then(|x| NonZero::new(x + 1));

    (changed, value, vs)
}

/// Turn a [`Fragment`]/[`FragmentQuery`] into a [`NonSpecialQuery`].
pub fn fragment_to_non_special_query<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value.into()), FRAGMENT_TO_NON_SPECIAL_QUERY)
}
