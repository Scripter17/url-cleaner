//! Allows passing additional details into various types in URL Cleaner.

use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};

use crate::glue::*;
use crate::util::*;

/// Configuration options to choose the behaviour of various URL Cleaner types.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Params {
    /// Booleans variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// String variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// Set variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: HashMap<String, HashSet<String>>,
    /// List variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: HashMap<String, Vec<String>>,
    /// Map variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: HashMap<String, HashMap<String, String>>,
    /// If [`true`], enables reading from caches. Defaults to [`true`]
    #[cfg(feature = "cache")]
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub read_cache: bool,
    /// If [`true`], enables writing to caches. Defaults to [`true`]
    #[cfg(feature = "cache")]
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub write_cache: bool,
    /// The default headers to send in HTTP requests.
    #[cfg(feature = "http")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub http_client_config: HttpClientConfig
}

const fn is_true(x: &bool) -> bool {!*x}

#[allow(clippy::derivable_impls, reason = "When the `cache` feature is enabled, this can't be derived.")]
impl Default for Params {
    fn default() -> Self {
        Self {
            flags: HashSet::default(),
            vars : HashMap::default(),
            sets : HashMap::default(),
            lists: HashMap::default(),
            maps : HashMap::default(),
            #[cfg(feature = "cache")] read_cache: true,
            #[cfg(feature = "cache")] write_cache: true,
            #[cfg(feature = "http")]
            http_client_config: HttpClientConfig::default()
        }
    }
}

/// Serde helper function.
const fn get_true() -> bool {true}

impl Params {
    /// Gets an HTTP client with [`Self`]'s configuration pre-applied.
    /// # Errors
    /// Errors if [`reqwest::ClientBuilder::build`] errors.
    #[cfg(feature = "http")]
    pub fn http_client(&self, http_client_config_diff: Option<&HttpClientConfigDiff>) -> reqwest::Result<reqwest::blocking::Client> {
        debug!(Params::http_client, self, http_client_config_diff);
        match http_client_config_diff {
            Some(http_client_config_diff) => {
                let mut temp_http_client_config = self.http_client_config.clone();
                http_client_config_diff.apply(&mut temp_http_client_config);
                temp_http_client_config.apply(reqwest::blocking::ClientBuilder::new())
            },
            None => {self.http_client_config.apply(reqwest::blocking::ClientBuilder::new())}
        }?.build()
    }
}

/// Allows changing [`Config::params`].
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ParamsDiff {
    /// Adds to [`Params::flags`]. Defaults to an empty [`HashSet`].
    #[serde(default, skip_serializing_if = "is_default")] pub flags  : HashSet<String>,
    /// Removes from [`Params::flags`] Defaults to an empty [`HashSet`].
    #[serde(default, skip_serializing_if = "is_default")] pub unflags: HashSet<String>,
    /// Adds to [`Params::vars`]. Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")] pub vars  : HashMap<String, String>,
    /// Removes from [`Params::vars`]. Defaults to an empty [`HashSet`].
    #[serde(default, skip_serializing_if = "is_default")] pub unvars: HashSet<String>,
    /// Initializes new sets in [`Params::sets`].
    #[serde(default, skip_serializing_if = "is_default")] pub init_sets: Vec<String>,
    /// Initializes new sets in [`Params::sets`] if they don't already exist, then inserts values into them.
    #[serde(default, skip_serializing_if = "is_default")] pub insert_into_sets: HashMap<String, Vec<String>>,
    /// If the sets exist in [`Params::sets`], removes values from them.
    #[serde(default, skip_serializing_if = "is_default")] pub remove_from_sets: HashMap<String, Vec<String>>,
    /// If the sets exist in [`Params::sets`], remove them.
    #[serde(default, skip_serializing_if = "is_default")] pub delete_sets: Vec<String>,
    /// Initializes new maps in [`Params::maps`].
    #[serde(default, skip_serializing_if = "is_default")] pub init_maps: Vec<String>,
    /// Initializes new maps in [`Params::maps`] if they don't already exist, then inserts values into them.
    #[serde(default, skip_serializing_if = "is_default")] pub insert_into_maps: HashMap<String, HashMap<String, String>>,
    /// If the maps exist in [`Params::maps`], removes values from them.
    #[serde(default, skip_serializing_if = "is_default")] pub remove_from_maps: HashMap<String, HashMap<String, String>>,
    /// If the maps exist in [`Params::maps`], remove them.
    #[serde(default, skip_serializing_if = "is_default")] pub delete_maps: Vec<String>,
    /// If [`Some`], sets [`Params::read_cache`]. Defaults to [`None`].
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")] pub read_cache : Option<bool>,
    /// If [`Some`], sets [`Params::write_cache`]. Defaults to [`None`].
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")] pub write_cache: Option<bool>,
    /// If [`Some`], calls [`HttpClientConfigDiff::apply`] with `to`'s [`HttpClientConfig`]. Defaults to [`None`].
    #[cfg(feature = "http")]
    #[serde(default, skip_serializing_if = "is_default")] pub http_client_config_diff: Option<HttpClientConfigDiff>
}

impl ParamsDiff {
    /// Applies the differences specified in `self` to `to`.
    /// In order:
    /// 1. Extends `to.flags` with [`Self::flags`].
    /// 2. Removes all flags found in [`Self::unflags`] from `to.flags`.
    /// 3. Extends `to.vars` with [`Self::vars`], overwriting any keys found in both.
    /// 4. Removes all keys found in [`Self::unvars`] from `to.vars`.
    /// 5. Initializes all sets specified by [`Self::init_sets`] to [`HashSet::default`] if they don't exist.
    /// 6. Inserts all values into sets as specified by [`Self::insert_into_sets`].
    /// 7. Removes all values from sets as specified by [`Self::remove_from_sets`].
    /// 8. Deletes all sets specified in [`Self::delete_sets`].
    /// 9. Initializes all maps specified by [`Self::init_maps`] to [`HashSet::default`] if they don't exist.
    /// 10. Inserts all values into maps as specified by [`Self::insert_into_maps`].
    /// 11. Removes all values from maps as specified by [`Self::remove_from_maps`].
    /// 12. Deletes all maps specified in [`Self::delete_maps`].
    /// 13. If [`Self::read_cache`] is [`Some`], sets `to.read_cache` to the contained value.
    /// 14. If [`Self::write_cache`] is [`Some`], sets `to.write_cache` to the contained value.
    /// 15. If [`Self::http_client_config_diff`] is [`Some`], calls [`HttpClientConfigDiff::apply`] with `to.http_client_config`.
    pub fn apply(&self, to: &mut Params) {
        #[cfg(feature = "debug")]
        let old_to = to.clone();

        to.flags.extend(self.flags.clone());
        for flag in &self.unflags {to.flags.remove(flag);}

        to.vars.extend(self.vars.clone());
        for var in &self.unvars {to.vars.remove(var);}

        for k in self.init_sets.iter() {
            if !to.sets.contains_key(k) {to.sets.insert(k.clone(), Default::default());}
        }
        for (k, v) in self.insert_into_sets.iter() {
            to.sets.entry(k.clone()).or_default().extend(v.clone());
        }
        for (k, vs) in self.remove_from_sets.iter() {
            if let Some(x) = to.sets.get_mut(k) {
                for v in vs.iter() {
                    x.remove(v);
                }
            }
        }
        for k in self.delete_sets.iter() {
            to.sets.remove(k);
        }

        for k in self.init_maps.iter() {
            if !to.maps.contains_key(k) {to.maps.insert(k.clone(), Default::default());}
        }
        for (k, v) in self.insert_into_maps.iter() {
            to.maps.entry(k.clone()).or_default().extend(v.clone());
        }
        for (k, vs) in self.remove_from_maps.iter() {
            if let Some(x) = to.maps.get_mut(k) {
                for v in vs.keys() {
                    x.remove(v);
                }
            }
        }
        for k in self.delete_maps.iter() {
            to.maps.remove(k);
        }

        #[cfg(feature = "cache")] if let Some(read_cache ) = self.read_cache  {to.read_cache  = read_cache ;}
        #[cfg(feature = "cache")] if let Some(write_cache) = self.write_cache {to.write_cache = write_cache;}

        #[cfg(feature = "http")] if let Some(http_client_config_diff) = &self.http_client_config_diff {http_client_config_diff.apply(&mut to.http_client_config);}
        debug!(ParamsDiff::apply, self, old_to, to);
    }
}
