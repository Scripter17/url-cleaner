//! A system to stop me from accidentally committing debug stuff to the bundled cleaner.

use std::fmt::Debug;
use std::collections::{HashSet, HashMap};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use crate::prelude::*;

pub(crate) use url_cleaner_macros::Suitability;

/// A trait for things that may or may not be suitable for use in the bundled cleaner.
pub(crate) trait Suitability: Debug {
    /// If `self` is deemed unsuitable to be in the bundled cleaner, panics.
    /// # Panics
    /// If `self` is deemed unsuitable to be in the bundled cleaner, panics.
    fn assert_suitability(&self, cleaner: &Cleaner);
}

/// Generate [`Suitability`] impls for types that are always suitable for use in the bundled cleaner.
macro_rules! always_suitable {
    ($($t:ty),+) => {
        $(impl Suitability for $t {fn assert_suitability(&self, _: &Cleaner) {}})+
    }
}

always_suitable!(char, str, String, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, url::Url, BetterUrl, serde_json::Value, serde_json::Number, Path, PathBuf, std::time::Duration, BetterPosition);
#[cfg(feature = "http")] always_suitable!(reqwest::header::HeaderMap, reqwest::header::HeaderValue, reqwest::Method);

/// Suitability helper function to check that a set is documented.
pub(crate) fn set_is_documented(name: &StringSource, cleaner: &Cleaner) {
    if let StringSource::String(name) = name {
        assert!(cleaner.params.sets.contains_key(name), "Unset Set: {name}");
        assert!(cleaner.docs.sets.contains_key(name), "Undocumented Set: {name}");
    }
}
/// Suitability helper function to check that a map is documented.
pub(crate) fn map_is_documented(name: &StringSource, cleaner: &Cleaner) {
    if let StringSource::String(name) = name {
        assert!(cleaner.params.maps.contains_key(name), "Unset Map: {name}");
        assert!(cleaner.docs.maps.contains_key(name), "Undocumented Map: {name}");
    }
}
/// Suitability helper function to check that a named partitioning is documented.
pub(crate) fn partitioning_is_documented(name: &StringSource, cleaner: &Cleaner) {
    if let StringSource::String(name) = name {
        assert!(cleaner.params.partitionings.contains_key(name), "Unset Partitioning: {name}");
        assert!(cleaner.docs.partitionings.contains_key(name), "Undocumented Partitioning: {name}");
    }
}

/// For [`Condition::UrlIs`].
pub(crate) fn string_source_string_literal_is_url_literal(value: &StringSource, _: &Cleaner) {
    if let StringSource::String(value) = value {
        assert_eq!(value, url::Url::parse(value).expect("Condition::UrlIs isn'tn even a valid URL").as_str(), "Condition::UrlIs will never be satisfied because its value changes when parsed.")
    }
}

impl<K: Suitability, V: Suitability> Suitability for HashMap<K, V> {
    fn assert_suitability(&self, cleaner: &Cleaner) {
        for (k, v) in self.iter() {
            k.assert_suitability(cleaner);
            v.assert_suitability(cleaner);
        }
    }
}

impl<T: Suitability> Suitability for HashSet<T> {
    fn assert_suitability(&self, cleaner: &Cleaner) {
        for x in self.iter() {
            x.assert_suitability(cleaner)
        }
    }
}

impl<T: Suitability> Suitability for Vec<T> {
    fn assert_suitability(&self, cleaner: &Cleaner) {
        for x in self.iter() {
            x.assert_suitability(cleaner)
        }
    }
}

impl<T: Suitability> Suitability for [T] {
    fn assert_suitability(&self, cleaner: &Cleaner) {
        for x in self.iter() {
            x.assert_suitability(cleaner)
        }
    }
}

impl<T: Suitability + ?Sized> Suitability for Box<T> {
    fn assert_suitability(&self, cleaner: &Cleaner) {
        (**self).assert_suitability(cleaner)
    }
}

impl<T: Suitability> Suitability for Option<T> {
    fn assert_suitability(&self, cleaner: &Cleaner) {
        if let Some(x) = self {x.assert_suitability(cleaner);}
    }
}

impl<'a, T: Suitability + ToOwned + ?Sized + 'a> Suitability for Cow<'a, T> where <T as ToOwned>::Owned: Suitability {
    fn assert_suitability(&self, cleaner: &Cleaner) {
        (**self).assert_suitability(cleaner)
    }
}

impl<T: Suitability + ?Sized> Suitability for Rc<T> {
    fn assert_suitability(&self, cleaner: &Cleaner) {
        (**self).assert_suitability(cleaner)
    }
}
impl<T: Suitability + ?Sized> Suitability for Arc<T> {
    fn assert_suitability(&self, cleaner: &Cleaner) {
        (**self).assert_suitability(cleaner)
    }
}
