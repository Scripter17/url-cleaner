//! [`MyUrl::join_no_scheme`].

use crate::prelude::*;

mod relative;
mod file;

impl MyUrl {
    pub(super) fn join_no_scheme(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        match self.path().starts_with('/') || self.host().is_some() {
            true => match self.details.scheme.is_file() {
                true  => self.join_no_scheme_file    (rest)?,
                false => self.join_no_scheme_relative(rest)?,
            },
            false => match rest.strip_prefix('#') {
                Some(f) => self.set_fragment(f)?,
                None    => Err(InvalidJoin::MissingSchemeNonRelativeUrl)?,
            }
        }

        Ok(())
    }
}
