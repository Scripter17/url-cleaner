//! Port stuff.

use crate::prelude::*;

/// Make a [`Port`].
/// # Errors
/// If `value` is not a valid port, returns the error [`InvalidPort`].
pub fn make_port<'a, T: Into<Cow<'a, str>>>(value: T) -> Result<(bool, u16, Cow<'a, str>), InvalidPort> {
    let value = value.into();

    let num = value.parse().map_err(|_| InvalidPort)?;

    Ok(match (num, value.as_bytes()) {
        (0  , [b'0', _, ..]) => (true , 0  , "0".into()            ),
        (1.., [b'0'   , ..]) => (true , num, num.to_string().into()),
        _                    => (false, num, value                 ),
    })
}

/// Make a [`MaybePort`].
/// # Errors
/// If [`make_port`] returns an error, that error is returned.
#[expect(clippy::type_complexity, reason = "It's fine.")]
pub fn make_maybe_port<'a, T: Into<Cow<'a, str>>>(value: Option<T>) -> Result<(bool, Option<(u16, Cow<'a, str>)>), InvalidPort> {
    Ok(match value.map(make_port).transpose()? {
        Some((changed, num, value)) => (changed, Some((num, value))),
        None                        => (false  , None              ),
    })
}

/// Make a [`Port`] from bytes.
/// # Errors
/// If `value` is not a valid port, returns the error [`InvalidPort`].
pub fn make_port_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> Result<(bool, u16, Cow<'a, str>), InvalidPort> {
    make_port(try_cow_bytes_to_str(value).map_err(|_| InvalidPort)?)
}

/// Make a [`MaybePort`] from bytes.
/// # Errors
/// If [`make_port_bytes`] returns an error, that error is returned.
#[expect(clippy::type_complexity, reason = "It's fine.")]
pub fn make_maybe_port_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: Option<T>) -> Result<(bool, Option<(u16, Cow<'a, str>)>), InvalidPort> {
    Ok(match value.map(make_port_bytes).transpose()? {
        Some((changed, num, value)) => (changed, Some((num, value))),
        None                        => (false  , None              ),
    })
}
