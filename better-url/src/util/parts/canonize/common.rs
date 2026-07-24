//! Common stuff.

use crate::prelude::*;

/// General canonifier for most part setters.
///
/// You should use dedicated canonizers where applicable.
pub fn canonize_part_setter<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if value.memchr3(b'\t', b'\n', b'\r').is_some() {
        unsafe {
            value.to_mut().as_mut_vec().retain(|&b| b != b'\t' && b != b'\n' && b != b'\r');
        }
        changed = true;
    }

    (changed, value)
}
