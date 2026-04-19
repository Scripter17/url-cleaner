//! [`PartTranscoder`].

mod encoding;
mod decoding;

/// General part encoder/decoder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartTranscoder {
    /// Username/password.
    UserinfoPart,
    /// Opaque host.
    OpaqueHost,
    /// Special path.
    SpecialPath,
    /// Non-special Path.
    NonSpecialPath,
    /// [`Self::SpecialPath`] plus `/`, `\\`, and `%`.
    SpecialPathSegment,
    /// [`Self::NonSpecialPath`] plus `\` and `%`.
    NonSpecialPathSegment,
    /// [`Self::NonSpecialQuery`] plus `'`.
    SpecialQuery,
    /// Non special query.
    NonSpecialQuery,
    /// Application/x-www-form-urlencoded.
    QueryPart,
    /// Fragment.
    Fragment,
}

/// A set of ASCII bytes to encode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct AsciiSet(pub(crate) u128);

impl AsciiSet {
    /// If the set contains `b`.
    pub(crate) const fn contains(self, b: u8) -> bool {
        match b {
            128.. => true,
            x => self.0 & 1 << x != 0
        }
    }

    /// Add `b` to the set.
    /// # Panics
    /// if `b` is npn-ASCII, panics.
    pub(crate) const fn add(self, b: u8) -> Self {
        assert!(b.is_ascii());

        Self(self.0 | (1 << b))
    }

    /// [`Self::add`] each byte.
    pub(crate) const fn add_many(self, bs: &[u8]) -> Self {
        match bs.split_first() {
            Some((&b, bs)) => self.add(b).add_many(bs),
            None => self
        }
    }
}

/// The C0 control set.
pub(crate) const C0: AsciiSet = AsciiSet(0)
    .add_many(&[
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
    ]);

/// The component set.
pub(crate) const COMPONENT               : AsciiSet = USERINFO_PART    .add_many(b"$&+,");

/// The userinfo set.
pub(crate) const USERINFO_PART           : AsciiSet = PATH             .add_many(b"/:;=@[\\]|");
/// The opaque host set.
pub(crate) const OPAQUE_HOST             : AsciiSet = C0;
/// The path set.
pub(crate) const PATH                    : AsciiSet = NON_SPECIAL_QUERY.add_many(b"?^`{}");
/// The path segment set.
pub(crate) const SPECIAL_PATH_SEGMENT    : AsciiSet = PATH             .add_many(b"/%");
/// The path segment set.
pub(crate) const NON_SPECIAL_PATH_SEGMENT: AsciiSet = SPECIAL_PATH_SEGMENT.add_many(b"\\");
/// The non-special query set.
pub(crate) const NON_SPECIAL_QUERY       : AsciiSet = C0               .add_many(b" \"#<>");
/// The special query set.
pub(crate) const SPECIAL_QUERY           : AsciiSet = NON_SPECIAL_QUERY.add_many(b"'");
/// The query part set.
pub(crate) const QUERY_PART              : AsciiSet = COMPONENT        .add_many(b"!'()~");
/// The fragment set.
pub(crate) const FRAGMENT                : AsciiSet = C0               .add_many(b" \"<>`");



/// Nibbles.
pub(crate) const NIBBLES: &[u8; 16] = b"0123456789ABCDEF";



/// Decode a pair of ASCII hex nibbles.
pub(crate) fn decode_hex_byte(h: u8, l: u8) -> Option<u8> {
    Some(decode_hex_nibble(h)? * 16 + decode_hex_nibble(l)?)
}

/// Decode an ASCII hex nibble.
pub(crate) fn decode_hex_nibble(x: u8) -> Option<u8> {
    match x {
        b'0'..=b'9' => Some(x - b'0'),
        b'a'..=b'f' => Some(x - b'a' + 10),
        b'A'..=b'F' => Some(x - b'A' + 10),
        _ => None
    }
}
