//! [`MyUrl::join_no_scheme_relative_slash_authority`].

use crate::prelude::*;

impl MyUrl {
    pub(super) fn join_no_scheme_relative_slash_authority(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        *self = MyUrl::after_scheme(self.scheme().parse().expect("???"), rest)?;
        Ok(())
    }
}
