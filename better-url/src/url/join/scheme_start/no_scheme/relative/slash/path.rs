//! [`BetterUrl::join_no_scheme_relative_slash_path`].

use crate::prelude::*;

impl BetterUrl {
    /// Found a leading path.
    pub(super) fn join_no_scheme_relative_slash_path(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        let (p, q, f) = split_pqf(&rest[1..]);

        self.set_path(p)?;
        if let Some(q) = q {self.set_query   (q)?;}
        if let Some(f) = f {self.set_fragment(f)?;}

        Ok(())
    }
}
