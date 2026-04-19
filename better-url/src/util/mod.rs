//! General utility functions.

use crate::prelude::*;

mod idna;
mod scheme;
mod host;
mod ipv4;
mod path;
mod query;
mod fragment;
mod transcode;
mod macros;
mod ext_traits;

pub use idna::*;
pub use scheme::*;
pub use host::*;
pub use ipv4::*;
pub use path::*;
pub use query::*;
pub use fragment::*;
pub(crate) use transcode::*;
pub(crate) use macros::*;
pub(crate) use ext_traits::*;

/// Convert a `Cow<'_, str>` into a `Cow<'_, [u8]>`.
pub fn cow_str_to_bytes(value: Cow<'_, str>) -> Cow<'_, [u8]> {
    match value {
        Cow::Owned   (x) => Cow::Owned   (x.into    ()),
        Cow::Borrowed(x) => Cow::Borrowed(x.as_bytes())
    }
}

/// Convert a `Cow<'_, [u8]>` into a `Cow<'_, str>`.
/// # Safety
/// `value` must be valid UTF-8.
pub unsafe fn cow_bytes_to_str(value: Cow<'_, [u8]>) -> Cow<'_, str> {
    match value {
        Cow::Borrowed(x) => unsafe {str   ::from_utf8_unchecked(x)}.into(),
        Cow::Owned   (x) => unsafe {String::from_utf8_unchecked(x)}.into()
    }
}

/// Lossily decode a `Cow<'_, [u8]>`, ideally in-place.
pub fn decode_utf8_cow_lossy(value: Cow<'_, [u8]>) -> Cow<'_, str> {
    match String::from_utf8_lossy(&value) {
        // SAFETY: Can only be borrowed if `value` is UTF-8.
        Cow::Borrowed(_) => unsafe {cow_bytes_to_str(value)},
        Cow::Owned(x) => Cow::Owned(x)
    }
}
