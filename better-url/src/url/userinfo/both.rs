//! [`Userinfo`].

use crate::prelude::*;

impl BetterUrl {
    /// If it has a visible userinfo.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert!(!BetterUrl::new("https://example.com"                  ).unwrap().has_visible_userinfo());
    /// assert!( BetterUrl::new("https://username@example.com"         ).unwrap().has_visible_userinfo());
    /// assert!( BetterUrl::new("https://:password@example.com"        ).unwrap().has_visible_userinfo());
    /// assert!( BetterUrl::new("https://username:password@example.com").unwrap().has_visible_userinfo());
    /// ```
    pub fn has_visible_userinfo(&self) -> bool {
        self.username_start().is_some()
    }



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



    /// The visible userinfo as a [`str`].
    ///
    /// If the userinfo is the empty string, and thus doesn't show in the URL, returns [`None`].
    ///
    /// Thus, any [`str`] returned is guaranteed to be a substring of the URL.
    pub fn visible_userinfo_str(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.userinfo_range()?)})
    }

    /// The userinfo as a [`str`].
    ///
    /// Please note that in the case of an empty userinfo the returned `str` will not be a substring of the URL.
    ///
    /// If you need that property, see [`Self::visible_userinfo_str`].
    pub fn userinfo_str(&self) -> &str {
        self.visible_userinfo_str().unwrap_or_default()
    }

    /// The visible [`Userinfo`].
    ///
    /// See [`Self::visible_userinfo_str`] foe details.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let url = BetterUrl::new("https://example.com").unwrap();
    ///
    /// assert!(url.visible_userinfo().is_none());
    ///
    ///
    /// let url = BetterUrl::new("https://username@example.com").unwrap();
    /// let userinfo = url.visible_userinfo().unwrap();
    ///
    /// assert_eq!(userinfo.username_str        (), "username");
    /// assert_eq!(userinfo.visible_password_str(), None      );
    /// assert_eq!(userinfo.password_str        (), ""        );
    ///
    ///
    /// let url = BetterUrl::new("https://:password@example.com").unwrap();
    /// let userinfo = url.visible_userinfo().unwrap();
    ///
    /// assert_eq!(userinfo.username_str        (), ""              );
    /// assert_eq!(userinfo.visible_password_str(), Some("password"));
    /// assert_eq!(userinfo.password_str        (), "password"      );
    ///
    ///
    /// let url = BetterUrl::new("https://username:password@example.com").unwrap();
    /// let userinfo = url.visible_userinfo().unwrap();
    ///
    /// assert_eq!(userinfo.username_str        (), "username"      );
    /// assert_eq!(userinfo.visible_password_str(), Some("password"));
    /// assert_eq!(userinfo.password_str        (), "password"      );
    /// ```
    pub fn visible_userinfo(&self) -> Option<Userinfo<'_>> {
        let raw = self.visible_userinfo_str()?;
        let password_start = self.password_start().and_then(|password_start| NonZero::new(password_start - self.username_start()?));

        Some(unsafe {
            Userinfo::new_unchecked(raw, password_start)
        })
    }

    /// The [`Userinfo`].
    ///
    /// See [`Self::userinfo_str`] foe details.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let url = BetterUrl::new("https://example.com").unwrap();
    ///
    /// assert!(url.visible_userinfo().is_none());
    ///
    ///
    /// let url = BetterUrl::new("https://username@example.com").unwrap();
    /// let userinfo = url.userinfo();
    ///
    /// assert_eq!(userinfo.username_str        (), "username");
    /// assert_eq!(userinfo.visible_password_str(), None      );
    /// assert_eq!(userinfo.password_str        (), ""        );
    ///
    ///
    /// let url = BetterUrl::new("https://:password@example.com").unwrap();
    /// let userinfo = url.userinfo();
    ///
    /// assert_eq!(userinfo.username_str        (), ""              );
    /// assert_eq!(userinfo.visible_password_str(), Some("password"));
    /// assert_eq!(userinfo.password_str        (), "password"      );
    ///
    ///
    /// let url = BetterUrl::new("https://username:password@example.com").unwrap();
    /// let userinfo = url.userinfo();
    ///
    /// assert_eq!(userinfo.username_str        (), "username"      );
    /// assert_eq!(userinfo.visible_password_str(), Some("password"));
    /// assert_eq!(userinfo.password_str        (), "password"      );
    /// ```
    pub fn userinfo(&self) -> Userinfo<'_> {
        self.visible_userinfo().unwrap_or(unsafe {Userinfo::new_unchecked("", None)})
    }
}
