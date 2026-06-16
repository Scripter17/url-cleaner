//! [`validate_domain_segment_joiners`].

use icu_properties::{CodePointMapDataBorrowed, props::JoiningType};

use crate::prelude::*;

/// The [`JoiningType`] getter.
static JT: CodePointMapDataBorrowed<JoiningType> = CodePointMapDataBorrowed::new();

/** [`JoiningType::Transparent`]  **/ const JTT: JoiningType = JoiningType::Transparent;
/** [`JoiningType::LeftJoining`]  **/ const JTL: JoiningType = JoiningType::LeftJoining;
/** [`JoiningType::RightJoining`] **/ const JTR: JoiningType = JoiningType::RightJoining;
/** [`JoiningType::DualJoining`]  **/ const JTD: JoiningType = JoiningType::DualJoining;

/** If `c` is [`JTT`]            **/ fn jtt (c: char) -> bool {matches!(JT.get(c), JTT)}
/** If `c` is [`JTL`] or [`JTD`] **/ fn jtld(c: char) -> bool {matches!(JT.get(c), JTL | JTD)}
/** If `c` is [`JTR`] or [`JTD`] **/ fn jtrd(c: char) -> bool {matches!(JT.get(c), JTR | JTD)}
/** If `c` is virama             **/ fn cccv(c: char) -> bool {UTS46.is_virama(c)}

/// [Validity Criteria 8](https://www.unicode.org/reports/tr46/#Validity_Criteria) (CheckJoiners), specifically [RFC 5982](https://www.rfc-editor.org/info/rfc5892/)'s [Appendix A.1](https://www.rfc-editor.org/info/rfc5892/#appendix-A.1) and [Appendix A.2](https://www.rfc-editor.org/info/rfc5892/#appendix-A.2).
pub fn validate_domain_segment_joiners(value: &str) -> bool {
    // Appendix A.1

    if value.contains('\u{200C}') {
        let mut x = value.split('\u{200C}').peekable();

        while let Some((l, &r)) = x.next().zip(x.peek()) {
            if !l.ends_with(cccv) {
                return false;
            }

            if !l.trim_end_matches(jtt).ends_with(jtld) || !r.trim_start_matches(jtt).starts_with(jtrd) {
                return false;
            }
        }
    }

    // Appendix A.2

    if value.contains('\u{200D}') {
        let mut x = value.split('\u{200D}').peekable();

        while let Some((l, _)) = x.next().zip(x.peek()) {
            if !l.ends_with(cccv) {
                return false;
            }
        }
    }

    true
}
