//! [`Profile`].

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// A [`Params`] that has had a [`ParamsDiff`] from a [`ProfileConfig`] applied.
///
/// Usually made from [`ProfileConfig`].
///
/// Mainly used inside [`Profiles`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
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
