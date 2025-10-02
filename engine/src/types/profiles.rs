//! [`Profiles`] allow for cheaply switching between common [`ParamsDiff`]s.

use std::collections::HashMap;
use std::borrow::Cow;

use crate::types::*;
use crate::util::*;

use serde::{Serialize, Deserialize};

impl<'a> Cleaner<'a> {
    /// Convert the [`Cleaner`] into a [`ProfiledCleaner`] using the specified [`ProfilesConfig`].
    pub fn with_profiles(self, profiles: ProfilesConfig) -> ProfiledCleaner<'a> {
        ProfiledCleaner {
            profiles: profiles.make(self.params),
            cleaner: UnprofiledCleaner {
                docs   : self.docs,
                commons: self.commons,
                actions: self.actions
            }
        }
    }
}

/// A [`UnprofiledCleaner`] and [`Profiles`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProfiledCleaner<'a> {
    /// The [`UnprofiledCleaner`].
    cleaner: UnprofiledCleaner<'a>,
    /// The [`Profiles`].
    profiles: Profiles<'a>
}

impl<'a> ProfiledCleaner<'a> {
    /// Reconstructs a cheap copy of the original [`Cleaner`].
    pub fn original(&'a self) -> Cleaner<'a> {
        Cleaner {
            docs   : Cow::Borrowed(&*self.cleaner.docs),
            params : self.profiles().original_params().borrowed(),
            commons: Cow::Borrowed(&*self.cleaner.commons),
            actions: Cow::Borrowed(&*self.cleaner.actions)
        }
    }
    
    /// Get the [`UnprofiledCleaner`].
    pub fn unprofiled(&'a self) -> UnprofiledCleaner<'a> {
        self.cleaner.borrowed()
    }

    /// Get the [`Profiles`].
    pub fn profiles(&self) -> &Profiles<'a> {
        &self.profiles
    }

    /// Make a [`Cleaner`] borrowing each field of `self` and using the specified profile.
    pub fn profile(&'a self, name: Option<&str>) -> Option<Cleaner<'a>> {
        Some(Cleaner {
            docs   : Cow::Borrowed(&*self.cleaner.docs),
            params : self.profiles.get(name)?.params().borrowed(),
            commons: Cow::Borrowed(&*self.cleaner.commons),
            actions: Cow::Borrowed(&*self.cleaner.actions)
        })
    }
}

/// A [`Cleaner`] with no [`Params`], for use with [`Profiles`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct UnprofiledCleaner<'a> {
    /// The documentation.
    ///
    /// Defaults to an empty [`CleanerDocs`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: Cow<'a, CleanerDocs>,
    /// Basically functions.
    ///
    /// Defaults to an empty [`Commons`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub commons: Cow<'a, Commons>,
    /// The [`Action`]s to apply.
    ///
    /// Defaults to an empty [`Vec`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub actions: Cow<'a, [Action]>
}

impl<'a> UnprofiledCleaner<'a> {
    /// Create a new [`Self`] that [`Cow::Borrowed`]s all fields.
    pub fn borrowed(&'a self) -> Self {
        Self {
            docs   : Cow::Borrowed(&*self.docs),
            commons: Cow::Borrowed(&*self.commons),
            actions: Cow::Borrowed(&*self.actions)
        }
    }

    /// Make a [`Cleaner`] borrowing each field of `self` and using the specified [`Params`].
    pub fn with_profile(&'a self, profile: &'a Profile) -> Cleaner<'a> {
        Cleaner {
            docs   : Cow::Borrowed(&*self.docs),
            params : profile.params.borrowed(),
            commons: Cow::Borrowed(&*self.commons),
            actions: Cow::Borrowed(&*self.actions)
        }
    }
}

/// A default and named [`Profile`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Profiles<'a> {
    /// The original [`Params`].
    original: Params<'a>,
    /// The base [`Profile`].
    base: Profile<'a>,
    /// The named [`Profile`]s.
    named: HashMap<String, Profile<'a>>
}

impl<'a> Profiles<'a> {
    /// Get an iterator over the names of the [`Profile`]s.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.named.keys().map(String::as_str)
    }

    /// Returns the original [`Params`] used.
    pub fn original_params(&self) -> &Params<'a> {
        &self.original
    }

    /// Get the specified [`Profile`].
    pub fn get(&'a self, name: Option<&str>) -> Option<&'a Profile<'a>> {
        match name {
            None => Some(&self.base),
            Some(name) => self.named.get(name)
        }
    }

    /// [`Self::get`] but consumes `self` and discards everything else.
    pub fn into_profile(mut self, name: Option<&str>) -> Option<Profile<'a>> {
        match name {
            None => Some(self.base),
            Some(name) => self.named.remove(name)
        }
    }
}

/// A [`ParamsDiff`] profile.
///
/// Constructed by giving [`ProfilesConfig::make`] a [`Params`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Profile<'a> {
    /// The [`ParamsDiff`].
    params_diff: ParamsDiff,
    /// The [`Params`].
    #[serde(skip)]
    params: Params<'a>
}

impl<'a> Profile<'a> {
    /// Get the [`Params`].
    pub fn params(&self) -> &Params<'a> {
        &self.params
    }

    /// Get the [`ParamsDiff`].
    pub fn params_diff(&self) -> &ParamsDiff {
        &self.params_diff
    }
}

impl From<Profile<'_>> for ParamsDiff {
    fn from(value: Profile) -> Self {
        value.params_diff
    }
}

impl<'a> From<Profile<'a>> for Params<'a> {
    fn from(value: Profile<'a>) -> Self {
        value.params
    }
}

/// A [`Cleaner`] and a [`ProfilesConfig`] to make a [`ProfiledCleaner`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfiledCleanerConfig<'a> {
    /// The [`Cleaner`] to use.
    pub cleaner: Cleaner<'a>,
    /// The [`ProfilesConfig`] to use.
    pub profiles: ProfilesConfig
}

impl<'a> ProfiledCleanerConfig<'a> {
    /// Make the [`ProfiledCleaner`].
    pub fn make(self) -> ProfiledCleaner<'a> {
        ProfiledCleaner {
            profiles: self.profiles.make(self.cleaner.params),
            cleaner: UnprofiledCleaner {
                docs   : self.cleaner.docs,
                commons: self.cleaner.commons,
                actions: self.cleaner.actions
            }
        }
    }
}

/// The configuration for a [`Profile`]s.
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

    /// Create and return only the [`Params`] for the specified profile.
    pub fn into_params<'a>(mut self, name: Option<&str>, mut params: Params<'a>) -> Option<Params<'a>> {
        self.base.params_diff.apply_once(&mut params);
        match name {
            None => Some(params),
            Some(name) => {
                self.named.remove(name)?.params_diff.apply_once(&mut params);
                Some(params)
            }
        }
    }

    /// Make a [`Profiles`] with the provided [`Params`].
    pub fn make(self, params: Params) -> Profiles {
        let original = params.clone();
        let base = self.base.make(params);
        Profiles {
            original,
            named: self.named.into_iter().map(|(name, profile)| (name, profile.make(base.params().clone()))).collect(),
            base
        }
    }
}

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
    pub fn make(self, mut params: Params) -> Profile {
        Profile {
            params: {self.params_diff.apply_multiple(&mut params); params},
            params_diff: self.params_diff
        }
    }
}
