//! [`BetterUrl::new`].

use crate::prelude::*;

mod file;
mod special_not_file;
mod non_special;

impl BetterUrl {
    /// Make a new [`Self`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new<T: TryInto<Self>>(value: T) -> Result<Self, T::Error> {
        value.try_into()
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
