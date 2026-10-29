//! [`DOMAIN_BYTE_CLASS_DATA`].

use icu_properties::props::BidiClass;

use crate::prelude::*;

/// A classification of each byte whose bits, from lowest to highest, are:
///
/// 1. If it's an ASCII uppercase letter.
///
/// 2. If it's an ASCII number.
///
/// 3. If it's not ASCII.
///
/// 4. If it's in [`FORBIDDEN_DOMAIN_SEGMENT_INPUT`].
///
/// 5. If it's in [`FORBIDDEN_DOMAIN_SEGMENTS_INPUT`].
///
/// 6. If it's not ASCII and/or can't start a [`BidiDetail::Ltr`] domain label.
///
/// 7. If it's not ASCII and/or can't continue a [`BidiDetail::Ltr`] domain label.
///
/// 8. If it's not ASCII and/or can't end a [`BidiDetail::Ltr`] domain label.
///
/// The mask `0b0000_1101` being nonzero is equivalent to the byte not being valid for a [`DomainSegment`] literal.
///
/// The mask `0b0001_0101` being nonzero is equivalent to the byte not being valid for a [`DomainSegments`] literal.
///
/// THe mask `0b0100_0000` being nonzero is equivalent to being not ASCII and/or not being able to start, continue, or end a [`BidiDetail::ForceLtr`] segment.
pub const DOMAIN_BYTE_CLASS_DATA: [u8; 256] = get_domain_byte_class_data();

/// Get [`DOMAIN_BYTE_CLASS_DATA`].
const fn get_domain_byte_class_data() -> [u8; 256] {
    let mut ret = [0; 256];

    let mut i = 0;
    while i < 256 {
        if  (i as u8).is_ascii_uppercase() { ret[i as usize] |= 0b0000_0001; }
        if  (i as u8).is_ascii_digit    () { ret[i as usize] |= 0b0000_0010; }
        if !(i as u8).is_ascii          () { ret[i as usize] |= 0b0000_0100; }

        if FORBIDDEN_DOMAIN_SEGMENT_INPUT .contains(i as u8) { ret[i as usize] |= 0b0000_1000; }
        if FORBIDDEN_DOMAIN_SEGMENTS_INPUT.contains(i as u8) { ret[i as usize] |= 0b0001_0000; }

        if !matches!(ASCII_BIDI_DETAIL_BYTE_CLASS_DATA_BASE[i as usize], Some(L                              )) {ret[i as usize] |= 0b0010_0000;}
        if !matches!(ASCII_BIDI_DETAIL_BYTE_CLASS_DATA_BASE[i as usize], Some(L | EN | ES | CS | ET | ON | BN)) {ret[i as usize] |= 0b0100_0000;}
        if !matches!(ASCII_BIDI_DETAIL_BYTE_CLASS_DATA_BASE[i as usize], Some(L | EN                         )) {ret[i as usize] |= 0b1000_0000;}

        i += 1;
    }

    ret
}

/** Shorthand. */ const B   : BidiClass = BidiClass::B ;
/** Shorthand. */ const BN  : BidiClass = BidiClass::BN;
/** Shorthand. */ const CS  : BidiClass = BidiClass::CS;
/** Shorthand. */ const EN  : BidiClass = BidiClass::EN;
/** Shorthand. */ const ES  : BidiClass = BidiClass::ES;
/** Shorthand. */ const ET  : BidiClass = BidiClass::ET;
/** Shorthand. */ const L   : BidiClass = BidiClass::L ;
/** Shorthand. */ const ON  : BidiClass = BidiClass::ON;
/** Shorthand. */ const S   : BidiClass = BidiClass::S ;
/** Shorthand. */ const WS  : BidiClass = BidiClass::WS;

/// The [`BidiClass`] for each ASCII byte.
///
/// Used to compute [`ASCII_BIDI_DETAIL_BYTE_CLASS_DATA_BASE`].
const ASCII_BIDI_DETAIL_BYTE_CLASS_DATA_BASE: [Option<BidiClass>; 256] = [
                /* X0 */  /* X1 */  /* X2 */  /* X3 */  /* X4 */  /* X5 */  /* X6 */  /* X7 */  /* X8 */  /* X9 */  /* XA */  /* XB */  /* XC */  /* XD */  /* XE */  /* XF */
    /* 0X-0X */ Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(S ), Some(B ), Some(S ), Some(WS), Some(B ), Some(BN), Some(BN),
    /* 1X-1X */ Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(BN), Some(B ), Some(B ), Some(B ), Some(S ),
    /* 2X-2X */ Some(WS), Some(ON), Some(ON), Some(ET), Some(ET), Some(ET), Some(ON), Some(ON), Some(ON), Some(ON), Some(ON), Some(ES), Some(CS), Some(ES), Some(CS), Some(CS),
    /* 3X-3X */ Some(EN), Some(EN), Some(EN), Some(EN), Some(EN), Some(EN), Some(EN), Some(EN), Some(EN), Some(EN), Some(CS), Some(ON), Some(ON), Some(ON), Some(ON), Some(ON),
    /* 4X-4X */ Some(ON), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ),
    /* 5X-5X */ Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(ON), Some(ON), Some(ON), Some(ON), Some(ON),
    /* 6X-6X */ Some(ON), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ),
    /* 7X-7X */ Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(L ), Some(ON), Some(ON), Some(ON), Some(ON), Some(BN),

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
