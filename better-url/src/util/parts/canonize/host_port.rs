//! [`Host`] and co. and [`Port`].

use crate::prelude::*;

/// Canonize the input for the host setter to a form parsable by the various [`Host`] types and [`Port`].
///
/// Notably, the returned port setter input being in two [`Option`]s is technically redundant.
///
/// The outer [`Option`] is [`Some`] if the port should be set to its contents.
///
/// The doubling is to prevent accidentally setting the port to [`None`] when it shouldn't be.
/// # Errors
/// If the host would be empty, returns the error [`CantBeEmpty`].
pub fn canonize_host_setter<'a, T: Into<Cow<'a, str>>>(value: T, scheme_type: SchemeType) -> Result<(Cow<'a, str>, Option<Option<u16>>), CantBeEmpty> {
    let (_, mut value) = canonize_part_setter(value);

    if value.starts_with(':') {
        Err(CantBeEmpty)?;
    }

    if scheme_type.is_file() {
        return Ok((value, None));
    }

    let mut in_brackets = false;

    for (i, b) in value.bytes().enumerate() {
        match b {
            b'[' => in_brackets = true,
            b']' => in_brackets = false,

            b':' if !in_brackets => {
                let (_, p) = canonize_port_setter(&value[i+1..]);

                let p = p.and_then(|x| x.parse().ok()).map(Some);
                value.retain_range(..i);

                return Ok((value, p));
            },

            b'/' | b'?' | b'#'                             => {value.retain_range(..i); return Ok((value, None));}
            b'\\'              if scheme_type.is_special() => {value.retain_range(..i); return Ok((value, None));}

            _ => {}
        }
    }

    Ok((value, None))
}
