//! [`BetterUrl::join_scheme_file_abs_path`].

use crate::prelude::*;

impl BetterUrl {
    /// Join with an absolute path.
    pub(super) fn join_scheme_file_abs_path(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        let (p, q, f) = split_pqf(rest);

        let p = FilePath         ::new(p);
        let q = MaybeSpecialQuery::new(q);
        let f = MaybeFragment    ::new(f);

        self.join_abs_path(p, q, f)?;

        Ok(())
    }
}
