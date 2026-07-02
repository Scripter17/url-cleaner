//! [`ParamsDiff`].

use std::path::Path;
use std::fs::read_to_string;

use crate::prelude::*;

/// A diff of a [`Params`] for things like enabling non-default features.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ParamsDiff {
    /// [`Params::flags`] to enable.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// [`Params::flags`] to disable.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub unflags: HashSet<String>,

    /// [`Params::vars`] to set.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// [`Params::vars`] to unset.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub unvars: HashSet<String>,

    /// Values to insert into [`Params::sets`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub insert_into_sets: HashMap<String, Set<String>>,
    /// Values to remove from [`Params::sets`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub remove_from_sets: HashMap<String, Set<String>>,

    /// Entries to insert into [`Params::maps`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub insert_into_maps: HashMap<String, Map<String>>,
    /// Entries to remove from [`Params::maps`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub remove_from_maps: HashMap<String, Set<String>>,
}

impl ParamsDiff {
    /// Load a [`Self`] from a JSON file.
    /// # Errors
    /// If the call to [`read_to_string`] returns an error, that error is returned.
    ///
    /// If the call to [`serde_json::from_str`] returns an error, that error is returned.
    pub fn load<T: AsRef<Path>>(path: T) -> Result<(String, ParamsDiff), LoadParamsDiffError> {
        let string = read_to_string(path)?;
        let params_diff = serde_json::from_str(&string)?;
        Ok((string, params_diff))
    }

    /// [`Self::load`] or [`Self::default`].
    /// # Errors
    /// If the call to [`Self::load`] returns an error, that error is returned.
    pub fn load_or_default<T: AsRef<Path>>(path: Option<T>) -> Result<(Cow<'static, str>, ParamsDiff), LoadParamsDiffError> {
        match path {
            Some(path) => {
                let (x, y) = Self::load(path)?;
                Ok((x.into(), y))
            },
            None => Ok(("{}".into(), Default::default()))
        }
    }

    /// Check if `self` is "empty", meaning [`Self::apply`] has no effect.
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty()
            && self.unflags.is_empty()
            && self.vars   .is_empty()
            && self.unvars .is_empty()
            && self.insert_into_sets.is_empty()
            && self.remove_from_sets.is_empty()
            && self.insert_into_maps.is_empty()
            && self.remove_from_maps.is_empty()
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
        if !self.unflags.is_empty() {
            to.flags.to_mut().retain(|x| !self.unflags.contains(x));
        }

        // Vars
        if !self.vars.is_empty() {
            to.vars.to_mut().extend(self.vars);
        }
        if !self.unvars.is_empty() {
            to.vars.to_mut().retain(|k, _| !self.unvars.contains(k))
        }

        // Sets
        for (n, i) in self.insert_into_sets {
            to.sets.to_mut().entry(n).or_default().extend(i);
        }
        for (n, r) in self.remove_from_sets {
            if to.sets.contains_key(&n) && let Some(s) = to.sets.to_mut().get_mut(&n) {
                s.retain(|x| !r.contains(x));
            }
        }

        // Maps
        for (n, i) in self.insert_into_maps {
            to.maps.to_mut().entry(n).or_default().extend(i);
        }
        for (n, r) in self.remove_from_maps {
            if to.maps.contains_key(&n) && let Some(m) = to.maps.to_mut().get_mut(&n) {
                m.retain(|k, _| !r.contains(k));
            }
        }
    }
}

impl Suitability for ParamsDiff {
    fn assert_suitability(&self, cleaner: &Cleaner<'_>) {
        for name in self.flags.iter() {
            if !cleaner.docs.params.flags.contains_key(name) {
                panic!("ParamsDiff sets undocumented Flag {name:?}");
            }

            if self.unflags.contains(name) {
                panic!("ParamsDiff sets and unsets Flag {name:?}.");
            }
        }



        for (name, value) in self.vars.iter() {
            match cleaner.docs.params.vars.get(name) {
                Some(doc) => {
                    if let Some(variants) = &doc.variants && !variants.contains_key(value) {
                        panic!("ParamsDiff sets Var {name:?} to undocumented variant {value:?}");
                    }
                },
                None => panic!("ParamsDiff sets undocumented Var {name:?}"),
            }

            if self.unvars.contains(name) {
                panic!("ParamsDiff sets and unsets Var {name:?}.");
            }
        }

        for name in self.unvars.iter() {
            if let Some(doc) = cleaner.docs.params.vars.get(name) && doc.required {
                panic!("ParamsDiff unsets required Var {name:?}");
            }
        }



        for (name, inserts) in self.insert_into_sets.iter() {
            if !cleaner.docs.params.sets.contains_key(name) {
                panic!("ParamsDiff inserts into undocumented Set {name:?}");
            }

            if let Some(removes) = self.remove_from_sets.get(name) {
                for element in inserts {
                    if removes.contains(element) {
                        panic!("ParamsDiff inserts and removes {element:?} into and from Set {name:?}.");
                    }
                }
            }
        }



        for (name, inserts) in self.insert_into_maps.iter() {
            if !cleaner.docs.params.maps.contains_key(name) {
                panic!("ParamsDiff inserts into undocumented Map {name:?}");
            }

            if let Some(removes) = self.remove_from_maps.get(name) {
                for (name, _) in inserts {
                    if removes.contains(name) {
                        panic!("ParamsDiff inserts and removes {name:?} into and from Map {name:?}.");
                    }
                }
            }
        }
    }
}
