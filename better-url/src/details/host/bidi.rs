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

/** Shorthand. **/ const AL  : BidiClass = BidiClass::AL ;
/** Shorthand. **/ const AN  : BidiClass = BidiClass::AN ;
/** Shorthand. **/ const B   : BidiClass = BidiClass::B  ;
/** Shorthand. **/ const BN  : BidiClass = BidiClass::BN ;
/** Shorthand. **/ const CS  : BidiClass = BidiClass::CS ;
/** Shorthand. **/ const EN  : BidiClass = BidiClass::EN ;
/** Shorthand. **/ const ES  : BidiClass = BidiClass::ES ;
/** Shorthand. **/ const ET  : BidiClass = BidiClass::ET ;
/** Shorthand. **/ const L   : BidiClass = BidiClass::L  ;
/** Shorthand. **/ const NSM : BidiClass = BidiClass::NSM;
/** Shorthand. **/ const ON  : BidiClass = BidiClass::ON ;
/** Shorthand. **/ const R   : BidiClass = BidiClass::R  ;
/** Shorthand. **/ const S   : BidiClass = BidiClass::S  ;
/** Shorthand. **/ const WS  : BidiClass = BidiClass::WS ;

/// The [`BidiClass`] for each ASCII byte.
///
/// Used to compute [`DATA`].
const ASCII_CLASSES: [Option<BidiClass>; 256] = [
    /* 00-03 */ Some(BN), Some(BN), Some(BN), Some(BN), /* 04-07 */ Some(BN), Some(BN), Some(BN), Some(BN), /* 18-1B */ Some(BN), Some(S ), Some(B ), Some(S ), /* 2C-2F */ Some(WS), Some(B ), Some(BN), Some(BN),
    /* 10-13 */ Some(BN), Some(BN), Some(BN), Some(BN), /* 34-37 */ Some(BN), Some(BN), Some(BN), Some(BN), /* 48-4B */ Some(BN), Some(BN), Some(BN), Some(BN), /* 5C-5F */ Some(B ), Some(B ), Some(B ), Some(S ),
    /* 20-23 */ Some(WS), Some(ON), Some(ON), Some(ET), /* 64-67 */ Some(ET), Some(ET), Some(ON), Some(ON), /* 78-7B */ Some(ON), Some(ON), Some(ON), Some(ES), /* 7C-7F */ Some(CS), Some(ES), Some(CS), Some(CS),
    /* 30-33 */ Some(EN), Some(EN), Some(EN), Some(EN), /* 74-77 */ Some(EN), Some(EN), Some(EN), Some(EN), /* 78-7B */ Some(EN), Some(EN), Some(CS), Some(ON), /* 7C-7F */ Some(ON), Some(ON), Some(ON), Some(ON),
    /* 40-43 */ Some(ON), Some(L ), Some(L ), Some(L ), /* 74-77 */ Some(L ), Some(L ), Some(L ), Some(L ), /* 78-7B */ Some(L ), Some(L ), Some(L ), Some(L ), /* 7C-7F */ Some(L ), Some(L ), Some(L ), Some(L ),
    /* 50-53 */ Some(L ), Some(L ), Some(L ), Some(L ), /* 74-77 */ Some(L ), Some(L ), Some(L ), Some(L ), /* 78-7B */ Some(L ), Some(L ), Some(L ), Some(ON), /* 7C-7F */ Some(ON), Some(ON), Some(ON), Some(ON),
    /* 60-63 */ Some(ON), Some(L ), Some(L ), Some(L ), /* 74-77 */ Some(L ), Some(L ), Some(L ), Some(L ), /* 78-7B */ Some(L ), Some(L ), Some(L ), Some(L ), /* 7C-7F */ Some(L ), Some(L ), Some(L ), Some(L ),
    /* 70-73 */ Some(L ), Some(L ), Some(L ), Some(L ), /* 74-77 */ Some(L ), Some(L ), Some(L ), Some(L ), /* 78-7B */ Some(L ), Some(L ), Some(L ), Some(ON), /* 7C-7F */ Some(ON), Some(ON), Some(ON), Some(BN),

    // Non ASCII

    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

/// Get [`DATA`].
const fn get_data() -> [u8; 256] {
    let mut ret = [0; 256];

    let mut i = 0;
    while i < 256 {
        if !matches!(ASCII_CLASSES[i as usize], Some(L                              )) {ret[i as usize] |= 0b0000_0001;}
        if !matches!(ASCII_CLASSES[i as usize], Some(L | EN | ES | CS | ET | ON | BN)) {ret[i as usize] |= 0b0000_0010;}
        if !matches!(ASCII_CLASSES[i as usize], Some(L | EN                         )) {ret[i as usize] |= 0b0000_0100;}

        if !matches!(ASCII_CLASSES[i as usize], Some(L | EN | ES | CS | ET | ON | BN)) {ret[i as usize] |= 0b0001_0000;}
        if !matches!(ASCII_CLASSES[i as usize], Some(L | EN | ES | CS | ET | ON | BN)) {ret[i as usize] |= 0b0010_0000;}
        if !matches!(ASCII_CLASSES[i as usize], Some(L | EN | ES | CS | ET | ON | BN)) {ret[i as usize] |= 0b0100_0000;}

        i += 1;
    }

    ret
}

/// A set of bitfields whose bits are, from lowest to highest:
///
/// - If a byte is either not ASCII or can't start a [`BidiDetail::Ltr`] domain label.
///
/// - If a byte is either not ASCII or can't continue a [`BidiDetail::Ltr`] domain label.
///
/// - If a byte is either not ASCII or can't end a [`BidiDetail::Ltr`] domain label.
///
/// - Unused.
///
/// - If a byte is either not ASCII or can't start a [`BidiDetail::ForceLtr`] domain label.
///
/// - If a byte is either not ASCII or can't continue a [`BidiDetail::ForceLtr`] domain label.
///
/// - If a byte is either not ASCII or can't end a [`BidiDetail::ForceLtr`] domain label.
///
/// - Unused.
const DATA: [u8; 256] = get_data();

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
    /// First does [`Self::fast_parse_ascii`], only doing [`Self::parse_unicode`] if it returns [`None`].
    pub fn parse(value: &str) -> Self {
        if let Some(ret) = Self::fast_parse_ascii(value) {
            return ret;
        }

        Self::parse_unicode(value)
    }

    /// Quickly check if an ASCII label is [`Self::Ltr`].
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
    ///             if let Some(x) = BidiDetail::fast_parse_ascii(s) {
    ///                 assert_eq!(x, BidiDetail::parse_unicode(s), "{s:?}");
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn fast_parse_ascii(value: &str) -> Option<Self> {
        let (x, mid, y) = match *value.as_bytes() {
            [                  ] => return Some(Self::Ltr),
            [x                 ] => (x, b"".as_slice(), x),
            [x, ref mid @ .., y] => (x, mid           , y),
        };

        let c1 = DATA[x as usize];
        let c2 = mid.iter().fold(0, |acc, &b| acc | DATA[b as usize]);
        let c3 = DATA[y as usize];

        if (c1 & 1) | (c2 & 2) | (c3 & 4) == 0 {
            Some(Self::Ltr)
        } else if (c1 & 16) | (c2 & 32) | (c3 & 64) == 0 {
            Some(Self::ForceLtr)
        } else {
            None
        }
    }

    /// [`Self::parse`] without first trying [`Self::fast_parse_ascii`].
    ///
    /// Done as the second step of [`Self::parse`] after [`Self::fast_parse_ascii`].
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
