//! [`CommonArgs`].

use std::collections::{HashSet, HashMap};

use crate::prelude::*;

/// The args a [`Commons`] thing is called with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommonArgs<'a> {
    /// The flags that are set.
    pub flags: &'a HashSet<String>,
    /// The vars that are set.
    pub vars: HashMap<&'a str, String>,
    /// The [`Condition`]s to use.
    pub conditions: &'a HashMap<String, Condition>,
    /// The [`Action`]s to use.
    pub actions: &'a HashMap<String, Action>,
    /// The [`StringSource`]s to use.
    pub string_sources: &'a HashMap<String, StringSource>,
    /// The [`StringModification`]s to use.
    pub string_modifications: &'a HashMap<String, StringModification>,
    /// The [`StringMatcher`]s to use.
    pub string_matchers: &'a HashMap<String, StringMatcher>
}

