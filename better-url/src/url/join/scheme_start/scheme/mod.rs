//! [`BetterUrl::join_scheme`].

use crate::prelude::*;

mod file;
mod snf;
mod ns;

impl BetterUrl {
    /// Found a scheme.
    pub(super) fn join_scheme(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        match scheme.r#type() {
            SchemeType::File           => self.join_scheme_file(scheme, rest),
            SchemeType::SpecialNotFile => self.join_scheme_snf (scheme, rest),
            SchemeType::NonSpecial     => self.join_scheme_ns  (scheme, rest),
        }
    }
}
