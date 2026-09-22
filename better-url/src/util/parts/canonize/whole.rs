//! The whole URL.

use crate::prelude::*;

/// Canonize the input to the URL parser.
///
/// Done automatically by [`BetterUrl::new`].
pub fn canonize_parser_input<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let (changed, value) = canonize_parser_input_bytes(cow_str_to_bytes(value));

    (changed, unsafe {cow_bytes_to_str_unchecked(value)})
}

/// Canonize the input to the URL parser.
///
/// Done automatically by [`BetterUrl::new`].
pub fn canonize_parser_input_bytes<'a, T: Into<Cow<'a, [u8]>>>(value: T) -> (bool, Cow<'a, [u8]>) {
    let mut value = value.into();
    let mut changed = false;

    match &*value {
        &[a, .., b] if a > 0x20 && b > 0x20 => {},
        _ => {
            let start = value.iter(). position(|&b| b > 0x20).unwrap_or(0);
            let after = value.iter().rposition(|&b| b > 0x20).map_or(0, |x| x + 1);

            unsafe {
                value.retain_range_unchecked(start..after);
            }
            changed = true;
        }
    }

    let (a, value) = canonize_part_setter_bytes(value);

    changed |= a;

    (changed, value)
}
