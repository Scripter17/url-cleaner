//! [`Username`].

use crate::prelude::*;

/// Canonize the input for the username setter to a form parsable by [`Username`]
///
/// Technically a no-op, but useful for signaling intent.
pub fn canonize_set_username<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    (false, value.into())
}
