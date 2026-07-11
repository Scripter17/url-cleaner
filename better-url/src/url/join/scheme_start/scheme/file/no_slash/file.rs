//! [`BetterUrl::join_scheme_file_no_slash_from_file`].

use crate::prelude::*;

impl BetterUrl {
    /// `self` is a `file` URL.
    pub(super) fn join_scheme_file_no_slash_from_file(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        let (p, q, f) = split_pqf(rest);

        match p.as_bytes() {
            [] => {
                if let Some(q) = q {self.set_query   (q)?};
                                    self.set_fragment(f)?;
            },
            [b'a'..=b'z' | b'A'..=b'Z', b':' | b'|', b'/' | b'\\', ..] | [b'a'..=b'z' | b'A'..=b'Z', b':' | b'|'] => {
                                    self.set_path    (p)?;
                if let Some(q) = q {self.set_query   (q)?;}
                                    self.set_fragment(f)?;
            },
            _  => {
                let old = self.path_str();
                let i = old.bytes().rposition(|b| b == b'/').unwrap_or_default();
                let p = format!("{}/{p}", &old[..i]);

                                    self.set_path    (p)?;
                if let Some(q) = q {self.set_query   (q)?;}
                                    self.set_fragment(f)?;
            }
        }

        Ok(())
    }
}
