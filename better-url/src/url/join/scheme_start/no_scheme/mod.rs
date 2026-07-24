//! [`BetterUrl::join_no_scheme`].

use crate::prelude::*;

mod file;
mod special_not_file;
mod non_special;

impl BetterUrl {
    /// Join without a scheme.
    pub(super) fn join_no_scheme(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        match self.can_be_a_base() {
            true => match self.scheme_type() {
                SchemeType::File           => self.join_no_scheme_file(rest)?,
                SchemeType::SpecialNotFile => self.join_no_scheme_snf (rest)?,
                SchemeType::NonSpecial     => self.join_no_scheme_ns  (rest)?,
            },
            false => match rest.strip_prefix('#') {
                Some(f) => {
                    if let Some(x) = self.details.fragment_mark.take() {
                        self.serialization.truncate(x.get() as usize);
                    }
                    self.join_push_fragment(f);
                }
                None    => Err(InvalidJoin::MissingSchemeNonRelativeUrl)?,
            }
        }

        Ok(())
    }
}
