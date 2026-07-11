//! Non-special URLs.

use crate::prelude::*;

mod can_be_a_base;
mod cannot_be_a_base;

impl BetterUrl {
    /// Make a new non-special [`Self`].
    pub(super) fn new_non_special(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        match rest.as_bytes() {
            [b'/', ..]  => Self::new_can_be_a_base   (scheme, rest),
            _           => Self::new_cannot_be_a_base(scheme, rest),
        }
    }
}
