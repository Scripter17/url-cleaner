//! [`BetterUrl::join_scheme`].

use crate::prelude::*;

mod file;
mod special_not_file;
mod non_special;

impl BetterUrl {
    /// Join with a scheme.
    pub(super) fn join_scheme(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        match self.scheme() == scheme {
            true => match scheme.r#type() {
                SchemeType::File           => self.join_scheme_file(        rest),
                SchemeType::SpecialNotFile => self.join_scheme_snf (        rest),
                SchemeType::NonSpecial     => self.join_scheme_ns  (scheme, rest),
            },
            false => {
                *self = Self::after_scheme(scheme, rest)?;
                Ok(())
            }
        }
    }
}
