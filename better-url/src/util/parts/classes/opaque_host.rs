//! [`OPAQUE_HOST_BYTE_CLASS_DATA`].

use crate::prelude::*;

/// A classification of each byte whose bits, from lowest to highest, are:
///
/// 1. If it's in [`FORBIDDEN_HOST_INPUT`].
///
/// 2. If it's in [`OPAQUE_HOST`].
///
/// 3. Unused.
///
/// 4. Unused.
///
/// 5. Unused.
///
/// 6. Unused.
///
/// 7. Unused.
///
/// 8. Unused.
pub const OPAQUE_HOST_BYTE_CLASS_DATA: [u8; 256] = get_opaque_host_byte_class_data();

/// Get [`OPAQUE_HOST_BYTE_CLASS_DATA`].
const fn get_opaque_host_byte_class_data() -> [u8; 256] {
    let mut ret = [0; 256];

    let mut i = 0;

    while i < 256 {
        if FORBIDDEN_HOST_INPUT.contains(i as u8) {ret[i] |= 0b01;}
        if OPAQUE_HOST         .contains(i as u8) {ret[i] |= 0b10;}

        i += 1;
    }

    ret
}
