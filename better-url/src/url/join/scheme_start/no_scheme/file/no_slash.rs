//! [`BetterUrl::join_no_scheme_file_no_slash`].

use crate::prelude::*;

impl BetterUrl {
    /// Didn't find a leading slash.
    pub(super) fn join_no_scheme_file_no_slash(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        let (p, q, f) = split_pqf(rest);

        match p.as_bytes() {
            [] => {},
            [b'a'..=b'z' | b'A'..=b'Z', b':' | b'|', b'/' | b'\\', ..] | [b'a'..=b'z' | b'A'..=b'Z', b':' | b'|'] => self.set_path(p)?,
            _ => {
                let old = self.path_str();
                let i = old.bytes().rposition(|b| b == b'/').unwrap_or_default();
                self.set_path(format!("{}/{p}", &old[..i]))?;
            }
        }

        if let Some(q) = q {self.set_query(q)?;}

        self.set_fragment(f)?;

        Ok(())
    }
}
