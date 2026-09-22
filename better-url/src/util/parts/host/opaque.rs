//! Opaque hosts.

use crate::prelude::*;

/// Encode an opaque host.
/// # Errors
/// If `value` is not a valid opaque host, returns the error [`InvalidOpaqueHost`].
pub fn encode_opaque_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidOpaqueHost> {
    encode_opaque_host_bytes(cow_str_to_bytes(value))
}

/// Encode an opaque host from bytes.
/// # Errors
/// If `value` is not a valid opaque host, returns the error [`InvalidOpaqueHost`].
pub fn encode_opaque_host_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>), InvalidOpaqueHost> {
    let value = value.into();

    if value.is_empty() {
        Err(InvalidOpaqueHost)?;
    }

    if value.iter().any(|&b| FORBIDDEN_HOST_INPUT.contains(b)) {
        Err(InvalidOpaqueHost)?;
    }

    Ok(percent_encode_bytes(value, OPAQUE_HOST))
}
