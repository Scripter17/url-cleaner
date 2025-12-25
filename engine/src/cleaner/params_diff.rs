//! [`ParamsDiff`].

use std::collections::{HashMap, HashSet};
#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::borrow::Cow;
use std::io;
use std::path::Path;
use std::fs::read_to_string;

use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use thiserror::Error;

use crate::prelude::*;

/// Logic for changind a [`Params`].
///
/// If you're frequently using the same few [`ParamsDiff`]s, please check out [`ProfiledCleaner`] for both convenience and speed.
#[serde_as]
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Suitability)]
#[serde(deny_unknown_fields)]
pub struct ParamsDiff {
    /// [`Params::flags`] to enable.
    #[serde_with = "SetPreventDuplicates<_>"]
    #[serde(default, skip_serializing_if = "is_default")] pub flags: HashSet<String>,
    /// [`Params::flags`] to disable.
    #[serde_with = "SetPreventDuplicates<_>"]
    #[serde(default, skip_serializing_if = "is_default")] pub unflags: HashSet<String>,

    /// [`Params::vars`] to set.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub vars: HashMap<String, String>,
    /// [`Params::vars`] to unset.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub unvars: HashSet<String>,

    /// Values to insert into [`Params::sets`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub insert_into_sets: HashMap<String, HashSet<Option<String>>>,
    /// Values to remove from [`Params::sets`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub remove_from_sets: HashMap<String, HashSet<Option<String>>>,

    /// Entries to insert into [`Params::maps`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub insert_into_maps: HashMap<String, HashMap<String, String>>,
    /// Entries to remove from [`Params::maps`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub remove_from_maps: HashMap<String, HashSet<String>>,
    /// Sets [`Params::maps`]'s [`Map::if_none`]s.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub set_map_if_none : HashMap<String, Option<String>>,
    /// Sets [`Params::maps`]'s [`Map::else`]s.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub set_map_else    : HashMap<String, Option<String>>
}

impl ParamsDiff {
    /// Load [`Self`] from a JSON file.
    /// # Errors
    #[doc = edoc!(callerr(std::fs::read_to_string), callerr(serde_json::from_str))]
    pub fn load_from_file<T: AsRef<Path>>(path: T) -> Result<ParamsDiff, GetParamsDiffError> {
        serde_json::from_str(&read_to_string(path)?).map_err(Into::into)
    }

    /// Applies each difference, only calling [`Cow::to_mut`] on fields that are actually modified.
    ///
    /// Exact order is not guaranteed to be stable, but currently removals/deletions happen after inittings/insertions/settings.
    pub fn apply(self, to: &mut Params) {
        if self.is_empty() {
            return;
        }

        // Flags
        if !self.flags.is_empty() {
            to.flags.to_mut().extend(self.flags);
        }
        for flag in self.unflags {to.flags.to_mut().remove(&flag);}

        // Vars
        if !self.vars.is_empty() {
            to.vars.to_mut().extend(self.vars);
        }
        for var in self.unvars {to.vars.to_mut().remove(&var);}

        // Sets
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

        // Maps
        for (k, v) in self.insert_into_maps {
            to.maps.to_mut().entry(k).or_default().map.extend(v);
        }
        for (m, vs) in self.remove_from_maps {
            if let Some(x) = to.maps.to_mut().get_mut(&m) {
                for v in vs {
                    x.map.remove(&v);
                }
            }
        }

        for (k, v) in self.set_map_if_none {
            to.maps.to_mut().entry(k).or_default().if_none = v;
        }
        for (k, v) in self.set_map_else {
            to.maps.to_mut().entry(k).or_default().r#else = v;
        }
    }

    /// Modifies `self` such that [`Self::apply`]ing it effectively also then [`Self::apply`]s `other`.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let mut base = serde_json::from_str::<ParamsDiff>(r#"
    ///     {
    ///         "flags"  : ["a", "b"],
    ///         "unflags": ["c", "d"],
    ///
    ///         "vars": {
    ///             "a": "1",
    ///             "b": "2"
    ///         },
    ///         "unvars": ["c", "d"],
    ///
    ///         "insert_into_sets": {
    ///             "a": ["1", "2"],
    ///             "b": ["1", "2"]
    ///         },
    ///         "remove_from_sets": {
    ///             "c": ["1", "2"],
    ///             "d": ["1", "2"]
    ///         },
    ///
    ///         "insert_into_maps": {
    ///             "a": {
    ///                 "1": "1",
    ///                 "2": "2"
    ///             },
    ///             "b": {
    ///                 "1": "1",
    ///                 "2": "2"
    ///             }
    ///         },
    ///         "remove_from_maps": {
    ///             "c": ["1"],
    ///             "d": ["1"]
    ///         }
    ///     }
    /// "#).unwrap();
    ///
    /// let layer = serde_json::from_str::<ParamsDiff>(r#"
    ///     {
    ///         "flags"  : ["c"],
    ///         "unflags": ["a"],
    ///
    ///         "vars": {
    ///             "c": "3"
    ///         },
    ///         "unvars": ["a"],
    ///
    ///         "insert_into_sets": {
    ///             "c": ["1", "2"]
    ///         },
    ///         "remove_from_sets": {
    ///             "a": ["1"],
    ///             "d": ["1", "2"]
    ///         },
    /// 
    ///         "insert_into_maps": {
    ///             "c": {
    ///                 "1": "1"
    ///             }
    ///         },
    ///         "remove_from_maps": {
    ///             "a": ["1"]
    ///         }
    ///     }
    /// "#).unwrap();
    ///
    /// base.with_then(layer);
    /// base.normalize();
    ///
    /// assert_eq!(base, serde_json::from_str::<ParamsDiff>(r#"
    ///     {
    ///         "flags": ["b", "c"],
    ///         "unflags": ["a", "d"],
    ///
    ///         "vars": {
    ///             "b": "2",
    ///             "c": "3"
    ///         },
    ///         "unvars": ["a", "d"],
    ///
    ///         "insert_into_sets": {
    ///             "a": ["2"],
    ///             "b": ["1", "2"],
    ///             "c": ["1", "2"]
    ///         },
    ///         "remove_from_sets": {
    ///             "a": ["1"],
    ///             "d": ["1", "2"]
    ///         },
    ///
    ///         "insert_into_maps": {
    ///             "a": {
    ///                 "2": "2"
    ///             },
    ///             "b": {
    ///                 "1": "1",
    ///                 "2": "2"
    ///             },
    ///             "c": {
    ///                 "1": "1"
    ///             }
    ///         },
    ///         "remove_from_maps": {
    ///             "a": ["1"],
    ///             "d": ["1"]
    ///         }
    ///     }
    /// "#).unwrap());
    /// ```
    pub fn with_then(&mut self, layer: Self) -> &mut Self {
        if layer.is_empty() {
            return self;
        }

        self.unflags.retain(|flag| !layer.flags.contains(flag));
        self.flags.extend(layer.flags);

        self.flags.retain(|flag| !layer.unflags.contains(flag));
        self.unflags.extend(layer.unflags);



        self.unvars.retain(|var| !layer.vars.contains_key(var));
        self.vars.extend(layer.vars);

        self.vars.retain(|var, _| !layer.unvars.contains(var));
        self.unvars.extend(layer.unvars);



        for (name, set) in layer.insert_into_sets {
            if let Some(x) = self.remove_from_sets.get_mut(&name) {
                x.retain(|element| !set.contains(element));
            }
            self.insert_into_sets.entry(name).or_default().extend(set)
        }


        for (name, set) in layer.remove_from_sets {
            if let Some(x) = self.insert_into_sets.get_mut(&name) {
                x.retain(|element| !set.contains(element));
            }
            self.remove_from_sets.entry(name).or_default().extend(set)
        }



        for (name, map) in layer.insert_into_maps {
            if let Some(x) = self.remove_from_maps.get_mut(&name) {
                x.retain(|k| !map.contains_key(k));
            }
            self.insert_into_maps.entry(name).or_default().extend(map);
        }

        for (name, map) in layer.remove_from_maps {
            if let Some(x) = self.insert_into_maps.get_mut(&name) {
                x.retain(|k, _| !map.contains(k));
            }
            self.remove_from_maps.entry(name).or_default().extend(map);
        }

        self.set_map_if_none.extend(layer.set_map_if_none);
        self.set_map_else   .extend(layer.set_map_else   );

        self
    }

    /// A consuming [`Self::with_then`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let base = serde_json::from_str::<ParamsDiff>(r#"
    ///     {
    ///         "flags"  : ["a", "b"],
    ///         "unflags": ["c", "d"],
    ///
    ///         "vars": {
    ///             "a": "1",
    ///             "b": "2"
    ///         },
    ///         "unvars": ["c", "d"],
    ///
    ///         "insert_into_sets": {
    ///             "a": ["1", "2"],
    ///             "b": ["1", "2"]
    ///         },
    ///         "remove_from_sets": {
    ///             "c": ["1", "2"],
    ///             "d": ["1", "2"]
    ///         },
    ///
    ///         "insert_into_maps": {
    ///             "a": {
    ///                 "1": "1",
    ///                 "2": "2"
    ///             },
    ///             "b": {
    ///                 "1": "1",
    ///                 "2": "2"
    ///             }
    ///         },
    ///         "remove_from_maps": {
    ///             "c": ["1"],
    ///             "d": ["1"]
    ///         }
    ///     }
    /// "#).unwrap();
    ///
    /// let layer = serde_json::from_str::<ParamsDiff>(r#"
    ///     {
    ///         "flags"  : ["c"],
    ///         "unflags": ["a"],
    ///
    ///         "vars": {
    ///             "c": "3"
    ///         },
    ///         "unvars": ["a"],
    ///
    ///         "insert_into_sets": {
    ///             "c": ["1", "2"]
    ///         },
    ///         "remove_from_sets": {
    ///             "a": ["1"],
    ///             "d": ["1", "2"]
    ///         },
    /// 
    ///         "insert_into_maps": {
    ///             "c": {
    ///                 "1": "1"
    ///             }
    ///         },
    ///         "remove_from_maps": {
    ///             "a": ["1"]
    ///         }
    ///     }
    /// "#).unwrap();
    ///
    /// let mut merged = base.then(layer);
    /// merged.normalize();
    ///
    /// assert_eq!(merged, serde_json::from_str::<ParamsDiff>(r#"
    ///     {
    ///         "flags": ["b", "c"],
    ///         "unflags": ["a", "d"],
    ///
    ///         "vars": {
    ///             "b": "2",
    ///             "c": "3"
    ///         },
    ///         "unvars": ["a", "d"],
    ///
    ///         "insert_into_sets": {
    ///             "a": ["2"],
    ///             "b": ["1", "2"],
    ///             "c": ["1", "2"]
    ///         },
    ///         "remove_from_sets": {
    ///             "a": ["1"],
    ///             "d": ["1", "2"]
    ///         },
    ///
    ///         "insert_into_maps": {
    ///             "a": {
    ///                 "2": "2"
    ///             },
    ///             "b": {
    ///                 "1": "1",
    ///                 "2": "2"
    ///             },
    ///             "c": {
    ///                 "1": "1"
    ///             }
    ///         },
    ///         "remove_from_maps": {
    ///             "a": ["1"],
    ///             "d": ["1"]
    ///         }
    ///     }
    /// "#).unwrap());
    /// ```
    pub fn then(mut self, layer: Self) -> Self{
        self.with_then(layer);
        self
    }

    /// Check if `self` is "empty", meaning [`Self::apply`], [`Self::with_then`], and [`Self::then`] have no effect.
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty() &&
            self.unflags.is_empty() &&
            self.vars   .is_empty() &&
            self.unvars .is_empty() &&
            self.insert_into_sets.is_empty() &&
            self.remove_from_sets.is_empty() &&
            self.insert_into_maps.is_empty() &&
            self.remove_from_maps.is_empty() &&
            self.set_map_if_none .is_empty() &&
            self.set_map_else    .is_empty()
    }

    /// Remove empty entries in [`Self::insert_into_sets`], [`Self::remove_from_sets`], [`Self::insert_into_maps`], and [`Self::remove_from_maps`].
    pub fn normalize(&mut self) {
        self.insert_into_sets.retain(|_, x| !x.is_empty());
        self.remove_from_sets.retain(|_, x| !x.is_empty());
        self.insert_into_maps.retain(|_, x| !x.is_empty());
        self.remove_from_maps.retain(|_, x| !x.is_empty());
    }
}

/// The enum of errors that can happen when loading a [`ParamsDiff`].
#[derive(Debug, Error)]
pub enum GetParamsDiffError {
    /// Returned when loading a [`ParamsDiff`] fails.
    #[error(transparent)]
    CantLoadParamsDiff(#[from] io::Error),
    /// Returned when deserializing a [`ParamsDiff`] fails.
    #[error(transparent)]
    CantParseParamsDiff(#[from] serde_json::Error),
}
