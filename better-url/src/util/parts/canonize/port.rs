//! [`Port`] and co.

use crate::prelude::*;

/// Canonize the input for the port setter to a form parsable by [`MaybePort`].
pub fn canonize_port_setter<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Option<Cow<'a, str>>) {
    let value = value.into();

    if value.is_empty() {
        return (false, None);
    }

    let (mut changed, mut value) = canonize_part_setter(value);

    if let Some(i) = value.bytes().position(|b| !b.is_ascii_digit()) {
        value.retain_range(..i);
        changed = true;
    }

    (changed, Some(value))
}
