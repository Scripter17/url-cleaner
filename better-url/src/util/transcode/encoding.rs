//! Encoding.

use crate::prelude::*;

impl PartTranscoder {
    /// Returns [`true`] if `b` is in `self`'s set.
    pub(crate) fn set(self) -> AsciiSet {
        match self {
            Self::UserinfoPart          => USERINFO_PART           ,
            Self::OpaqueHost            => OPAQUE_HOST             ,
            Self::SpecialPath           => PATH                    ,
            Self::NonSpecialPath        => PATH                    ,
            Self::SpecialPathSegment    => SPECIAL_PATH_SEGMENT    ,
            Self::NonSpecialPathSegment => NON_SPECIAL_PATH_SEGMENT,
            Self::SpecialQuery          => SPECIAL_QUERY           ,
            Self::NonSpecialQuery       => NON_SPECIAL_QUERY       ,
            Self::Fragment              => FRAGMENT                ,
            Self::QueryPart             => QUERY_PART              ,
        }
    }

    /// [`Self::encode_bytes`].
    pub fn encode(self, value: Cow<'_, str>) -> Cow<'_, str> {
        self.encode_bytes(cow_str_to_bytes(value))
    }

    /// Encode a `Cow<'_, [u8]>` into a `Cow<'_, str>`, ideally in-place.
    #[allow(clippy::indexing_slicing, reason = "Can't happen.")]
    pub fn encode_bytes(self, mut value: Cow<'_, [u8]>) -> Cow<'_, str> {
        let set = self.set();

        for i in (0..value.len()).rev() {
            match value[i] {
                b'\\' if self == Self::SpecialPath => value.to_mut()[i] = b'/',
                b' '  if self == Self::QueryPart   => value.to_mut()[i] = b'+',
                b =>  if set.contains(b) {
                    value.to_mut()[i] = b'%';
                    value.to_mut().insert(i + 1, NIBBLES[b as usize >> 4]);
                    value.to_mut().insert(i + 2, NIBBLES[b as usize & 15]);
                }
            }
        }

        // SAFETY: `value` contains only ASCII bytes.
        unsafe {cow_bytes_to_str(value)}
    }
}
