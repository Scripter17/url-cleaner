//! Normal stuff.

use crate::prelude::*;

impl BetterDomainHost<'_> {
    /// Get the normal.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// assert_eq!(BetterDomainHost::try_from(        "example.com" ).unwrap().normal(),         "example.com");
    /// assert_eq!(BetterDomainHost::try_from(        "example.com.").unwrap().normal(),         "example.com");
    /// assert_eq!(BetterDomainHost::try_from(    "www.example.com" ).unwrap().normal(),         "example.com");
    /// assert_eq!(BetterDomainHost::try_from(    "www.example.com.").unwrap().normal(),         "example.com");
    /// assert_eq!(BetterDomainHost::try_from("www.abc.example.com" ).unwrap().normal(), "www.abc.example.com");
    /// assert_eq!(BetterDomainHost::try_from("www.abc.example.com.").unwrap().normal(), "www.abc.example.com");
    /// ```
    pub fn normal(&self) -> &str {
        &self.host[self.details.normal_range()]
    }
}
