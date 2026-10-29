//! [`BidiDetail`].

use icu_properties::props::{EnumeratedProperty, BidiClass};

use crate::prelude::*;

/// The result of the Bidi Rule for a set of domain segments.
///
/// Per [RFC 5893](https://www.rfc-editor.org/info/rfc5893):
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BidiDetail {
    /// Left to right and satisfies the Bidi Rule.
    Ltr,
    /// Left to right and violates the Bidi Rule.
    ForceLtr,
    /// Right to left and satisfies the Bidi Rule.
    Rtl,
    /// Invalid outside of Punycode encoding.
    ForceAscii,
}

/** Shorthand. */ const AL  : BidiClass = BidiClass::AL ;
/** Shorthand. */ const AN  : BidiClass = BidiClass::AN ;
/** Shorthand. */ const BN  : BidiClass = BidiClass::BN ;
/** Shorthand. */ const CS  : BidiClass = BidiClass::CS ;
/** Shorthand. */ const EN  : BidiClass = BidiClass::EN ;
/** Shorthand. */ const ES  : BidiClass = BidiClass::ES ;
/** Shorthand. */ const ET  : BidiClass = BidiClass::ET ;
/** Shorthand. */ const L   : BidiClass = BidiClass::L  ;
/** Shorthand. */ const NSM : BidiClass = BidiClass::NSM;
/** Shorthand. */ const ON  : BidiClass = BidiClass::ON ;
/** Shorthand. */ const R   : BidiClass = BidiClass::R  ;

impl BidiDetail {
    /// Join the [`BidiDetail`]s of multiple domain segments.
    pub fn join(self, other: Self) -> Self {
        match (self, other) {
            (Self::Ltr, other) | (other, Self::Ltr) => other,

            (Self::Rtl     , Self::Rtl     ) => Self::Rtl     ,
            (Self::ForceLtr, Self::ForceLtr) => Self::ForceLtr,

            (Self::ForceAscii, _             ) | (_             , Self::ForceAscii) => Self::ForceAscii,
            (Self::Rtl       , Self::ForceLtr) | (Self::ForceLtr, Self::Rtl       ) => Self::ForceAscii,
        }
    }

    /// Parse a decoded domain segment.
    ///
    /// First does [`Self::parse_ascii`], only doing [`Self::parse_unicode`] if it returns [`None`].
    pub fn parse(value: &str) -> Self {
        if let Some(ret) = Self::parse_ascii(value) {
            return ret;
        }

        Self::parse_unicode(value)
    }

    /// Quickly check if an ASCII label is [`Self::Ltr`] or [`Self::ForceLtr`].
    ///
    /// Done as the first step of [`Self::parse`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// for a in 0..=127 {
    ///     for b in 0..=127 {
    ///         for c in 0..=127 {
    ///             let bs = [a, b, c];
    ///             let s = str::from_utf8(&bs).unwrap();
    ///
    ///             if let Some(x) = BidiDetail::parse_ascii(s) {
    ///                 assert_eq!(x, BidiDetail::parse_unicode(s), "{s:?}");
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn parse_ascii(value: &str) -> Option<Self> {
        let (x, mid, y) = match *value.as_bytes() {
            [                  ] => return Some(Self::Ltr),
            [x                 ] => (x, b"".as_slice(), x),
            [x, ref mid @ .., y] => (x, mid           , y),
        };

        let c1 =                                    DOMAIN_BYTE_CLASS_DATA[x as usize] ;
        let c2 = mid.iter().fold(0, |acc, &b| acc | DOMAIN_BYTE_CLASS_DATA[b as usize]);
        let c3 =                                    DOMAIN_BYTE_CLASS_DATA[y as usize] ;

        if (c1 & 0b0010_0000) | (c2 & 0b0100_0000) | (c3 & 0b1000_0000) == 0 {
            Some(Self::Ltr)
        } else if (c1 | c2 | c3) & 0b0100_0000 == 0 {
            Some(Self::ForceLtr)
        } else {
            None
        }
    }

    /// [`Self::parse`] without first trying [`Self::parse_ascii`].
    ///
    /// Done as the second step of [`Self::parse`] after [`Self::parse_ascii`].
    pub fn parse_unicode(value: &str) -> Self {
        let mut classes = value.chars().map(BidiClass::for_char);

        // TODO: [u8; 256].

        match classes.next() {
            Some(R | AL) => {
                let mut check3 = true;
                let mut has_en = false;
                let mut has_an = false;

                for c in classes {
                    match c {
                        R | AL                 => check3 = true,
                        AN                     => {check3 = true; has_an = true;},
                        EN                     => {check3 = true; has_en = true;},
                        ES | CS | ET | ON | BN => check3 = false,
                        NSM                    => {}
                        _                      => return Self::ForceAscii,
                    }

                    if has_an && has_en {
                        return Self::ForceAscii;
                    }
                }

                match check3 {
                    true  => Self::Rtl,
                    false => Self::ForceAscii,
                }
            },
            Some(L) => {
                let mut check6 = true;

                for c in classes {
                    match c {
                        L | EN                 => check6 = true,
                        ES | CS | ET | ON | BN => check6 = false,
                        NSM                    => {},
                        _                      => return Self::ForceAscii,
                    }
                }

                match check6 {
                    true  => Self::Ltr,
                    false => Self::ForceLtr,
                }
            },
            Some(AN) => Self::ForceAscii,
            Some(_) => match classes.any(|c| matches!(c, R | AL | AN)) {
                true  => Self::ForceAscii,
                false => Self::ForceLtr,
            },
            None => Self::Ltr,
        }
    }
}

impl FromStr for BidiDetail {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl From<&str> for BidiDetail {
    fn from(value: &str) -> Self {
        Self::parse(value)
    }
}
