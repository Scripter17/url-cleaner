//! [`ParamsDiff`].

use std::collections::{HashMap, HashSet};
#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use crate::prelude::*;

/// Logic for changind a [`Params`].
///
/// If you're frequently using the same few [`ParamsDiff`]s, please check out [`ProfiledCleaner`] for both convenience and speed.
#[serde_as]
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
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
    #[serde(default, skip_serializing_if = "is_default")] pub insert_into_sets: HashMap<String, Vec<Option<String>>>,
    /// Values to remove from [`Params::sets`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub remove_from_sets: HashMap<String, Vec<Option<String>>>,

    /// Entries to insert into [`Params::maps`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub insert_into_maps: HashMap<String, HashMap<Option<String>, String>>,
    /// [`Map::else`]s to set for [`Params::maps`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub set_map_elses: HashMap<String, Option<String>>,
    /// Entries to remove from [`Params::maps`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub remove_from_maps: HashMap<String, HashSet<Option<String>>>,
}

impl ParamsDiff {
    /// Applies each difference, only calling [`Cow::to_mut`] on fields that are actually modified.
    ///
    /// Exact order is not guaranteed to be stable, but currently removals/deletions happen after inittings/insertions/settings.
    pub fn apply(self, to: &mut Params) {
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
        for (m, kv) in self.insert_into_maps {
            to.maps.to_mut().entry(m).or_default().extend(kv);
        }
        for (m, v) in self.set_map_elses {
            to.maps.to_mut().entry(m).or_default().r#else = v;
        }
        for (m, vs) in self.remove_from_maps {
            if let Some(x) = to.maps.to_mut().get_mut(&m) {
                for v in vs {
                    x.remove(v.as_ref());
                }
            }
        }
    }
}
