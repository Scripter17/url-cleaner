//! Port stuff.

use crate::prelude::*;

/// Make a [`Port`].
/// # Errors
/// If `value` is not a valid port, returns the error [`InvalidPort`].
pub fn make_port<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, u16, Cow<'a, str>), InvalidPort> {
    let value = value.into();

    let num = value.parse().map_err(|_| InvalidPort)?;

    match (num, value.len()) {
        (0..10, 1) | (10..100, 2) | (100..1_000, 3) | (1_000..10_000, 4) | (10_000.., 5) => Ok((false, num, value)),
        _ => Ok((true, num, num.to_string().into()))
    }
}

/// Make a [`MaybePort`].
/// # Errors
/// If the call to [`make_port`] returns an error, that error is returned.
pub fn make_maybe_port<'a, T: Into<Cow<'a, str>>>(value: Option<T>) -> Result<Option<(bool, u16, Cow<'a, str>)>, InvalidPort> {
    value.map(make_port).transpose()
}
