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
                    let pa = self.path_after();

                    if pa + q.search_len() + f.hash_len() > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    unsafe {
                        self.serialization.modify(|x| x.truncate(pa));
                    }
                    self.details.query_mark    = None;
                    self.details.fragment_mark = None;

                    self.join_push_query   (q);
                    self.join_push_fragment(f);
                } else if let Some(fm) = self.details.fragment_mark {
                    if fm.get() as usize + f.hash_len() > u32::MAX as usize {
                        Err(TooLong)?;
                    }

                    unsafe {
                        self.serialization.modify(|x| x.truncate(fm.get() as usize));
                    }
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
