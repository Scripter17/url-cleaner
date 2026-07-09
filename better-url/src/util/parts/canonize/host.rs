//! [`Host`] and co.

use crate::prelude::*;

/// Canonize the input for the hostname setter to a form parsable by the various [`Host`] types.
pub fn canonize_maybe_hostname_setter<'a, T: Into<Cow<'a, str>>>(value: Option<T>, special: bool) -> (bool, Option<Cow<'a, str>>) {
    match special {
        true  => canonize_maybe_special_hostname_setter    (value),
        false => canonize_maybe_non_special_hostname_setter(value),
    }
}

/// Canonize the input for the hostname setter to a form parsable by the various [`Host`] types.
pub fn canonize_hostname_setter<'a, T: Into<Cow<'a, str>>>(value: T, special: bool) -> (bool, Cow<'a, str>) {
    match special {
        true  => canonize_special_hostname_setter    (value),
        false => canonize_non_special_hostname_setter(value),
    }
}

/// Canonize the input for the hostname setter on non-special URLs to a form parsable by the various [`Host`] types.
pub fn canonize_maybe_non_special_hostname_setter<'a, T: Into<Cow<'a, str>>>(value: Option<T>) -> (bool, Option<Cow<'a, str>>) {
    match value {
        Some(value) => {
            let (changed, value) = canonize_non_special_hostname_setter(value);

            (changed, Some(value))
        },
        None => (false, None)
    }
}

/// Canonize the input for the hostname setter on special URLs to a form parsable by the various [`Host`] types.
pub fn canonize_maybe_special_hostname_setter<'a, T: Into<Cow<'a, str>>>(value: Option<T>) -> (bool, Option<Cow<'a, str>>) {
    match value {
        Some(value) => {
            let (changed, value) = canonize_special_hostname_setter(value);

            (changed, Some(value))
        },
        None => (false, None)
    }
}

/// Canonize the input for the hostname setter on non-special URLs to a form parsable by the various [`Host`] types.
pub fn canonize_non_special_hostname_setter<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if let Some(i) = value.bytes().position(|b| b == b'/' || b == b'?' || b == b'#') {
        value.retain_range(..i);
        changed = true;
    }

    let (a, value) = canonize_part_setter(value);

    changed |= a;

    (changed, value)
}

/// Canonize the input for the hostname setter on special URLs to a form parsable by the various [`Host`] types.
pub fn canonize_special_hostname_setter<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    if let Some(i) = value.bytes().position(|b| b == b'/' || b == b'\\' || b == b'?' || b == b'#') {
        value.retain_range(..i);
        changed = true;
    }

    let (a, value) = canonize_part_setter(value);

    changed |= a;

    (changed, value)
}

