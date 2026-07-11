//! Password stuff.

use crate::prelude::*;

impl<'a> Userinfo<'a> {
    /// The [`Range::start`] of the password.
    pub(crate) fn password_start(&self) -> usize {
        self.ps.map_or(0, NonZero::get)
    }

    /// The [`Range::end`] of the password.
    pub(crate) fn password_after(&self) -> usize {
        match self.ps {
            None    => 0,
            Some(_) => self.len(),
        }
    }

    /// The [`Range`] of the password.
    pub(crate) fn password_range(&self) -> Range<usize> {
        self.password_start() .. self.password_after()
    }

    /// Borrow the password as a [`str`].
    pub fn password_str(&self) -> &str {
        &self.raw[self.password_range()]
    }

    /// Make a [`Password`].
    pub fn password(&self) -> Password<'_> {
        Password(Cow::Borrowed(self.password_str()))
    }

    /// Set the password.
    pub fn set_password<'b, T: Into<Password<'b>>>(&mut self, value: T) {
        match (self.password_start(), value.into().as_str()) {
            (0, "" ) => {},
            (0, new) => {
                self.ps = NonZero::new(self.raw.len() + 1);
                self.raw.to_mut().extend([":", new]);
            },
            (x, "") => {
                self.raw.retain_range(..x - 1);
                self.ps = None;
            },
            (x, new) => {
                self.raw.to_mut().replace_range(x.., new);
            }
        }
    }
}
