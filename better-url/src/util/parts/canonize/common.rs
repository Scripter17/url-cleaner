//! Common stuff.

use crate::prelude::*;

/// Bytes to keep.
static KEEPS: ByteSet = ByteSet::new().add_many(b"\t\n\r").invert();

/// General canonifier for most part setters.
///
/// You should use dedicated canonizers where applicable.
pub fn canonize_part_setter<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if value.memchrn(*b"\t\n\r").is_some() {
        unsafe {
            value.to_mut().as_mut_vec().retain(|&b| KEEPS.contains(b));
        }
        changed = true;
    }

    (changed, value)
}
