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
            profiles: profiles.make(self.params.into_owned()),
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
    profiles: Profiles
}

impl<'a> ProfiledCleaner<'a> {
    /// Get the [`UnprofiledCleaner`].
    pub fn cleaner(&self) -> &UnprofiledCleaner<'a> {
        &self.cleaner
    }

    /// Get the [`Profiles`].
    pub fn profiles(&self) -> &Profiles {
        &self.profiles
    }

    /// Make a [`Cleaner`] borrowing each field of `self` and using the specified profile.
    pub fn with_profile(&'a self, name: Option<&str>) -> Option<Cleaner<'a>> {
        Some(Cleaner {
            docs   : Cow::Borrowed(&*self.cleaner.docs),
            params : Cow::Borrowed(self.profiles.get(name)?.params()),
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
            params : Cow::Borrowed(&profile.params),
            commons: Cow::Borrowed(&*self.commons),
            actions: Cow::Borrowed(&*self.actions)
        }
    }
}

/// A default [`ParamsDiff`] and [`Profiles`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Profiles {
    /// The default [`Profile`].
    #[serde(flatten)]
    default: Profile,
    /// The [`Profile`]s.
    profiles: HashMap<String, Profile>
}

impl Profiles {
    /// Get an iterator over the names of the [`Profile`]s.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.profiles.keys().map(String::as_str)
    }

    /// Get the specified [`Profile`].
    pub fn get_profile<'a>(&'a self, name: Option<&str>) -> Option<&'a Profile> {
        match name {
            None => Some(&self.default),
            Some(name) => self.profiles.get(name)
        }
    }

    /// [`Self::get_profile`] but consumes `self` and discards everything else.
    pub fn into_profile(mut self, name: Option<&str>) -> Option<Profile> {
        match name {
            None => Some(self.default),
            Some(name) => self.profiles.remove(name)
        }
    }
}

impl Profiles {
    /// Get the [`HashMap`] of [`Profile`]s.
    /// Get a [`Profile`] by name.
    pub fn get<'a>(&'a self, name: Option<&str>) -> Option<&'a Profile> {
        match name {
            None => Some(&self.default),
            Some(name) => self.profiles.get(name)
        }
    }
}

/// A [`ParamsDiff`] profile.
///
/// Constructed by giving [`ProfilesConfig::make`] a [`Params`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Profile {
    /// The [`ParamsDiff`].
    params_diff: ParamsDiff,
    /// The [`Params`].
    #[serde(skip)]
    params: Params
}

impl Profile {
    /// Get the [`Params`].
    pub fn params(&self) -> &Params {
        &self.params
    }

    /// Get the [`ParamsDiff`].
    pub fn params_diff(&self) -> &ParamsDiff {
        &self.params_diff
    }
}

impl From<Profile> for ParamsDiff {
    fn from(value: Profile) -> Self {
        value.params_diff
    }
}

impl From<Profile> for Params {
    fn from(value: Profile) -> Self {
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
            profiles: self.profiles.make(self.cleaner.params.into_owned()),
            cleaner: UnprofiledCleaner {
                docs   : self.cleaner.docs,
                commons: self.cleaner.commons,
                actions: self.cleaner.actions
            }
        }
    }
}

/// A default [`ProfileConfig`] and [`ProfileConfig`]s to apply on top of [`Self::default`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ProfilesConfig {
    /// The default [`ParamsDiff`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub default: ProfileConfig,
    /// The [`Profile`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub profiles: HashMap<String, ProfileConfig>
}

impl ProfilesConfig {
    /// Get the specified [`ProfileConfig`].
    pub fn get_profile_config<'a>(&'a self, name: Option<&str>) -> Option<&'a ProfileConfig> {
        match name {
            None => Some(&self.default),
            Some(name) => self.profiles.get(name)
        }
    }

    /// Create and return only the [`Params`] for the specified profile.
    pub fn into_params(mut self, name: Option<&str>, mut params: Params) -> Option<Params> {
        self.default.params_diff.apply_once(&mut params);
        match name {
            None => Some(params),
            Some(name) => {
                self.profiles.remove(name)?.params_diff.apply_once(&mut params);
                Some(params)
            }
        }
    }
    
    /// Make a [`Profiles`] with the provided [`Params`].
    pub fn make(self, params: Params) -> Profiles {
        let default = self.default.make(params);
        Profiles {
            profiles: self.profiles.into_iter().map(|(name, profile)| (name, profile.make(default.params().clone()))).collect(),
            default
        }
    }
}

/// A [`ParamsDiff`] profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ProfileConfig {
    /// The [`ParamsDiff`].
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
