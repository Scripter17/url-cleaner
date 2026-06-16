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
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn from_url(url: &url::Url) -> Self {
        Self {
            scheme: SchemeDetails::new_unchecked(url.scheme()),
            host: url.host_str().map(|x| Host::new(x).expect("A valid host").into_parts().1)
        }
    }
}
