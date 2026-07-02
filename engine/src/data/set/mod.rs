//! [`Set`].

use crate::prelude::*;

mod source;
mod iter;
mod into_iter;

pub use source::*;
pub use iter::*;
pub use into_iter::*;

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
    /// If [`Some`], [`HashSet::contains`], otherwise [`Self::if_none`].
    pub fn contains<Q>(&self, value: Option<&Q>) -> bool where T: Borrow<Q>, Q: Hash + Eq + ?Sized {
        match value {
            Some(x) => self.set.contains(x),
            None    => self.if_none
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
            None        => {let ret = !self.if_none; self.if_none = true; ret}
        }
    }

    /// [`HashSet::remove`].
    pub fn remove<Q>(&mut self, value: Option<&Q>) -> bool where T: Borrow<Q>, Q: Hash + Eq + ?Sized {
        match value {
            Some(value) => self.set.remove(value),
            None => {let ret = self.if_none; self.if_none = false; ret}
        }
    }

    /// Keep only elements satisfying `f`.
    pub fn retain<F: FnMut(Option<&T>) -> bool>(&mut self, mut f: F) {
        self.set.retain(|x| f(Some(x)));
        if self.if_none && !f(None) {
            self.if_none = false;
        }
    }

    /// The length of the set.
    pub fn len(&self) -> usize {
        self.set.len() + (self.if_none as usize)
    }

    /// If the set is empty.
    pub fn is_empty(&self) -> bool {
        self.set.is_empty() && !self.if_none
    }
}

impl<T> Default for Set<T> {
    fn default() -> Self {
        Self {
            set    : Default::default(),
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



impl<T: Eq + Hash> FromIterator<T> for Set<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self{
        let mut ret = Self::default();
        ret.extend(iter);
        ret
    }
}

impl<T: Hash + Eq> Extend<T> for Set<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.set.extend(iter);
    }
}



impl<T: Eq + Hash> FromIterator<Option<T>> for Set<T> {
    fn from_iter<I: IntoIterator<Item = Option<T>>>(iter: I) -> Self{
        let mut ret = Self::default();
        ret.extend(iter);
        ret
    }
}

impl<T: Hash + Eq> Extend<Option<T>> for Set<T> {
    fn extend<I: IntoIterator<Item = Option<T>>>(&mut self, iter: I) {
        for x in iter {
            match x {
                Some(x) => {self.set.insert(x);},
                None    => self.if_none = true
            }
        }
    }
}



impl<T: Serialize + Eq + Hash> Serialize for Set<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;

        for element in self {
            seq.serialize_element(&element)?;
        }

        seq.end()
    }
}

impl<'de, T: Deserialize<'de> + Eq + Hash> Deserialize<'de> for Set<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(SetDeserializer::default())
    }
}

/// Y'know, I don't actually understand why serde uses this structure.
#[derive(Debug)]
struct SetDeserializer<T>(std::marker::PhantomData<T>);

impl<T> Default for SetDeserializer<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'de, T: Deserialize<'de> + Eq + Hash> Visitor<'de> for SetDeserializer<T> {
    type Value = Set<T>;

    fn expecting(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "A sequence of type {}", std::any::type_name::<Option<T>>())
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut ret = Self::Value::default();

        while let Some(x) = seq.next_element()? {
            ret.insert(x);
        }

        Ok(ret)
    }
}
