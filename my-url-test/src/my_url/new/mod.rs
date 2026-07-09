//! [`MyUrl::new`].

use crate::prelude::*;

mod file;
mod special_not_file;
mod non_special;

impl MyUrl {
    /// Make a new [`Self`].
    pub fn new(value: &str) -> Result<Self, InvalidUrl> {
        let start = value.bytes(). position(|b| b > 0x20 && b != 0x7F).unwrap_or(0);
        let end   = value.bytes().rposition(|b| b > 0x20 && b != 0x7F).map_or(0, |x| x + 1);

        let mut value = Cow::Borrowed(&value[start..end]);

        if value.bytes().any(|b| b == b'\t' || b == b'\n' || b == b'\r') {
            value.to_mut().retain(|c| c != '\t' && c != '\n' && c != '\r');
        }

        let i = value.bytes().position(|b| b == b':').ok_or(InvalidUrl::MissingScheme)?;

        let (scheme, rest) = unsafe {(value.get_unchecked(..i), value.get_unchecked(i+1..))};

        Self::after_scheme(Scheme::new(scheme)?, rest)
    }

    /// The after scheme state.
    pub(crate) fn after_scheme(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        match scheme.r#type() {
            SchemeType::File           => Self::new_file            (scheme, rest),
            SchemeType::SpecialNotFile => Self::new_special_not_file(scheme, rest),
            SchemeType::NonSpecial     => Self::new_non_special     (scheme, rest),
        }
    }
}
