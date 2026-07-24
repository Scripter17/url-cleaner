//! Password stuff.

use crate::prelude::*;

impl<'a> Userinfo<'a> {
    /// If it has a visible password.
    pub fn has_visible_password(&self) -> bool {
        self.password_start.is_some()
    }



    /// The [`Range::start`] of the password.
    pub(crate) fn password_start(&self) -> Option<usize> {
        Some(self.password_start?.get())
    }

    /// The [`Range::end`] of the password.
    pub(crate) fn password_after(&self) -> Option<usize> {
        self.password_start.map(|_| self.len())
    }

    /// The [`Range`] of the password.
    pub(crate) fn password_range(&self) -> Option<Range<usize>> {
        Some(self.password_start()? .. self.password_after()?)
    }



    /// The visible password as a [`str`].
    ///
    /// If the password is the empty string, and thus doesn't show in the URL, returns [`None`].
    pub fn visible_password_str(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.password_range()?)})
    }

    /// The password as a [`str`].
    ///
    /// Please note that in the case of an empty password the returned `str` will not be a substring of the URL.
    ///
    /// If you need that property, see [`Self::visible_password_str`].
    pub fn password_str(&self) -> &str {
        self.visible_password_str().unwrap_or_default()
    }

    /// The visible [`Password`].
    ///
    /// See [`Self::visible_password_str`] foe details.
    pub fn visible_password(&self) -> Option<Password<'_>> {
        Some(unsafe {Password::new_unchecked(self.visible_password_str()?)})
    }

    /// The [`Password`].
    ///
    /// See [`Self::password_str`] foe details.
    pub fn password(&self) -> Password<'_> {
        unsafe {Password::new_unchecked(self.password_str())}
    }



    /// Set the password.
    pub fn set_password<'b, T: Into<Password<'b>>>(&mut self, value: T) {
        match (self.password_start(), value.into().as_str()) {
            (None, "" ) => {},
            (None, new) => {
                self.password_start = NonZero::new(self.len() + 1);
                self.userinfo.extend([":", new]);
            },
            (Some(x), "") => {
                self.userinfo.retain_range(..x - 1);
                self.password_start = None;
            },
            (Some(x), new) => {
                self.userinfo.replace_range(x.., new);
            }
        }
    }
}
