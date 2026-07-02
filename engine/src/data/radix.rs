//! [`Radix`].

use crate::prelude::*;

/// A known valid [`char::is_digit`] radix.
///
/// Specifically, a [`u8`] between 2 and 36, inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Suitability)]
pub struct Radix(NonZero<u8>);

impl Radix {
    /// Get the inner [`u8`].
    pub fn get(self) -> u8 {
        self.0.get()
    }

    /// If `b` is a valid digit for the specified radix.
    ///
    /// Equivalent to [`char::is_digit`] but should be faster.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let r10 = Radix::try_from(10).unwrap();
    ///
    /// assert!( r10.byte_is_digit(b'9'));
    /// assert!(!r10.byte_is_digit(b'a'));
    /// assert!(!r10.byte_is_digit(b'A'));
    ///
    /// let r36 = Radix::try_from(36).unwrap();
    ///
    /// assert!(r36.byte_is_digit(b'9'));
    /// assert!(r36.byte_is_digit(b'z'));
    /// assert!(r36.byte_is_digit(b'Z'));
    /// ```
    pub fn byte_is_digit(self, b: u8) -> bool {
        let x = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'z' => b - b'a' + 10,
            b'A'..=b'Z' => b - b'A' + 10,
            _ => return false
        };

        x < self.get()
    }

    /// [`char::is_digit`].
    pub fn char_is_digit(self, c: char) -> bool {
        c.is_digit(self.get().into())
    }
}

impl From<Radix> for u8 {
    fn from(value: Radix) -> Self {
        value.get()
    }
}

impl TryFrom<u8> for Radix {
    type Error = InvalidRadix;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match NonZero::new(value) {
            Some(x) if (2..=36).contains(&x.get()) => Ok (Self(x)),
            _                                      => Err(InvalidRadix(value))
        }
    }
}

impl Serialize for Radix {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(self.get())
    }
}

impl<'de> Deserialize<'de> for Radix {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(RadixVisitor)
    }
}

/// [`Visitor`] for [`Radix`].
#[derive(Debug)]
struct RadixVisitor;

impl<'de> Visitor<'de> for RadixVisitor {
    type Value = Radix;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "A u8 between 2 and 36, inclusive.")
    }

    fn visit_u8<E: de::Error>(self, v: u8) -> Result<Self::Value, E> {
        v.try_into().map_err(E::custom)
    }
}
