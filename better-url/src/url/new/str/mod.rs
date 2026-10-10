//! Constructor stuff.

use crate::prelude::*;

mod file;
mod special_not_file;
mod non_special;

impl BetterUrl {
    /// Parse the rest of a URL given the scheme.
    pub(crate) fn after_scheme_str(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        match scheme.r#type() {
            SchemeType::File           => Self::new_file_str            (scheme, rest),
            SchemeType::SpecialNotFile => Self::new_special_not_file_str(scheme, rest),
            SchemeType::NonSpecial     => Self::new_non_special_str     (scheme, rest),
        }
    }
}
