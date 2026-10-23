//! [`NonSpecialHost`].

use crate::prelude::*;

/// Make a [`NonSpecialHost`].
/// # Errors
/// If `value` is an invalid [`NonSpecialHost`], returns the error [`InvalidFileHost`].
pub fn make_non_special_host<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, Cow<'a, str>, NonSpecialHostDetails), InvalidNonSpecialHost> {
    let value = value.into();

    Ok(match value.as_bytes() {
        b""        => (false, value, EmptyHostDetails.into()),
        [b'[', ..] => {
            let (changed, value, details) = make_ipv6_host(value)?;

            (changed, value, details.into())
        },
        _ => {
            let (changed, value, details) = make_opaque_host(value)?;

            (changed, value, details.into())
        },
    })
}

/// Make a [`NonSpecialHost`] from bytes.
/// # Errors
/// If `value` is an invalid [`NonSpecialHost`], returns the error [`InvalidFileHost`].
pub fn make_non_special_host_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, Cow<'a, str>, NonSpecialHostDetails), InvalidNonSpecialHost> {
    let value = value.into();

    Ok(match &*value {
        b""        => (false, unsafe {cow_bytes_to_str_unchecked(value)}, EmptyHostDetails.into()),
        [b'[', ..] => {
            let (changed, value, details) = make_ipv6_host_bytes(value)?;

            (changed, value, details.into())
        },
        _ => {
            let (changed, value, details) = make_opaque_host_bytes(value)?;

            (changed, value, details.into())
        },
    })
}
