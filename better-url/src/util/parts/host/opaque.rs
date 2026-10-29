//! Opaque hosts.

use crate::prelude::*;

/// Encode an opaque host.
/// # Errors
/// If `value` is not a valid opaque host, returns the error [`InvalidOpaqueHost`].
pub fn make_opaque_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, OpaqueHostDetails), InvalidOpaqueHost> {
    make_opaque_host_bytes(cow_str_to_bytes(value))
}

/// Encode an opaque host from bytes.
/// # Errors
/// If `value` is not a valid opaque host, returns the error [`InvalidOpaqueHost`].
pub fn make_opaque_host_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>, OpaqueHostDetails), InvalidOpaqueHost> {
    let value = value.into();

    if value.is_empty() {
        Err(InvalidOpaqueHost)?;
    }

    let class = value.iter().fold(0, |acc, &b| acc | OPAQUE_HOST_BYTE_CLASS_DATA[b as usize]);

    if class & 1 == 1 {
        Err(InvalidOpaqueHost)?;
    }

    if class & 2 == 0 {
        return Ok((false, unsafe {cow_bytes_to_str_unchecked(value)}, OpaqueHostDetails));
    }

    let (changed, value) = percent_encode_bytes(value, &OPAQUE_HOST);

    Ok((changed, value, OpaqueHostDetails))
}
