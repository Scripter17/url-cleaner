//! A system to stop me from accidentally comitting debug stuff to the default config.

use std::fmt::Debug;
use std::collections::{HashSet, HashMap};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use crate::types::*;

pub(crate) use url_cleaner_macros::Suitability;

/// A trait for things that may or may not be suitable for use in the default config.
pub(crate) trait Suitability: Debug {
    /// If `self` is deemed unsuitable to be in the default config, panics.
    /// # Panics
    /// If `self` is deemed unsuitable to be in the default config, panics.
    fn assert_suitability(&self, config: &Config);
}

/// Generate [`Suitability`] impls for types that are always suitable for use in the default config.
macro_rules! always_suitable {
    ($($t:ty),+) => {
        $(impl Suitability for $t {fn assert_suitability(&self, _: &Config) {}})+
    }
}

always_suitable!(char, str, String, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, url::Url, BetterUrl, serde_json::Value, serde_json::Number, Path, PathBuf, std::time::Duration);
#[cfg(feature = "http")] always_suitable!(reqwest::header::HeaderMap, reqwest::header::HeaderValue, reqwest::Method);
#[cfg(feature = "glob")] always_suitable!(glob::Pattern, glob::MatchOptions);

/// Suitability helper function to check that a context var is documented.
pub(crate) fn context_var_is_documented       (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.job_context.vars   .contains_key(name), "Undocumented JobContext var: {name}")}}
/// Suitability helper function to check that a jobs context var is documented.
pub(crate) fn jobs_context_var_is_documented  (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.jobs_context.vars  .contains_key(name), "Undocumented JobsContext var: {name}")}}
/// Suitability helper function to check that a flag is documented.
pub(crate) fn flag_is_documented              (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.flags              .contains_key(name), "Undocumented Flag: {name}")}}
/// Suitability helper function to check that a var is documented.
pub(crate) fn var_is_documented               (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.vars               .contains_key(name), "Undocumented Var: {name}")}}
/// Suitability helper function to check that a set is documented.
pub(crate) fn set_is_documented               (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.sets               .contains_key(name), "Undocumented Set: {name}")}}
/// Suitability helper function to check that a map is documented.
pub(crate) fn map_is_documented               (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.maps               .contains_key(name), "Undocumented Map: {name}")}}
/// Suitability helper function to check that a env var is documented.
pub(crate) fn env_var_is_documented           (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.environment_vars   .contains_key(name), "Undocumented Env var: {name}")}}
/// Suitability helper function to check that a named partitioning is documented.
pub(crate) fn named_partitioning_is_documented(name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.named_partitionings.contains_key(name), "Undocumented NamedPartitioning: {name}")}}

impl<K: Suitability, V: Suitability> Suitability for HashMap<K, V> {
    fn assert_suitability(&self, config: &Config) {
        for (k, v) in self.iter() {
            k.assert_suitability(config);
            v.assert_suitability(config);
        }
    }
}

impl<T: Suitability> Suitability for HashSet<T> {
    fn assert_suitability(&self, config: &Config) {
        for x in self.iter() {
            x.assert_suitability(config)
        }
    }
}

impl<T: Suitability> Suitability for Vec<T> {
    fn assert_suitability(&self, config: &Config) {
        for x in self.iter() {
            x.assert_suitability(config)
        }
    }
}

impl<T: Suitability> Suitability for &[T] {
    fn assert_suitability(&self, config: &Config) {
        for x in self.iter() {
            x.assert_suitability(config)
        }
    }
}

impl<T: Suitability + ?Sized> Suitability for Box<T> {
    fn assert_suitability(&self, config: &Config) {
        (**self).assert_suitability(config)
    }
}

impl<T: Suitability> Suitability for Option<T> {
    fn assert_suitability(&self, config: &Config) {
        if let Some(x) = self {x.assert_suitability(config);}
    }
}

impl<'a, T: Suitability + ToOwned + ?Sized + 'a> Suitability for Cow<'a, T> where <T as ToOwned>::Owned: Suitability {
    fn assert_suitability(&self, config: &Config) {
        (**self).assert_suitability(config)
    }
}

impl<T: Suitability + ?Sized> Suitability for Rc<T> {
    fn assert_suitability(&self, config: &Config) {
        (**self).assert_suitability(config)
    }
}
impl<T: Suitability + ?Sized> Suitability for Arc<T> {
    fn assert_suitability(&self, config: &Config) {
        (**self).assert_suitability(config)
    }
}
