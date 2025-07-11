//! Flags, variables, etc. that adjust the exact behavior of a config.

use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Flags, variables, etc. that adjust the exact behavior of a config.
///
/// Bundles all the state that determines how the [`Cleaner`] works in one convenient area.
#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Suitability)]
pub struct Params {
    /// Flags allow enabling and disabling certain behavior.
    ///
    /// Defaults to an empty [`HashSet`].
    #[serde_with = "SetPreventDuplicates<_>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// Vars allow setting strings used for certain behaviors.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// Sets allow quickly checking if a string is in a certain genre of possible values.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: HashMap<String, Set<String>>,
    /// Lists are a niche thing that lets you iterate over a set of values in a known order.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: HashMap<String, Vec<String>>,
    /// Maps allow mapping input values to output values.
    ///
    /// Please note that [`Map`]s make this more powerful than a normal [`HashMap`], notably including a default value.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: HashMap<String, Map<String>>,
    /// Named partitionings effectively let you check which if several sets a value is in.
    ///
    /// See [this Wikipedia article](https://en.wikipedia.org/wiki/Partition_of_a_set) for the math end of this idea.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub named_partitionings: HashMap<String, NamedPartitioning>,
    /// If [`true`], things that interact with the cache will read from the cache.
    ///
    /// Defaults to true.
    #[cfg(feature = "cache")]
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub read_cache: bool,
    /// If [`true`], things that interact with the cache will write to the cache.
    ///
    /// Defaults to [`true`].
    #[cfg(feature = "cache")]
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub write_cache: bool,
    /// The default [`HttpClientConfig`], prior to relevant [`HttpClientConfigDiff`]s.
    ///
    /// Defaults to [`HttpClientConfig::default`].
    #[cfg(feature = "http")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub http_client_config: HttpClientConfig
}

#[allow(clippy::derivable_impls, reason = "When the `cache` feature is enabled, this can't be derived.")]
impl Default for Params {
    fn default() -> Self {
        Self {
            flags: HashSet::default(),
            vars : HashMap::default(),
            sets : HashMap::default(),
            lists: HashMap::default(),
            maps : HashMap::default(),
            named_partitionings: HashMap::default(),
            #[cfg(feature = "cache")] read_cache: true,
            #[cfg(feature = "cache")] write_cache: true,
            #[cfg(feature = "http")]
            http_client_config: HttpClientConfig::default()
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
    /// If [`Some`], sets, [`Params::read_cache`].
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")] pub read_cache : Option<bool>,
    /// If [`Some`], sets [`Params::write_cache`].
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")] pub write_cache: Option<bool>,
    /// If [`Some`], applies the [`HttpClientConfigDiff`] to [`Params::http_client_config`].
    #[cfg(feature = "http")]
    #[serde(default, skip_serializing_if = "is_default")] pub http_client_config_diff: Option<HttpClientConfigDiff>
}

impl ParamsDiff {
    /// Applies the diff.
    ///
    /// Exact order is not guaranteed to be stable, but currently removals/deletions happen after inittings/insertions/settings.
    ///
    /// If you want to apply `self` mutliple times, use [`Self::apply_multiple`] as it's slightly faster than [`Clone::clone`]ing this then usine [`Self::apply_once`] on each clone.
    pub fn apply_once(self, to: &mut Params) {
        debug!(Params::apply_once, &self, to);
        to.flags.extend(self.flags);
        for flag in self.unflags {to.flags.remove(&flag);}

        to.vars.extend(self.vars);
        for var in self.unvars {to.vars.remove(&var);}

        for k in self.init_sets {
            to.sets.entry(k).or_default();
        }
        for (k, v) in self.insert_into_sets {
            to.sets.entry(k).or_default().extend(v);
        }
        for (k, vs) in self.remove_from_sets {
            if let Some(x) = to.sets.get_mut(&k) {
                for v in vs {
                    x.remove(v.as_ref());
                }
            }
        }
        for k in self.delete_sets {
            to.sets.remove(&k);
        }

        for k in self.init_maps {
            to.maps.entry(k).or_default();
        }
        for (k, v) in self.map_diffs {
            v.apply_once(to.maps.entry(k).or_default());
        }
        for k in self.delete_maps {
            to.maps.remove(&k);
        }

        #[cfg(feature = "cache")] if let Some(read_cache ) = self.read_cache  {to.read_cache  = read_cache ;}
        #[cfg(feature = "cache")] if let Some(write_cache) = self.write_cache {to.write_cache = write_cache;}

        #[cfg(feature = "http")] if let Some(http_client_config_diff) = self.http_client_config_diff {http_client_config_diff.apply_once(&mut to.http_client_config);}
    }

    /// Applies the diff.
    ///
    /// Exact order is not guaranteed to be stable, but currently removals/deletions happen after inittings/insertions/settings.
    ///
    /// If you only want to apply `self` once, use [`Self::apply_once`].
    pub fn apply_multiple(&self, to: &mut Params) {
        debug!(Params::apply_multiple, self, to);
        to.flags.extend(self.flags.iter().cloned());
        for flag in &self.unflags {to.flags.remove(flag);}

        to.vars.extend(self.vars.iter().map(|(k, v)| (k.clone(), v.clone())));
        for var in &self.unvars {to.vars.remove(var);}

        for k in &self.init_sets {
            to.sets.entry(k.clone()).or_default();
        }
        for (k, v) in &self.insert_into_sets {
            to.sets.entry(k.clone()).or_default().extend(v.iter().cloned());
        }
        for (k, vs) in &self.remove_from_sets {
            if let Some(x) = to.sets.get_mut(k) {
                for v in vs {
                    x.remove(v.as_ref());
                }
            }
        }
        for k in &self.delete_sets {
            to.sets.remove(k);
        }

        for k in &self.init_maps {
            to.maps.entry(k.clone()).or_default();
        }
        for (k, v) in &self.map_diffs {
            v.apply_multiple(to.maps.entry(k.clone()).or_default());
        }
        for k in &self.delete_maps {
            to.maps.remove(k);
        }

        #[cfg(feature = "cache")] if let Some(read_cache ) = self.read_cache  {to.read_cache  = read_cache ;}
        #[cfg(feature = "cache")] if let Some(write_cache) = self.write_cache {to.write_cache = write_cache;}

        #[cfg(feature = "http")] if let Some(http_client_config_diff) = &self.http_client_config_diff {http_client_config_diff.apply_multiple(&mut to.http_client_config);}
    }
}
