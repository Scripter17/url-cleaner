//! [`BetterUrl::join_no_scheme_relative_not_slash`].

use crate::prelude::*;

impl BetterUrl {
    /// Didn't find a leading slash.
    pub(super) fn join_no_scheme_relative_not_slash(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        let (p, q, f) = split_pqf(rest);

        if !p.is_empty() {
            let mut x = self.path().to_string();
            if let Some(i) = x.bytes().rposition(|b| b == b'/') {
                x.truncate(i);
            }
            x.extend(["/", p]);
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
