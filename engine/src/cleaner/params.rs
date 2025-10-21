//! [`Params`] and [`ParamsDiff`].

use std::collections::{HashMap, HashSet};
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use crate::prelude::*;

/// A data store to fine tune the behavior of [`Cleaner`]s.
///
/// Has two main functions:
///
/// 1. [`Self::flags`] and [`Self::vars`] give easy ways for [`Cleaner`]s to let users enable/change behavior they only sometimes want, such as embed compatibility and unmobiling.
///
/// 2. [`Self::sets`], [`Self::lists`], [`Self::maps`], and [`Self::named_partitionings`] allow giving names to websites, query parameters, etc. that share a common property, such as redirect websites.
///
/// While neither have to be used for those reasons ([`ParamsDiff`]s will happily modify sets and maps), those are what those fields seem to do best.
///
/// There's also two main ways to apply situational/per-[`Job`] changes to [`Params`]:
///
/// 1. [`ProfiledCleanerConfig`] to give commonly used [`ParamsDiff`]s names.
///
/// 2. [`ParamsDiff`] for unnamed one-off changes.
///
/// For both methods, it's recommended to use [`Cleaner::borrowed`]/[`Params::borrowed`] beforehand to allow [`ParamsDiff`] to only clone data it modifies.
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
///
/// // Usually Params are in Cleaners, but this is example code.
///
/// let params = serde_json::from_str::<Params>(r#"
/// {
///     "sets": {
///         "redirect_websites": ["bit.ly", "t.co"]
///     }
/// }
/// "#).unwrap();
///
/// // Makes an equivalent Params where each field is just a reference to the respective field from the above Params.
///
/// let mut borrowed_params = params.borrowed();
///
/// let params_diff = serde_json::from_str::<ParamsDiff>(r#"
/// {
///     "flags": [
///         "This ParamsDiff doesn't touch the Params::sets field at all",
///         "and thus when this is applied to the borrowed Params,",
///         "won't duplicate it, saving some memory.",
///
///         "For simple example code the benefit is smaller than this explanation,",
///         "but for the default cleaner and the 4 profiles I use, borrowing saves 500KB."
///     ]
/// }
/// "#).unwrap();
///
/// params_diff.apply_once(&mut borrowed_params);
/// ```
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
    pub named_partitionings: Cow<'a, HashMap<String, NamedPartitioning>>
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
            named_partitionings: Cow::Borrowed(&*self.named_partitionings)
        }
    }
}

/// Logic for changind a [`Params`].
///
/// If you're frequently using the same few [`ParamsDiff`]s, please check out [`ProfiledCleaner`] for both convenience and speed.
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
    #[serde(default, skip_serializing_if = "is_default")] pub delete_maps: Vec<String>
}

impl ParamsDiff {
    /// Applies each difference, only calling [`Cow::to_mut`] on fields that are actually modified.
    ///
    /// Exact order is not guaranteed to be stable, but currently removals/deletions happen after inittings/insertions/settings.
    ///
    /// If you want to apply `self` multiple times, use [`Self::apply_multiple`] for its slightly better performance in that case.
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
    }

    /// Applies each difference, only calling [`Cow::to_mut`] on fields that are actually modified.
    ///
    /// Exact order is not guaranteed to be stable, but currently removals/deletions happen after inittings/insertions/settings.
    ///
    /// If you only want to apply `self` once, use [`Self::apply_once`] to avoid unnecessary clones.
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
    }
}
