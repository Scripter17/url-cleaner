//! FQDN stuff.

use crate::prelude::*;

impl DomainDetails {
    /// Returns [`true`] if the domain has an FQDN dot.
    ///
    /// Identical to [`Self::has_fqddot`].
    pub fn has_fqddot(self) -> bool {
        self.fq
    }

    /// Returns [`true`] if the domain is an FQDN.
    ///
    /// Identical to [`Self::is_fqdn`].
    pub fn is_fqdn(self) -> bool {
        self.fq
    }

    /// The [`Range::start`] of the FQDN dot.
    pub fn fqddot_start(self) -> Option<usize> {
        match self.fq {
            false => None,
            true  => Some(self.sa as usize)
        }
    }

    /// The [`Range::end`] of the FQDN dot.
    pub fn fqddot_after(self) -> Option<usize> {
        match self.fq {
            false => None,
            true  => Some(self.sa as usize + 1)
        }
    }

    /// The [`Range`] of the FQDN dot.
    pub fn fqddot_range(self) -> Option<Range<usize>> {
        Some(self.fqddot_start()? .. self.fqddot_after()?)
    }
}
