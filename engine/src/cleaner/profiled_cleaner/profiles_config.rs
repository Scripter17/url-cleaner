//! [`ProfilesConfig`].

use std::collections::BTreeMap;
use std::io;
use std::path::Path;
use std::fs::read_to_string;

use serde::{Serialize, Deserialize};
use thiserror::Error;

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
    /// Defaults to an empty [`BTreeMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub named: BTreeMap<String, ProfileConfig>
}

impl ProfilesConfig {
    /// Load [`Self`] from a JSON file.
    /// # Errors
    #[doc = edoc!(callerr(std::fs::read_to_string), callerr(serde_json::from_str))]
    pub fn load_from_file<T: AsRef<Path>>(path: T) -> Result<ProfilesConfig, GetProfilesConfigError> {
        serde_json::from_str(&read_to_string(path)?).map_err(Into::into)
    }

    /// Make a [`Profiles`] with the provided [`Params`].
    pub fn make<'a>(self, params: Params<'a>) -> Profiles<'a> {
        let base = self.base.make(params);
        Profiles {
            named: self.named.into_iter().map(|(name, profile)| (name, profile.make(base.params.clone()))).collect(),
            base
        }
    }

    /// Get [`Self::base`]
    pub fn get_base(&self) -> &ProfileConfig {
        &self.base
    }

    /// Get [`Self::base`] and discard the rest.
    pub fn into_base(self) -> ProfileConfig {
        self.base
    }

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

    /// Make a borrowing [`Iterator`] of [`ProfileConfig`]s and their names.
    pub fn get_each(&self) -> impl Iterator<Item = (Option<&str>, &ProfileConfig)> {
        std::iter::once((None, &self.base)).chain(self.named.iter().map(|(k, v)| (Some(&**k), v)))
    }

    /// Make a consumeing [`Iterator`] of [`ProfileConfig`]s and their names.
    pub fn into_each(self) -> impl Iterator<Item = (Option<String>, ProfileConfig)> {
        std::iter::once((None, self.base)).chain(self.named.into_iter().map(|(k, v)| (Some(k), v)))
    }

    /// Make a borrowing [`Iterator`] of named [`ProfileConfig`]s and their names.
    pub fn get_each_named(&self) -> impl Iterator<Item = (&str, &ProfileConfig)> {
        self.named.iter().map(|(k, v)| (&**k, v))
    }

    /// Make a consumeing [`Iterator`] of named [`ProfileConfig`]s and their names.
    pub fn into_each_named(self) -> impl Iterator<Item = (String, ProfileConfig)> {
        self.named.into_iter()
    }

    /// [`Self::get_base`] and [`Self::get_each_named`].
    pub fn get_base_and_each_named(&self) -> (&ProfileConfig, impl Iterator<Item = (&str, &ProfileConfig)>) {
        (
            self.get_base(),
            self.get_each_named()
        )
    }

    /// [`Self::into_base`] and [`Self::get_each_named`].
    pub fn into_base_and_each_named(self) -> (ProfileConfig, impl Iterator<Item = (String, ProfileConfig)>) {
        (
            self.base,
            self.named.into_iter()
        )
    }
}

/// The enum of errors that can happen when loading a [`ProfilesConfig`].
#[derive(Debug, Error)]
pub enum GetProfilesConfigError {
    /// Returned when loading a [`ProfilesConfig`] fails.
    #[error(transparent)]
    CantLoadProfilesConfig(#[from] io::Error),
    /// Returned when deserializing a [`ProfilesConfig`] fails.
    #[error(transparent)]
    CantParseProfilesConfig(#[from] serde_json::Error),
}
