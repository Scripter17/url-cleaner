//! Non-special URLs.

use crate::prelude::*;

mod can_be_a_base;
mod cannot_be_a_base;

impl MyUrl {
    /// Make a new non-special [`Self`].
    pub(super) fn new_non_special(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        match rest.starts_with('/') {
            true  => Self::new_can_be_a_base   (scheme, rest),
            false => Self::new_cannot_be_a_base(scheme, rest),
        }
    }
}
