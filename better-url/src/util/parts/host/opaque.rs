//! Opaque hosts.

use crate::prelude::*;

/// Encode an opaque host.
pub fn encode_opaque_host<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value.into()), OPAQUE_HOST)
}
