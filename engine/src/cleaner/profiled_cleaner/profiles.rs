//! [`Profiles`].

use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// A base [`Profile`] and named [`Profile`]s.
///
/// Usually made from [`ProfilesConfig`].
///
/// Mainly used in [`ProfiledCleaner`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profiles<'a> {
    /// The base [`Profile`].
    pub base: Profile<'a>,
    /// The named [`Profile`]s.
    pub named: BTreeMap<String, Profile<'a>>
}

impl<'a> Profiles<'a> {
    /// Make a borrowing [`Profile`] with the base profile.
    pub fn get_base(&'a self) -> Profile<'a> {
        self.base.borrowed()
    }

    /// Make a consuming [`Profile`] with the base profile.
    pub fn into_base(self) -> Profile<'a> {
        self.base
    }

    /// Make a borrowing [`Profile`] with the specified profile.
    pub fn get(&'a self, name: Option<&str>) -> Option<Profile<'a>> {
        match name {
            None       => Some(self.base.borrowed()),
            Some(name) => self.named.get(name).map(Profile::borrowed)
        }
    }

    /// Make a consuming [`Profile`] with the specified profile.
    pub fn into_profile(mut self, name: Option<&str>) -> Option<Profile<'a>> {
        match name {
            None => Some(self.base),
            Some(name) => self.named.remove(name)
        }
    }

    /// Make each borrowing [`Profile`], alphabetically starting with the base.
    pub fn get_each(&'a self) -> impl Iterator<Item = (Option<&'a str>, Profile<'a>)> {
        std::iter::once((None, self.get_base()))
            .chain(self.named.iter().map(|(name, profile)| (Some(&**name), profile.borrowed())))
    }

    /// Make each consuming [`Profile`], alphabetically starting with the base.
    pub fn into_each(self) -> impl Iterator<Item = (Option<String>, Profile<'a>)> {
        std::iter::once((None, self.base))
            .chain(self.named.into_iter().map(|(name, profile)| (Some(name), profile)))
    }

    /// Make each borrowing [`Profile`], alphabetically excluding the base.
    pub fn get_each_named(&'a self) -> impl Iterator<Item = (&'a str, Profile<'a>)> {
        self.named.iter().map(|(name, profile)| (&**name, profile.borrowed()))
    }

    /// Make each consuming [`Profile`], alphabetically excluding the base.
    pub fn into_each_named(self) -> impl Iterator<Item = (String, Profile<'a>)> {
        self.named.into_iter()
    }

    /// [`Self::get_base`] and [`Self::get_each_named`].
    pub fn get_base_and_each_named(&'a self) -> (Profile<'a>, impl Iterator<Item = (&'a str, Profile<'a>)>) {
        (
            self.get_base(),
            self.get_each_named()
        )
    }

    /// [`Self::into_base`] and [`Self::get_each_named`].
    pub fn into_base_and_each_named(self) -> (Profile<'a>, impl Iterator<Item = (String, Profile<'a>)>) {
        (
            self.base,
            self.named.into_iter()
        )
    }
}
