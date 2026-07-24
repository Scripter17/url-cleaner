//! The whole URL.

use crate::prelude::*;

/// Canonize the input to the URL parser.
///
/// Done automatically by [`BetterUrl::new`].
pub fn canonize_parser_input<'a, T: Into<Cow<'a, str>>>(value: T) -> (bool, Cow<'a, str>) {
    let mut value = value.into();
    let mut changed = false;

    match value.as_bytes() {
        &[a, .., b] if a > 0x20 && b > 0x20 => {},
        _ => {
            let start = value.bytes(). position(|b| b > 0x20).unwrap_or(0);
            let after = value.bytes().rposition(|b| b > 0x20).map_or(0, |x| x + 1);

            if after - start != value.len() {
                unsafe {
                    value.retain_range_unchecked(start..after);
                }
                changed = true;
            }
        }
    }

    let (a, value) = canonize_part_setter(value);

    changed |= a;

    (changed, value)
}
