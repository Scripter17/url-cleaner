//! Validate domain segments.

use crate::prelude::*;

use icu_properties::{CodePointMapDataBorrowed, props::{GeneralCategory, GeneralCategoryGroup}};

/// the general category getter.
static GC: CodePointMapDataBorrowed<GeneralCategory> = CodePointMapDataBorrowed::new();

/// If `value` satisfies [validity criteria](https://www.unicode.org/reports/tr46/#Validity_Criteria) 1 through 8.
///
/// 9 is handled by [`BidiDetail`].
pub fn mostly_validate_domain_segment_unicode(value: &str) -> bool {
    UTS46.normalize_validate(value.chars()).eq(value.chars())
        && !value.starts_with("xn--")
        && !value.contains('.')
        && !value.starts_with(|c| GeneralCategoryGroup::Mark.contains(GC.get(c)))
        && !value.contains('\u{FFFD}')
        && validate_domain_segment_joiners(value)
}

/// [`mostly_validate_domain_segment_unicode`], assuming the input has been through [`uts46_map_normalize`].
pub fn mostly_validate_uts46_domain_segment_unicode(value: &str) -> bool {
    !value.starts_with("xn--")
        && !value.contains('.')
        && !value.starts_with(|c| GeneralCategoryGroup::Mark.contains(GC.get(c)))
        && !value.contains('\u{FFFD}')
        && validate_domain_segment_joiners(value)
}
