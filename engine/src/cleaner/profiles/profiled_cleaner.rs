//! [`ProfiledCleaner`].

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// A [`Cleaner`] with named and pre-computed [`ParamsDiff`]s for cheap and easy use.
///
/// Usually made via [`ProfiledCleanerConfig`].
///
/// To reduce memory usage, put the [`Cleaner`] through [`Box::new`], [`Box::leak`], [`Cleaner::borrowed`], and only then [`ProfiledCleanerConfig`].
///
/// This lets each [`ParamsDiff`] clone only the parts of the [`Params`] they actually modify while keeping the `'static` lifetime.
///
/// On my personal instance of URL Cleaner Site with the 4 profiles I use, this saves 500KB of memory, which for a program that rarely hits 10MB is pretty big.
/// # Examples
/// ```
/// use std::borrow::Cow;
/// use url_cleaner_engine::prelude::*;
///
/// // Load the Cleaner, unbothered by its eventual profiling.
///
/// let cleaner = serde_json::from_str::<Cleaner>(r#"
/// {
///     "actions": [
///         {"If": {
///             "if": {"All": [
///                 {"FlagIsSet": "embed_compatibility"},
///                 {"NormalizedHostIs": "x.com"}
///             ]},
///             "then": {"SetHost": "vxtwitter.com"}
///         }}
///     ]
/// }
/// "#).unwrap();
///
/// // Optional but can dramatically reduce memory usage.
///
/// let cleaner = Box::leak(Box::new(cleaner)).borrowed();
///
/// // Load the ProfilesConfig, usually from a file.
///
/// let profiles_config = serde_json::from_str::<ProfilesConfig>(r#"
/// {
///     "base": {
///         "params_diff": {
///             "flags": [
///                 "Each named Profile adds their ParamsDiffs on top of this one.",
///                 "So the \"Embed compatibility\" profile will also have these three flags.",
///                 "The entire \"base\" field can be omitted if empty."
///             ]
///         }
///     },
///     "named": {
///         "Embed compatibility": {
///             "params_diff": {
///                 "flags": ["embed_compatibility"]
///             }
///         }
///     }
/// }
/// "#).unwrap();
///
/// // Make the ProfiledCleaner.
///
/// let profiled_cleaner = ProfiledCleanerConfig {cleaner, profiles_config}.make();
///
/// // Test the base profile.
///
/// assert_eq!(
///     task!(
///         "https://x.com/user/status/1",
///         cleaner = &profiled_cleaner.get(None).unwrap()
///     ).r#do().unwrap(),
///     "https://x.com/user/status/1"
/// );
///
/// // Test the "Embed compatibility" profile.
///
/// assert_eq!(
///     task!(
///         "https://x.com/user/status/1",
///         cleaner = &profiled_cleaner.get(Some("Embed compatibility")).unwrap()
///     ).r#do().unwrap(),
///     "https://vxtwitter.com/user/status/1"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfiledCleaner<'a> {
    /// The [`UnprofiledCleaner`].
    pub unprofiled_cleaner: UnprofiledCleaner<'a>,
    /// The [`Profiles`].
    pub profiles: Profiles<'a>
}

impl<'a> ProfiledCleaner<'a> {
    /// Make a borrowing [`Cleaner`] with the base profile.
    pub fn get_base(&'a self) -> Cleaner<'a> {
        self.unprofiled_cleaner.borrowed().with_profile(self.profiles.get_base())
    }

    /// Make a consuming [`Cleaner`] with the base profile.
    pub fn into_base(self) -> Cleaner<'a> {
        self.unprofiled_cleaner.with_profile(self.profiles.into_base())
    }

    /// Make a borrowing [`Cleaner`] with the specified profile.
    pub fn get(&'a self, name: Option<&str>) -> Option<Cleaner<'a>> {
        Some(self.unprofiled_cleaner.borrowed().with_profile(self.profiles.get(name)?))
    }

    /// Make a consuming [`Cleaner`] with the specified profile.
    pub fn into_profile(self, name: Option<&str>) -> Option<Cleaner<'a>> {
        Some(self.unprofiled_cleaner.with_profile(self.profiles.into_profile(name)?))
    }

    /// Make each borrowing [`Cleaner`], alphabetically starting with the base.
    pub fn get_each(&'a self) -> impl Iterator<Item = (Option<&'a str>, Cleaner<'a>)> {
        self.profiles.get_each()
            .map(|(name, profile)| (name, self.unprofiled_cleaner.borrowed().with_profile(profile)))
    }

    /// Make each consuming [`Cleaner`], alphabetically starting with the base.
    pub fn into_each(self) -> impl Iterator<Item = (Option<String>, Cleaner<'a>)> {
        self.profiles.into_each()
            .map(move |(name, profile)| (name, self.unprofiled_cleaner.clone().with_profile(profile)))
    }

    /// Make each borrowing [`Cleaner`], alphabetically excluding the base.
    pub fn get_each_named(&'a self) -> impl Iterator<Item = (&'a str, Cleaner<'a>)> {
        self.profiles.get_each_named().map(|(name, profile)| (name, self.unprofiled_cleaner.borrowed().with_profile(profile)))
    }

    /// Make each consuming [`Cleaner`], alphabetically excluding the base.
    pub fn into_each_named(self) -> impl Iterator<Item = (String, Cleaner<'a>)> {
        self.profiles.into_each_named().map(move |(name, profile)| (name, self.unprofiled_cleaner.clone().with_profile(profile)))
    }

    /// [`Self::get_base`] and [`Self::get_each_named`].
    pub fn get_base_and_each_named(&'a self) -> (Cleaner<'a>, impl Iterator<Item = (&'a str, Cleaner<'a>)>) {
        (
            self.get_base(),
            self.get_each_named()
        )
    }

    /// [`Self::into_base`] and [`Self::get_each_named`].
    pub fn into_base_and_each_named(self) -> (Cleaner<'a>, impl Iterator<Item = (String, Cleaner<'a>)>) {
        (
            self.unprofiled_cleaner.clone().with_profile(self.profiles.base),
            self.profiles.named.into_iter().map(move |(name, profile)| (name, self.unprofiled_cleaner.clone().with_profile(profile)))
        )
    }
}
