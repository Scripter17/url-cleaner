//! [`BetterUrl::join_no_scheme_file_rel_path`].

use crate::prelude::*;

impl BetterUrl {
    /// Join with a relative path.
    pub(super) fn join_no_scheme_file_rel_path(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        let (p, q, f) = split_pqf(rest);

        let p = file_path_join_rel(self.path_str(), p).map(FilePath::new);
        let q = MaybeSpecialQuery::new(q);
        let f = MaybeFragment    ::new(f);

        self.join_rel_path(p, q, f)?;

        Ok(())
    }
}
