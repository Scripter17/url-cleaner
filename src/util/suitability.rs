//! Trait to unify "suitability".

use std::fmt::Debug;
use std::collections::{HashSet, HashMap};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use crate::types::*;

pub(crate) use url_cleaner_macros::Suitability;

/// Trait to stop me from comitting debug stuff.
pub(crate) trait Suitability: Debug {
    /// Panics if `self` is deemed "unsuitable" for being in the default config.
    fn assert_suitability(&self, config: &Config);
}

/// Quick implementations for types that are always suitable.
macro_rules! always_suitable {
    ($($t:ty),+) => {
        $(impl Suitability for $t {fn assert_suitability(&self, _: &Config) {}})+
    }
}

always_suitable!(char, str, String, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, url::Url, BetterUrl, serde_json::Value, serde_json::Number, Path, PathBuf, std::time::Duration);
#[cfg(feature = "http")] always_suitable!(reqwest::header::HeaderMap, reqwest::header::HeaderValue, reqwest::Method);
#[cfg(feature = "glob")] always_suitable!(glob::Pattern, glob::MatchOptions);

/// Helper function to make sure I don't miss documenting anything.
pub(crate) fn context_var_is_documented       (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.job_context.vars   .contains_key(name), "Undocumented JobContext var: {name}")}}
/// Helper function to make sure I don't miss documenting anything.
pub(crate) fn jobs_context_var_is_documented  (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.jobs_context.vars  .contains_key(name), "Undocumented JobsContext var: {name}")}}
/// Helper function to make sure I don't miss documenting anything.
pub(crate) fn flag_is_documented              (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.flags              .contains_key(name), "Undocumented Flag: {name}")}}
/// Helper function to make sure I don't miss documenting anything.
pub(crate) fn var_is_documented               (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.vars               .contains_key(name), "Undocumented Var: {name}")}}
/// Helper function to make sure I don't miss documenting anything.
pub(crate) fn set_is_documented               (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.sets               .contains_key(name), "Undocumented Set: {name}")}}
/// Helper function to make sure I don't miss documenting anything.
pub(crate) fn map_is_documented               (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.maps               .contains_key(name), "Undocumented Map: {name}")}}
/// Helper function to make sure I don't miss documenting anything.
pub(crate) fn env_var_is_documented           (name: &StringSource, config: &Config) {if let StringSource::String(name) = name {assert!(config.docs.environment_vars   .contains_key(name), "Undocumented Env var: {name}")}}
/// Helper function to make sure I don't miss documenting anything.
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
