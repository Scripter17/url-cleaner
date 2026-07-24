//! [`BidiDetail`].

use icu_properties::{CodePointMapDataBorrowed, props::BidiClass};

use crate::prelude::*;

/// Info about a domain segment's directionality.
///
/// Per [RFC 5893](https://www.rfc-editor.org/info/rfc5893):
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BidiDetail {
    /// Left to right.
    Ltr,
    /// LTR but not following the Bidi rule, requiring all other segments be LTR.
    ForceLtr,
    /// Right to left.
    Rtl,
    /// RTL but not following the Bidi rule, requiring all segments be ASCII.
    ForceAscii,
}

/// Bidi class getter.
static BC: CodePointMapDataBorrowed<BidiClass> = CodePointMapDataBorrowed::new();

/** Bidi class L.   **/ const L  : BidiClass = BidiClass::LeftToRight;
/** Bidi class R.   **/ const R  : BidiClass = BidiClass::RightToLeft;
/** Bidi class AL.  **/ const AL : BidiClass = BidiClass::ArabicLetter;
/** Bidi class AN.  **/ const AN : BidiClass = BidiClass::ArabicNumber;
/** Bidi class EN.  **/ const EN : BidiClass = BidiClass::EuropeanNumber;
/** Bidi class ES.  **/ const ES : BidiClass = BidiClass::EuropeanSeparator;
/** Bidi class CS.  **/ const CS : BidiClass = BidiClass::CommonSeparator;
/** Bidi class ET.  **/ const ET : BidiClass = BidiClass::EuropeanTerminator;
/** Bidi class ON.  **/ const ON : BidiClass = BidiClass::OtherNeutral;
/** Bidi class BN.  **/ const BN : BidiClass = BidiClass::BoundaryNeutral;
/** Bidi class NSM. **/ const NSM: BidiClass = BidiClass::NonspacingMark;

impl BidiDetail {
    /// Parse a decoded domain segment.
    pub fn parse(value: &str) -> Self {
        if value.bytes().all(|b| matches!(b, b'a'..=b'z' | b'-')) {
            return Self::Ltr;
        }

        let mut classes = value.chars().map(|c| BC.get(c));

        match classes.next() {
            Some(R | AL) => {
                let mut check3 = true;
                let mut has_en = false;
                let mut has_an = false;

                for c in classes {
                    match c {
                        NSM => {}
                        ES | CS | ET | ON | BN => check3 = false,
                        R | AL                 => check3 = true,
                        AN if !has_en => {check3 = true; has_an = true;},
                        EN if !has_an => {check3 = true; has_en = true;},
                        _ => return Self::ForceAscii,
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
                        NSM => {},
                        ES | CS | ET | ON | BN => check6 = false,
                        L | EN                 => check6 = true,
                        R | AL | AN => return Self::ForceAscii,
                        _ => return Self::ForceLtr,
                    }
                }

                match check6 {
                    true  => Self::Ltr,
                    false => Self::ForceLtr,
                }
            }
            _ => match classes.any(|c| matches!(c, R | AL | AN)) {
                true  => Self::ForceAscii,
                false => Self::ForceLtr,
            }
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
