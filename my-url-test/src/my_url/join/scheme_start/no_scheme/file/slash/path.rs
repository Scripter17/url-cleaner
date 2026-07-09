//! [`MyUrl::join_no_scheme_file_slash_path`].

use crate::prelude::*;

impl MyUrl {
    pub(super) fn join_no_scheme_file_slash_path(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        let (p, q, f) = split_pqf(rest);

        match p.as_bytes() {
            [b'/', b'a'..=b'z' | b'A'..=b'Z', b':' | b'|', b'/', ..] | [b'/', b'a'..=b'z' | b'A'..=b'Z', b':' | b'|'] => self.set_path(p)?,
            _ => match self.path().as_bytes() {
                [b'/', b'a'..=b'z' | b'A'..=b'Z', b':' | b'|', b'/', ..] | [b'/', b'a'..=b'z' | b'A'..=b'Z', b':' | b'|'] => self.set_path(format!("{}{p}", &self.path()[..3]))?,
                _ => self.set_path(p)?
            }
        }

        if let Some(q) = q {self.set_query(q)?;}

        self.set_fragment(f)?;

        Ok(())
    }
}
