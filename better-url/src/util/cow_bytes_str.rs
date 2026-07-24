//! Conversions between [`Cow`]s of bytes and strings.

use crate::prelude::*;

/// Convert a `Cow<'_, str>` into a `Cow<'_, [u8]>`.
pub fn cow_str_to_bytes<'a, T: Into<Cow<'a, str>>>(value: T) -> Cow<'a, [u8]> {
    match value.into() {
        Cow::Owned   (x) => Cow::Owned   (x.into    ()),
        Cow::Borrowed(x) => Cow::Borrowed(x.as_bytes()),
    }
}

/// Try to convert a `Cow<'_, [u8]>` into a `Cow<'_, str>`.
/// # Errors
/// If the call to [`str::from_utf8`] returns an error, that error and the `value` are returned.
pub fn try_cow_bytes_to_str<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<Cow<'a, str>, (std::str::Utf8Error, Cow<'a, [u8]>)> {
    let value = value.into();

    match str::from_utf8(&value) {
        Ok (_) => Ok(unsafe {cow_bytes_to_str_unchecked(value)}),
        Err(e) => Err((e, value)),
    }
}

/// Convert a `Cow<'_, [u8]>` into a `Cow<'_, str>` without checking for validity.
/// # Safety
/// `value` must be valid UTF-8.
pub unsafe fn cow_bytes_to_str_unchecked<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Cow<'a, str> {
    match value.into() {
        Cow::Borrowed(x) => unsafe {str   ::from_utf8_unchecked(x)}.into(),
        Cow::Owned   (x) => unsafe {String::from_utf8_unchecked(x)}.into(),
    }
}

/// Lossily a `Cow<'a, [u8]>` into a `Cow<'a, str>`.
pub fn lossy_cow_bytes_to_str<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Cow<'a, str> {
    let value = value.into();

    match String::from_utf8_lossy(&value) {
        Cow::Borrowed(_) => unsafe {cow_bytes_to_str_unchecked(value)},
        Cow::Owned   (x) => Cow::Owned(x),
    }
}
