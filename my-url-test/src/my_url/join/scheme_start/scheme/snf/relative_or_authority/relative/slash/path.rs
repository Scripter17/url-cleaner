//! [`MyUrl::join_scheme_snf_relative_slash_path`].

use crate::prelude::*;

impl MyUrl {
    pub(super) fn join_scheme_snf_relative_slash_path(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        let (p, q, f) = split_pqf(rest);

        self.set_path(p)?;

        if let Some(q) = q {self.set_query(q)?;}

        self.set_fragment(f)?;

        Ok(())
    }
}
