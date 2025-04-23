//! Faster `HashSet<Option<String>>`.

use std::collections::HashSet;
use std::hash::Hash;
use std::borrow::Borrow;

use serde::{Serialize, Deserialize, ser::{Serializer, SerializeSeq}, de::{Deserializer, Visitor, SeqAccess}};

use crate::util::*;

/// Allows semantics similar to a `HashSet<Option<String>>` without having to convert `Option<&str>`s to `Option<String>`s.
///
/// Serializes and deserializes identically to `HashSet<Option<String>>`, though it's not yet optimized.
/// # Examples
/// ```
/// use serde_json::from_str;
/// use url_cleaner::types::*;
///
/// assert_eq!(serde_json::from_str::<Set<String>>(r#"["abc"]"#      ).unwrap(), Set {set: ["abc".into()].into(), if_null: false});
/// assert_eq!(serde_json::from_str::<Set<String>>(r#"["abc", null]"#).unwrap(), Set {set: ["abc".into()].into(), if_null: true });
/// ```
#[derive(Debug, Clone, Suitability)]
pub struct Set<T> {
    /// The set of `T`.
    pub set: HashSet<T>,
    /// If [`true`], act like [`None`] is in [`Self::set`].
    pub if_null: bool
}

impl<T: Hash + Eq> Set<T> {
    /// # Examples
    /// ```
    /// use url_cleaner::types::*;
    ///
    /// let no_null = Set {set: ["abc"].into(), if_null: false};
    /// assert!( no_null.contains(Some("abc")));
    /// assert!(!no_null.contains(None::<&str>));
    ///
    /// let yes_null = Set {set: ["abc"].into(), if_null: true};
    /// assert!(yes_null.contains(Some("abc")));
    /// assert!(yes_null.contains(None::<&str>));
    /// ```
    pub fn contains<Q>(&self, value: Option<&Q>) -> bool where T: Borrow<Q>, Q: Hash + Eq + ?Sized {
        match value {
            Some(x) => self.set.contains(x),
            None => self.if_null
        }
    }
}

/// Implemented manually to avoid the `T: Default` bound.
impl<T> Default for Set<T> {
    fn default() -> Self {
        Self {
            set: Default::default(),
            if_null: Default::default()
        }
    }
}

impl<T: Hash + Eq> PartialEq for Set<T> {
    fn eq(&self, other: &Self) -> bool {
        self.set == other.set && self.if_null == other.if_null
    }
}
impl<T: Hash + Eq> Eq for Set<T> {}

impl<T: Hash + Eq> From<HashSet<Option<T>>> for Set<T> {
    fn from(value: HashSet<Option<T>>) -> Self {
        let mut ret = Self::default();
        for x in value {
            match x {
                Some(x) => {ret.set.insert(x);},
                None => ret.if_null = true
            }
        }
        ret
    }
}

impl<T: Hash + Eq> From<Set<T>> for HashSet<Option<T>> {
    fn from(value: Set<T>) -> Self {
        let mut ret = Self::default();
        for x in value.set {
            ret.insert(Some(x));
        }
        if value.if_null {ret.insert(None);}
        ret
    }
}

impl<T: Serialize> Serialize for Set<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
        let mut seq = serializer.serialize_seq(Some(self.set.len() + (self.if_null as usize)))?;
        if self.if_null {seq.serialize_element(&None::<T>)?;}
        for element in &self.set {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<'de, T: Deserialize<'de> + Eq + Hash> Deserialize<'de> for Set<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_seq(SetDeserializer::<T>(Default::default()))
    }
}

/// Y'know, I don't actually understand why serde uses this structure.
#[derive(Debug, Default)]
struct SetDeserializer<T>(std::marker::PhantomData<T>);

impl<'de, T: Deserialize<'de> + Eq + Hash> Visitor<'de> for SetDeserializer<T> {
    type Value = Set<T>;

    fn expecting(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "A sequence of type {}", std::any::type_name::<Option<T>>())
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut ret = Set::default();
        while let Some(x) = seq.next_element()? {
            match x {
                Some(x) => {ret.set.insert(x);},
                None => ret.if_null = true
            }
        }
        Ok(ret)
    }
}
