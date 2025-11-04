//! [`ProfilesConfig`].

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// The configuration for a [`Profiles`].
///
/// Mainly used in 2 ways:
///
/// 1. In [`ProfiledCleanerConfig`] to make a [`ProfiledCleaner`].
/// 2. To make just a [`Profiles`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ProfilesConfig {
    /// The base [`ProfileConfig`].
    ///
    /// Defaults to the default [`ProfileConfig`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub base: ProfileConfig,
    /// The [`Profile`]s.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub named: HashMap<String, ProfileConfig>
}

impl ProfilesConfig {
    /// Get the specified [`ProfileConfig`].
    pub fn get<'a>(&'a self, name: Option<&str>) -> Option<&'a ProfileConfig> {
        match name {
            None => Some(&self.base),
            Some(name) => self.named.get(name)
        }
    }

    /// Make only the specified [`Profile`], discarding the rest.
    pub fn into_profile<'a>(mut self, name: Option<&str>, params: Params<'a>) -> Option<Profile<'a>> {
        let ret = self.base.make(params);
        match name {
            None => Some(ret),
            Some(name) => Some(self.named.remove(name)?.make(ret.params))
        }
    }

    /// Make a [`Profiles`] with the provided [`Params`].
    pub fn make<'a>(self, params: Params<'a>) -> Profiles<'a> {
        let base = self.base.make(params);
        Profiles {
            named: self.named.into_iter().map(|(name, profile)| (name, profile.make(base.params.clone()))).collect(),
            base
        }
    }
}
