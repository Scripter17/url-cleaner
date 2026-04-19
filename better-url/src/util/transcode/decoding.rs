//! Decoding.

use crate::prelude::*;

impl PartTranscoder {
    /// Lossily decode a `Cow<'_, str>`, ideally in place.
    pub fn decode_lossy(self, value: Cow<'_, str>) -> Cow<'_, str> {
        decode_utf8_cow_lossy(self.decode(value))
    }

    /// [`Self::decode_bytes`].
    pub fn decode(self, value: Cow<'_, str>) -> Cow<'_, [u8]> {
        self.decode_bytes(cow_str_to_bytes(value))
    }

    /// Percent decode a `Cow<'_, [u8]>`, ideally in-place.
    #[allow(clippy::indexing_slicing, reason = "Can't happen.")]
    pub fn decode_bytes(self, mut value: Cow<'_, [u8]>) -> Cow<'_, [u8]> {
        for i in (0..value.len()).rev() {
            match value[i..] {
                [b'+', ..] if self == Self::QueryPart => value.to_mut()[i] = b' ',
                [b'%', h, l, ..] => if let Some(x) = decode_hex_byte(h, l) {
                    value.to_mut()[i] = x;
                    value.to_mut().remove(i + 1);
                    value.to_mut().remove(i + 1);
                },
                _ => {}
            }
        }

        value
    }
}
