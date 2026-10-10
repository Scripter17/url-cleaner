//! The whole URL.

use crate::prelude::*;

/// Bytes to keep.
const KEEPS: ByteSet = ByteSet::new().add_many(b"\t\n\r").invert();

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

    if matches!(&*value, &[..=b'\x20', ..] | &[.., ..=b'\x20']) {
        #[cold]
        fn outline(value: &mut Cow<'_, [u8]>) {
            let start = value.iter(). position(|&b| b > 0x20).unwrap_or(0);
            let after = value.iter().rposition(|&b| b > 0x20).map_or(0, |x| x + 1);

            unsafe {
                value.retain_range_unchecked(start..after);
            }
        }

        outline(&mut value);
        changed = true;
    }

    if value.memhas(b'\t') || value.memhas(b'\r') || value.memhas(b'\n') {
        #[cold]
        fn outline(value: &mut Cow<'_, [u8]>) {
            value.to_mut().retain(|&b| KEEPS.contains(b));
        }

        outline(&mut value);
        changed = true;
    }

    (changed, value)
}
