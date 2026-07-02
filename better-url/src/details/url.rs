//! [`UrlDetails`].

use crate::prelude::*;

/// The details of a [`BetterUrl`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UrlDetails {
    /// The [`SchemeDetails`].
    pub scheme: SchemeDetails,
    /// The [`HostDetails`].
    pub host: Option<HostDetails>
}

impl UrlDetails {
    /// Make a [`Self`] from a [`url::Url`].
    pub fn from_url(url: &url::Url) -> Self {
        Self {
            scheme: SchemeDetails::new_unchecked(url.scheme()),
            host  : HostDetails  ::from_url(url)
        }
    }
}
