//! [`MyUrl::join_scheme_snf_relative_not_slash`].

use crate::prelude::*;

impl MyUrl {
    pub(super) fn join_scheme_snf_relative_not_slash(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        let (p, q, f) = split_pqf(rest);

        if !p.is_empty() {
            let mut x = self.path().to_string();
            x.truncate(x.bytes().rposition(|b| b == b'/').expect("???") + 1);
            x.push_str(p);
            self.set_path(x)?;
        }

        if let Some(q) = q {
            self.set_query(q)?;
        }

        if let Some(f) = f {
            self.set_fragment(f)?;
        }

        Ok(())
    }
}
