//! [`BidiDetail`].

use icu_properties::{CodePointMapDataBorrowed, props::BidiClass};

use crate::prelude::*;

/// Info about a domain segment's directionality.
///
/// Per [RFC 5893](https://www.rfc-editor.org/info/rfc5893):
///
/// - An RTL segment is a segment that contains at least one character of type R, AL, or AN.
///
/// - An LTR segment is any segment that is not an RTL segment.
///
/// "Type" refers to the [bidirectional class](https://unicode.org/reports/tr44/#Bidi_Class_Values).
///
/// Additionally, any domain containing an RTL segment requires that all its segments satisfy the [Bidi rule](https://www.rfc-editor.org/info/rfc5893/#section-2).
///
/// Consequently, [`Self::parse`]ing any RTL segment that doesn't satisfy the Bidi rule returns [`InvalidDomainSegment`], as it can never appear in a valid domain.
///
/// Notably, domain names with only LTR segments are allowed to have segments that violate the Bidi rule.
///
/// Here, [`Self::Ltr`] refers to LTR segments that satisfy the Bidi rule and [`Self::Inv`] (better name pending) refers to LTR segments that don't.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BidiDetail {
    /// LTR and satisfying the Bidi rule.
    Ltr = 0b00,
    /// LTR and not satisfying the Bidi rule.
    ///
    /// Cannot be in the same domain as [`Self::Rtl`].
    Inv = 0b01,
    /// RTL and satisfying the Bidi rule.
    ///
    /// Cannot be in the same domain as [`Self::Inv`].
    Rtl = 0b11,
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
    /// Parse a valid decoded domain segment.
    /// # Errors
    /// If the segment is RTL but does not satisfy the Bidi rule, returns the error [`InvalidDomainSegment`].
    pub fn parse(value: &str) -> Result<Self, InvalidDomainSegment> {
        if value.bytes().all(|b| matches!(b, b'a'..=b'z' | b'-')) {
            return Ok(Self::Ltr);
        }

        let mut classes = value.chars().map(|c| BC.get(c));

        Ok(match classes.next() {
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
                        _ => Err(InvalidDomainSegment)?
                    }
                }

                match check3 {
                    true  => Self::Rtl,
                    false => Err(InvalidDomainSegment)?,
                }
            },
            Some(L) => {
                let mut check6 = true;

                for c in classes {
                    match c {
                        NSM => {},
                        ES | CS | ET | ON | BN => check6 = false,
                        L | EN                 => check6 = true,
                        R | AL | AN => Err(InvalidDomainSegment)?,
                        _ => return Ok(Self::Inv),
                    }
                }

                match check6 {
                    true  => Self::Ltr,
                    false => Self::Inv,
                }
            }
            _ => match classes.any(|c| matches!(c, R | AL | AN)) {
                true  => Err(InvalidDomainSegment)?,
                false => Self::Inv,
            }
        })
    }
}

impl FromStr for BidiDetail {
    type Err = InvalidDomainSegment;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for BidiDetail {
    type Error = InvalidDomainSegment;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}
