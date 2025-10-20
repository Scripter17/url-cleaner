//! [`ProfileConfig`].

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// A [`ParamsDiff`] profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ProfileConfig {
    /// The [`ParamsDiff`].
    ///
    /// Defaults to the default [`ParamsDiff`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub params_diff: ParamsDiff
}

impl ProfileConfig {
    /// Make a [`Profile`] with the provided [`Params`].
    pub fn make<'a>(self, mut params: Params<'a>) -> Profile<'a> {
        self.params_diff.apply_once(&mut params);
        Profile {params}
    }
}
