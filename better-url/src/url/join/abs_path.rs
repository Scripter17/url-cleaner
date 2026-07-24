//! [`BetterUrl::join_abs_path`].

use crate::prelude::*;

impl BetterUrl {
    /// Join with an absolute path.
    pub(super) fn join_abs_path<'a, P: Into<SegmentedPath<'a>>, Q: Into<MaybeQuery<'a>>, F: Into<MaybeFragment<'a>>>(&mut self, p: P, q: Q, f: F) -> Result<(), InvalidJoin> {
        let p = p.into();
        let q = q.into();
        let f = f.into();

        if self.path_start() + p.len() + q.search_len() + f.hash_len() > u32::MAX as usize {
            Err(TooLong)?;
        }

        self.join_path_thing(p);
        self.join_push_query(q);
        self.join_push_fragment(f);

        Ok(())
    }
}
