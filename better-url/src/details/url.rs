//! [`UrlDetails`].

use crate::prelude::*;

/// The details of a [`BetterUrl`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UrlDetails {
    /// The [`SchemeDetails`].
    pub scheme: SchemeDetails,
    /// The [`HostDetails`].
    pub host: Option<HostDetails>
}
