//! [`Password`].

use crate::prelude::*;

/// Canonize the input for the password setter to a form parsable by [`Password`]
///
/// Technically a no-op, but useful for signaling intent.
pub fn canonize_set_password<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    (false, value.into())
}
