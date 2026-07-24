//! [`BetterUrl::join_rel_path`].

use crate::prelude::*;

impl BetterUrl {
    /// Join with a relative path.
    pub(super) fn join_rel_path<'a, P: Into<SegmentedPath<'a>>, Q: Into<MaybeQuery<'a>>>(&mut self, p: Option<P>, q: Q, f: MaybeFragment<'a>) -> Result<(), InvalidJoin> {
        let p = p.map(Into::into);
        let q = q.into();

        match p {
            Some(p) => {
                if self.path_start() + p.len() + q.search_len() + f.hash_len() > u32::MAX as usize {
                    Err(TooLong)?;
                }

                self.join_path_thing(p);
                self.join_push_query(q);
                self.join_push_fragment(f);
            },
            None => {
                if q.is_some() {
                    if self.path_after() + q.search_len() + f.hash_len() > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    self.serialization.truncate(self.path_after());
                    self.details.query_mark    = None;
                    self.details.fragment_mark = None;

                    self.join_push_query   (q);
                    self.join_push_fragment(f);
                } else if let Some(x) = self.details.fragment_mark {
                    if x.get() as usize + f.hash_len() > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    self.serialization.truncate(x.get() as usize);
                    self.details.fragment_mark = None;

                    self.join_push_fragment(f);
                } else {
                    if self.len() + f.hash_len() > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    self.join_push_fragment(f);
                }
            }
        }

        Ok(())
    }
}
