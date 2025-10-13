//! [`ProfiledCleanerConfig`].

use crate::types::*;

use serde::{Serialize, Deserialize};

/// A [`Cleaner`] and a [`ProfilesConfig`] to make a [`ProfiledCleaner`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfiledCleanerConfig<'a> {
    /// The [`Cleaner`] to use.
    pub cleaner: Cleaner<'a>,
    /// The [`ProfilesConfig`] to use.
    pub profiles_config: ProfilesConfig
}

impl<'a> ProfiledCleanerConfig<'a> {
    /// Make the [`ProfiledCleaner`].
    pub fn make(self) -> ProfiledCleaner<'a> {
        ProfiledCleaner {
            unprofiled_cleaner: UnprofiledCleaner {
                docs   : self.cleaner.docs,
                commons: self.cleaner.commons,
                actions: self.cleaner.actions
            },
            profiles: self.profiles_config.make(self.cleaner.params)
        }
    }

    /// Make only the [`Cleaner`] for the specirfied profile.
    pub fn into_profile(self, name: Option<&str>) -> Option<Cleaner<'a>> {
        Some(Cleaner {
            docs   : self.cleaner.docs,
            params : self.profiles_config.into_params(name, self.cleaner.params)?,
            commons: self.cleaner.commons,
            actions: self.cleaner.actions
        })
    }
}
