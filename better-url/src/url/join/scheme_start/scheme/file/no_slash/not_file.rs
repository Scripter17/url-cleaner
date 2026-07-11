//! [`BetterUrl::join_scheme_file_no_slash_from_not_file`].

use crate::prelude::*;

impl BetterUrl {
    /// `self` is not a `file` URL.
    pub(super) fn join_scheme_file_no_slash_from_not_file(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = Self::after_scheme(scheme, rest)?;
        Ok(())
    }
}
