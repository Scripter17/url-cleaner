//! [`ProfiledCleaner`].

use crate::types::*;

use serde::Serialize;

/// A [`UnprofiledCleaner`] and [`Profiles`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProfiledCleaner<'a> {
    /// The [`UnprofiledCleaner`].
    pub(super) unprofiled_cleaner: UnprofiledCleaner<'a>,
    /// The [`Profiles`].
    pub(super) profiles: Profiles<'a>
}

impl<'a> ProfiledCleaner<'a> {
    /// Get the [`UnprofiledCleaner`].
    pub fn unprofiled_cleaner(&self) -> &UnprofiledCleaner<'a> {
        &self.unprofiled_cleaner
    }

    /// Get the [`Profiles`].
    pub fn profiles(&self) -> &Profiles<'a> {
        &self.profiles
    }

    /// Make a borrowing [`Cleaner`] with the specified profile.
    pub fn with_profile(&'a self, name: Option<&str>) -> Option<Cleaner<'a>> {
        Some(self.unprofiled_cleaner.borrowed().with_profile(self.profiles.get_profile(name)?.borrowed()))
    }

    /// Make an owning [`Cleaner`] with the specified profile.
    pub fn into_with_profile(self, name: Option<&str>) -> Option<Cleaner<'a>> {
        Some(self.unprofiled_cleaner.with_profile(self.profiles.into_profile(name)?))
    }

    /// A borrowing [`Iterator`] over each [`Profile`]'s [`Cleaner`] and their names, including the unnamed base [`Profile`].
    ///
    /// Returns [`Cleaner`]s in order, starting with the base profile.
    pub fn each_profile(&'a self) -> impl Iterator<Item = (Option<&'a str>, Cleaner<'a>)> {
        self.profiles.each_profile()
            .map(|(name, profile)| (name, self.unprofiled_cleaner.borrowed().with_profile(profile.borrowed())))
    }

    /// A consuming [`Iterator`] over each [`Profile`]'s [`Cleaner`] and their names, including the unnamed base [`Profile`].
    ///
    /// Returns [`Cleaner`]s in order, starting with the base profile.
    pub fn into_each_profile(self) -> impl Iterator<Item = (Option<String>, Cleaner<'a>)> {
        self.profiles.into_each_profile()
            .map(move |(name, profile)| (name, self.unprofiled_cleaner.clone().with_profile(profile)))
    }
}
