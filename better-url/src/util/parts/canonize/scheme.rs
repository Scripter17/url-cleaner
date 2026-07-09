//! [`Scheme`].

use crate::prelude::*;

/// Canonize the input for the protocol setter to a form parsable by [`Scheme`].
pub fn canonize_scheme_setter<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if let Some(i) = value.bytes().position(|b| b == b':') {
        value.retain_range(..i);
        changed = true;
    }

    let (a, value) = canonize_part_setter(value);

    changed |= a;

    (changed, value)
}
