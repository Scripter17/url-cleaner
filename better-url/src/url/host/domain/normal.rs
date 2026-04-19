//! Normal stuff.

use crate::prelude::*;

impl BetterUrl {
    /// The domain's normal.
    pub fn domain_normal(&self) -> Option<&str> {
        Some(&self.host_str()?[self.domain_details()?.normal_range()])
    }
}
