//! [`Fragment`] and co.

use crate::prelude::*;

/// Canonize the input for the pathname setter to a form parsable by the various [`MaybeFragment`] types.
pub fn canonize_fragment_setter<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Option<Cow<'a, str>>) {
    let mut value = value.into();
    let mut changed = false;

    if value.is_empty() {
        return (false, None);
    }

    if value.starts_with('#') {
        unsafe {
            value.retain_range_unchecked(1..);
        }
        changed = true;
    }

    let (a, value) = canonize_part_setter(value);

    changed |= a;

    (changed, Some(value))
}
