//! [`Profile`].

use crate::types::*;

use serde::Serialize;

/// A [`ParamsDiff`] profile.
///
/// Constructed by giving [`ProfilesConfig::make`] a [`Params`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Profile<'a> {
    /// The [`Params`].
    pub(super) params: Params<'a>
}

impl<'a> Profile<'a> {
    /// Get the [`Params`].
    pub fn params(&self) -> &Params<'a> {
        &self.params
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&'a self) -> Self {
        Self {
            params: self.params.borrowed()
        }
    }
}

impl<'a> From<Profile<'a>> for Params<'a> {
    fn from(value: Profile<'a>) -> Self {
        value.params
    }
}
