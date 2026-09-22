//! Setter stuff.

use crate::prelude::*;

/// Bytes to keep.
const KEEPS: ByteSet = ByteSet::new().add_many(b"\t\n\r").invert();

/// General canonifier for most part setters.
///
/// You should use dedicated canonizers where applicable.
pub fn canonize_part_setter<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (changed, value) = canonize_part_setter_bytes(cow_str_to_bytes(value));

    (changed, unsafe {cow_bytes_to_str_unchecked(value)})
}

/// General canonifier for most part setters.
///
/// You should use dedicated canonizers where applicable.
pub fn canonize_part_setter_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    let mut value = value.into();
    let mut changed = false;

    if value.memchr(b'\t').is_some() || value.memchr(b'\r').is_some() || value.memchr(b'\n').is_some() {
        value.to_mut().retain(|&b| KEEPS.contains(b));
        changed = true;
    }

    (changed, value)
}
