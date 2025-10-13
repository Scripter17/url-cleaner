//! [`Profiles`].

use std::collections::BTreeMap;

use crate::types::*;

use serde::Serialize;

/// A default and named [`Profile`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Profiles<'a> {
    /// The base [`Profile`].
    pub(super) base: Profile<'a>,
    /// The named [`Profile`]s.
    pub(super) named: BTreeMap<String, Profile<'a>>
}

impl<'a> Profiles<'a> {
    /// Get an iterator over the names of the [`Profile`]s.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.named.keys().map(|x| &**x)
    }

    /// Get the specified [`Profile`].
    pub fn get_profile(&'a self, name: Option<&str>) -> Option<&'a Profile<'a>> {
        match name {
            None => Some(&self.base),
            Some(name) => self.named.get(name)
        }
    }

    /// [`Self::get_profile`] but consumes `self` and discards everything else.
    pub fn into_profile(mut self, name: Option<&str>) -> Option<Profile<'a>> {
        match name {
            None => Some(self.base),
            Some(name) => self.named.remove(name)
        }
    }

    /// An [`Iterator`] over each [`Profile`] and their names, including the unnamed base [`Profile`].
    ///
    /// Returns [`Profile`]s in order, starting with the base profile.
    pub fn each_profile<'b>(&'b self) -> impl Iterator<Item = (Option<&'b str>, &'b Profile<'a>)> {
        std::iter::once((None, &self.base))
            .chain(self.named.iter().map(|(name, profile)| (Some(&**name), profile)))
    }

    /// An [`Iterator`] over each [`Profile`] and their names, including the unnamed base [`Profile`].
    ///
    /// Returns [`Profile`]s in order, starting with the base profile.
    pub fn into_each_profile(self) -> impl Iterator<Item = (Option<String>, Profile<'a>)> {
        std::iter::once((None, self.base))
            .chain(self.named.into_iter().map(|(name, profile)| (Some(name), profile)))
    }
}
