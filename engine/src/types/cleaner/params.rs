//! Flags, variables, etc. that adjust the exact behavior of a config.

use std::collections::{HashMap, HashSet};
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use crate::types::*;
use crate::glue::prelude::*;
use crate::util::*;

/// Flags, variables, etc. that adjust the exact behavior of a config.
///
/// Bundles all the state that determines how the [`Cleaner`] works in one convenient area.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Params<'a> {
    /// Flags allow enabling and disabling certain behavior.
    ///
    /// Defaults to an empty [`HashSet`].
    #[serde_with = "SetPreventDuplicates<_>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: Cow<'a, HashSet<String>>,
    /// Vars allow setting strings used for certain behaviors.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: Cow<'a, HashMap<String, String>>,
    /// Sets allow quickly checking if a string is in a certain genre of possible values.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: Cow<'a, HashMap<String, Set<String>>>,
    /// Lists are a niche thing that lets you iterate over a set of values in a known order.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: Cow<'a, HashMap<String, Vec<String>>>,
    /// Maps allow mapping input values to output values.
    ///
    /// Please note that [`Map`]s make this more powerful than a normal [`HashMap`], notably including a default value.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: Cow<'a, HashMap<String, Map<String>>>,
    /// Named partitionings effectively let you check which if several sets a value is in.
    ///
    /// See [this Wikipedia article](https://en.wikipedia.org/wiki/Partition_of_a_set) for the math end of this idea.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub named_partitionings: Cow<'a, HashMap<String, NamedPartitioning>>,
    /// The default [`HttpClientConfig`], prior to relevant [`HttpClientConfigDiff`]s.
    ///
    /// Defaults to [`HttpClientConfig::default`].
    #[cfg(feature = "http")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub http_client_config: Cow<'a, HttpClientConfig>
}

impl<'a> Params<'a> {
    /// Create a new [`Self`] that [`Cow::Borrowed`]s all fields.
    ///
    /// Basically a very cheap [`Clone`] that you can apply [`ParamsDiff`]s to.
    pub fn borrowed(&'a self) -> Self {
        Self {
            flags              : Cow::Borrowed(&*self.flags),
            vars               : Cow::Borrowed(&*self.vars),
            sets               : Cow::Borrowed(&*self.sets),
            lists              : Cow::Borrowed(&*self.lists),
            maps               : Cow::Borrowed(&*self.maps),
            named_partitionings: Cow::Borrowed(&*self.named_partitionings),
            #[cfg(feature = "http")]
            http_client_config : Cow::Borrowed(&*self.http_client_config)
        }
    }
}

/// Rules for updating a [`Params`].
///
/// Often you'll have a default [`ParamsDiff`] you use for all your URLs and only sometimes you want to change that behavior.
///
/// The diff pattern handles this use case very well without requiring you change the actual config file.
#[serde_as]
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ParamsDiff {
    /// [`Params::flags`] to set.
    #[serde_with = "SetPreventDuplicates<_>"]
    #[serde(default, skip_serializing_if = "is_default")] pub flags  : HashSet<String>,
    /// [`Params::flags`] to unset.
    #[serde_with = "SetPreventDuplicates<_>"]
    #[serde(default, skip_serializing_if = "is_default")] pub unflags: HashSet<String>,
    /// [`Params::vars`] to set.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub vars  : HashMap<String, String>,
    /// [`Params::vars`] to unset.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub unvars: HashSet<String>,
    /// [`Params::sets`] to init.
    ///
    /// Shouldn't ever actually change anything, but if you're really fussy or something.
    #[serde(default, skip_serializing_if = "is_default")] pub init_sets: Vec<String>,
    /// [`Params::sets`] and values to insert into them.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub insert_into_sets: HashMap<String, Vec<Option<String>>>,
    /// [`Params::sets`] and values to remove from them.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub remove_from_sets: HashMap<String, Vec<Option<String>>>,
    /// [`Params::sets`] to delete.
    #[serde(default, skip_serializing_if = "is_default")] pub delete_sets: Vec<String>,
    /// [`Params::maps`] to init.
    ///
    /// Shouldn't ever actually change anything, but if you're really fussy or something.
    #[serde(default, skip_serializing_if = "is_default")] pub init_maps: Vec<String>,
    /// [`MapDiff`]s to apply to [`Params::maps`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub map_diffs: HashMap<String, MapDiff<String>>,
    /// [`Params::maps`] to delete.
    #[serde(default, skip_serializing_if = "is_default")] pub delete_maps: Vec<String>,
    /// If [`Some`], applies the [`HttpClientConfigDiff`] to [`Params::http_client_config`].
    #[cfg(feature = "http")]
    #[serde(default, skip_serializing_if = "is_default")] pub http_client_config_diff: Option<HttpClientConfigDiff>
}

impl ParamsDiff {
    /// Applies the diff.
    ///
    /// Exact order is not guaranteed to be stable, but currently removals/deletions happen after inittings/insertions/settings.
    ///
    /// If you want to apply `self` multiple times, use [`Self::apply_multiple`] as it's slightly faster than [`Clone::clone`]ing this then using [`Self::apply_once`] on each clone.
    pub fn apply_once(self, to: &mut Params) {
        debug!(Params::apply_once, &self, to);
        to.flags.to_mut().extend(self.flags);
        for flag in self.unflags {to.flags.to_mut().remove(&flag);}

        to.vars.to_mut().extend(self.vars);
        for var in self.unvars {to.vars.to_mut().remove(&var);}

        for k in self.init_sets {
            to.sets.to_mut().entry(k).or_default();
        }
        for (k, v) in self.insert_into_sets {
            to.sets.to_mut().entry(k).or_default().extend(v);
        }
        for (k, vs) in self.remove_from_sets {
            if let Some(x) = to.sets.to_mut().get_mut(&k) {
                for v in vs {
                    x.remove(v.as_ref());
                }
            }
        }
        for k in self.delete_sets {
            to.sets.to_mut().remove(&k);
        }

        for k in self.init_maps {
            to.maps.to_mut().entry(k).or_default();
        }
        for (k, v) in self.map_diffs {
            v.apply_once(to.maps.to_mut().entry(k).or_default());
        }
        for k in self.delete_maps {
            to.maps.to_mut().remove(&k);
        }

        #[cfg(feature = "http")] if let Some(http_client_config_diff) = self.http_client_config_diff {http_client_config_diff.apply_once(to.http_client_config.to_mut());}
    }

    /// Applies the diff.
    ///
    /// Exact order is not guaranteed to be stable, but currently removals/deletions happen after inittings/insertions/settings.
    ///
    /// If you only want to apply `self` once, use [`Self::apply_once`].
    pub fn apply_multiple(&self, to: &mut Params) {
        debug!(Params::apply_multiple, self, to);
        to.flags.to_mut().extend(self.flags.iter().cloned());
        for flag in &self.unflags {to.flags.to_mut().remove(flag);}

        to.vars.to_mut().extend(self.vars.iter().map(|(k, v)| (k.clone(), v.clone())));
        for var in &self.unvars {to.vars.to_mut().remove(var);}

        for k in &self.init_sets {
            to.sets.to_mut().entry(k.clone()).or_default();
        }
        for (k, v) in &self.insert_into_sets {
            to.sets.to_mut().entry(k.clone()).or_default().extend(v.iter().cloned());
        }
        for (k, vs) in &self.remove_from_sets {
            if let Some(x) = to.sets.to_mut().get_mut(k) {
                for v in vs {
                    x.remove(v.as_ref());
                }
            }
        }
        for k in &self.delete_sets {
            to.sets.to_mut().remove(k);
        }

        for k in &self.init_maps {
            to.maps.to_mut().entry(k.clone()).or_default();
        }
        for (k, v) in &self.map_diffs {
            v.apply_multiple(to.maps.to_mut().entry(k.clone()).or_default());
        }
        for k in &self.delete_maps {
            to.maps.to_mut().remove(k);
        }

        #[cfg(feature = "http")] if let Some(http_client_config_diff) = &self.http_client_config_diff {http_client_config_diff.apply_multiple(to.http_client_config.to_mut());}
    }
}
