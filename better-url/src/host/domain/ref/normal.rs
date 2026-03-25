//! Normal stuff.

use crate::prelude::*;

impl<'a> BetterRefDomainHost<'a> {
    /// Get the normal.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert_eq!(BetterRefDomainHost::try_from(        "example.com" ).unwrap().normal(),         "example.com");
    /// assert_eq!(BetterRefDomainHost::try_from(        "example.com.").unwrap().normal(),         "example.com");
    /// assert_eq!(BetterRefDomainHost::try_from(    "www.example.com" ).unwrap().normal(),         "example.com");
    /// assert_eq!(BetterRefDomainHost::try_from(    "www.example.com.").unwrap().normal(),         "example.com");
    /// assert_eq!(BetterRefDomainHost::try_from("www.abc.example.com" ).unwrap().normal(), "www.abc.example.com");
    /// assert_eq!(BetterRefDomainHost::try_from("www.abc.example.com.").unwrap().normal(), "www.abc.example.com");
    /// ```
    pub fn normal(self) -> &'a str {
        &self.host[self.details.normal_range()]
    }
}
