//! [`Profile`].

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// A [`ParamsDiff`] profile.
///
/// Usually made via [`ProfileConfig`].
///
/// Constructed by giving [`ProfilesConfig::make`] a [`Params`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile<'a> {
    /// The [`Params`].
    pub params: Params<'a>
}

impl<'a> Profile<'a> {
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
