//! [`Set`].

use std::collections::HashSet;
use std::hash::Hash;
use std::borrow::Borrow;

use serde::{Serialize, Deserialize, ser::{Serializer, SerializeSeq}, de::{Deserializer, Visitor, SeqAccess, Error as _}};

use crate::prelude::*;

/// A `HashMap<Option<T>>` that allows indexing with `Option<&Q>` where `T: Borrow<Q>`.
///
/// Mainly used to allow indexing `Set<String>` with `Option<&str>`, where a [`HashSet`] would require `&Option<String>`.
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
///
/// assert_eq!(
///     serde_json::from_str::<Set<String>>(r#"["abc"]"#).unwrap(),
///     Set {
///         set: ["abc".into()].into(),
///         if_none: false
///     }
/// );
///
/// assert_eq!(
///     serde_json::from_str::<Set<String>>(r#"["abc", null]"#).unwrap(),
///     Set {
///         set: ["abc".into()].into(),
///         if_none: true
///     }
/// );
/// ```
#[derive(Debug, Clone, Suitability)]
pub struct Set<T> {
    /// The set of `T`.
    pub set: HashSet<T>,
    /// If [`true`], act like [`None`] is in [`Self::set`].
    pub if_none: bool
}

impl<T: Hash + Eq> Set<T> {
    /// [`HashSet::with_capacity`].
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            set: HashSet::with_capacity(capacity),
            if_none: false
        }
    }

    /// [`HashSet::contains`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let no_none = Set {set: ["abc"].into(), if_none: false};
    /// assert!( no_none.contains(Some("abc")));
    /// assert!(!no_none.contains(None::<&str>));
    ///
    /// let yes_none = Set {set: ["abc"].into(), if_none: true};
    /// assert!(yes_none.contains(Some("abc")));
    /// assert!(yes_none.contains(None::<&str>));
    /// ```
    pub fn contains<Q>(&self, value: Option<&Q>) -> bool where T: Borrow<Q>, Q: Hash + Eq + ?Sized {
        match value {
            Some(x) => self.set.contains(x),
            None => self.if_none
        }
    }

    /// [`Self::contains`] without [`None`].
    pub fn contains_some<Q>(&self, value: &Q) -> bool where T: Borrow<Q>, Q: Hash + Eq + ?Sized {
        self.set.contains(value)
    }

    /// [`HashSet::insert`].
    pub fn insert(&mut self, value: Option<T>) -> bool {
        match value {
            Some(value) => self.set.insert(value),
            None => {let ret = !self.if_none; self.if_none = true; ret}
        }
    }

    /// [`HashSet::remove`].
    pub fn remove<Q>(&mut self, value: Option<&Q>) -> bool where T: Borrow<Q>, Q: Hash + Eq + ?Sized {
        match value {
            Some(value) => self.set.remove(value),
            None => {let ret = self.if_none; self.if_none = false; ret}
        }
    }

    /// The length of the set.
    pub fn len(&self) -> usize {
        self.set.len() + (self.if_none as usize)
    }

    /// If the set is empty.
    pub fn is_empty(&self) -> bool {
        self.set.is_empty() || self.if_none
    }
}

/// Implemented manually to avoid the `T: Default` bound.
impl<T> Default for Set<T> {
    fn default() -> Self {
        Self {
            set: Default::default(),
            if_none: Default::default()
        }
    }
}

impl<T: Hash + Eq> PartialEq for Set<T> {
    fn eq(&self, other: &Self) -> bool {
        self.set == other.set && self.if_none == other.if_none
    }
}
impl<T: Hash + Eq> Eq for Set<T> {}

impl<T: Eq + Hash, const N: usize> From<[Option<T>; N]> for Set<T> {
    fn from(value: [Option<T>; N]) -> Self {
        let mut ret = Self::default();
        for x in value {
            ret.insert(x);
        }
        ret
    }
}

impl<T: Eq + Hash, const N: usize> From<[T; N]> for Set<T> {
    fn from(value: [T; N]) -> Self {
        let mut ret = Self::default();
        for x in value {
            ret.insert(Some(x));
        }
        ret
    }
}

impl<T: Eq + Hash> FromIterator<T> for Set<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self{
        let mut ret = Self::default();
        for x in iter {
            ret.insert(Some(x));
        }
        ret
    }
}

impl<T: Eq + Hash> FromIterator<Option<T>> for Set<T> {
    fn from_iter<I: IntoIterator<Item = Option<T>>>(iter: I) -> Self{
        let mut ret = Self::default();
        for x in iter {
            ret.insert(x);
        }
        ret
    }
}

impl<T: Hash + Eq> From<HashSet<Option<T>>> for Set<T> {
    fn from(value: HashSet<Option<T>>) -> Self {
        let mut ret = Self::default();
        for x in value {
            match x {
                Some(x) => {ret.set.insert(x);},
                None => ret.if_none = true
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
        if value.if_none {ret.insert(None);}
        ret
    }
}

impl<T: Hash + Eq> Extend<T> for Set<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.set.extend(iter);
    }
}

impl<T: Hash + Eq> Extend<Option<T>> for Set<T> {
    fn extend<I: IntoIterator<Item = Option<T>>>(&mut self, iter: I) {
        for x in iter {
            match x {
                Some(x) => {self.set.insert(x);},
                None => self.if_none = true
            }
        }
    }
}

impl<T: Serialize> Serialize for Set<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.set.len() + (self.if_none as usize)))?;
        if self.if_none {seq.serialize_element(&None::<T>)?;}
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
        let mut ret = Set::with_capacity(seq.size_hint().unwrap_or_default());
        while let Some(x) = seq.next_element()? {
            if !ret.insert(x) {
                Err(A::Error::custom("invalid entry: found duplicate value"))?;
            }
        }
        Ok(ret)
    }
}
