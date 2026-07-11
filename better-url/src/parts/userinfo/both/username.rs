//! Username stuff.

use crate::prelude::*;

impl<'a> Userinfo<'a> {
    /// The [`Range::start`] of the username.
    pub(crate) fn username_start(&self) -> usize {
        0
    }

    /// The [`Range::end`] of the username.
    pub(crate) fn username_after(&self) -> usize {
        self.ps.map_or(self.len(), |x| x.get() - 1)
    }

    /// The [`Range`] of the username.
    pub(crate) fn username_range(&self) -> Range<usize> {
        self.username_start() .. self.username_after()
    }

    /// Borrow the username as a [`str`].
    pub fn username_str(&self) -> &str {
        &self.raw[self.username_range()]
    }

    /// Make a [`Username`].
    pub fn username(&self) -> Username<'_> {
        Username(self.username_str().into())
    }

    /// Set the username.
    pub fn set_username<'b, T: Into<Username<'b>>>(&mut self, value: T) {
        let value = value.into().into_inner();

        self.raw.replace_range(self.username_range(), &value);

        if self.ps.is_some() {
            self.ps = NonZero::new(value.len() + 1);
        }
    }
}
