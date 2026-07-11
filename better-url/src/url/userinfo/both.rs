//! [`Userinfo`].

use crate::prelude::*;

impl BetterUrl {
    /// The [`Range::start`] of the userinfo.
    fn userinfo_start(&self) -> Option<usize> {
        self.username_start()
    }

    /// The [`Range::end`] of the userinfo.
    fn userinfo_after(&self) -> Option<usize> {
        self.password_after().or(self.username_after())
    }

    /// The [`Range`] of the userinfo.
    fn userinfo_range(&self) -> Option<Range<usize>> {
        Some(self.userinfo_start()? .. self.userinfo_after()?)
    }

    /// The userinfo as a [`str`].
    pub fn userinfo_str(&self) -> &str {
        self.userinfo_range().map_or("", |r| &self.serialization[r])
    }

    /// The [`Userinfo`].
    pub fn userinfo(&self) -> Userinfo<'_> {
        let raw = self.userinfo_str();
        let ps = raw.find(':').and_then(|x| NonZero::new(x + 1));

        unsafe {
            Userinfo::new_unchecked(raw, ps)
        }
    }
}
