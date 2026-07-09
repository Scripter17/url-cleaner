//! Opaque hosts.

use crate::prelude::*;

/// Encode an opaque host.
/// # Errors
/// If `value` is not a valid opaque host, returns the error [`InvalidOpaqueHost`].
pub fn encode_opaque_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidOpaqueHost> {
    let value = value.into();

    if value.is_empty() {
        Err(InvalidOpaqueHost)?;
    }

    if value.bytes().any(|b| b.is_ascii() && FORBIDDEN_HOST.contains(b)) {
        Err(InvalidOpaqueHost)?;
    }

    Ok(percent_encode::<'_, _, false, false, false>(cow_str_to_bytes(value), OPAQUE_HOST))
}
