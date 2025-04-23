//! Effectively a way to query multiple [`HashSet`]s at once.

use std::collections::{HashMap, hash_map::Entry};
use std::sync::Arc;
#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::collections::HashSet;

use serde::{Serialize, Deserialize, ser::{Serializer, SerializeMap}, de::{Visitor, MapAccess, Deserializer, Error}};
use thiserror::Error;

use crate::util::*;

/// A [`NamedPartitioning`] effectively allows you to query multiple disjoint [`HashSet`]s at once and finding which one an element belongs to.
///
/// Semantically, this is done by joining the sets into one and partitioning them into regions. Technically this is just a fancy [`HashMap`] with basic optimizations.
///
/// For the math end of this idea, see [this Wikipedia article](https://en.wikipedia.org/wiki/Partition_of_a_set).
/// ```
/// use serde_json::from_str;
/// use url_cleaner::types::*;
///
/// let digits = serde_json::from_str::<NamedPartitioning>(r#"{"even": ["0", "2", "4", "6", "8"], "odd": ["1", "3", "5", "7", "9"]}"#).unwrap();
///
/// assert_eq!(digits.get_partition("0"), Some("even"));
/// assert_eq!(digits.get_partition("1"), Some("odd" ));
/// assert_eq!(digits.get_partition("2"), Some("even"));
/// assert_eq!(digits.get_partition("3"), Some("odd" ));
/// assert_eq!(digits.get_partition("4"), Some("even"));
/// assert_eq!(digits.get_partition("5"), Some("odd" ));
/// assert_eq!(digits.get_partition("6"), Some("even"));
/// assert_eq!(digits.get_partition("7"), Some("odd" ));
/// assert_eq!(digits.get_partition("8"), Some("even"));
/// assert_eq!(digits.get_partition("9"), Some("odd" ));
/// assert_eq!(digits.get_partition("a"), None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Suitability)]
pub struct NamedPartitioning {
    /// The map from values to their partitions.
    map: HashMap<String, Arc<str>>,
    /// The list of partition names.
    ///
    /// Used for serialization.
    partition_names: Vec<Arc<str>>
}

/// The enum of errors that can happen when making a [`NamedPartitioning`].
#[derive(Debug, Error)]
pub enum MakeNamedPartitioningError {
    /// Returned when attempting to specify multiple partitions with the same name or the same partition twice.
    #[error("Attempted to specify multiple partitions with the same name or the same partition twice.")]
    DuplicateName {
        /// The name of the duplicate partition.
        name: String
    },
    /// Returned when attempting to assign a value to multiple partitions at once.
    #[error("Attempted to assign {value} to multiple partitions at once.")]
    DuplicateValue {
        /// The name of the second partition.
        name: String,
        /// The value.
        value: String
    }
}

impl NamedPartitioning {
    /// Collects an iterator of `(String, Vec<String>)` into a [`NamedPartitioning`].
    /// # Errors
    /// If multiple values have the same name (the first value in the tuple), returns the error [`MakeNamedPartitioningError::DuplicateName`].
    ///
    /// If multiple values contain the same value (the elements in the [`Vec`] in the second value of the tuple), returns the error [`MakeNamedPartitioningError::DuplicateValue`].
    pub fn try_from_iter<I: IntoIterator<Item = (String, Vec<String>)>>(iter: I) -> Result<Self, MakeNamedPartitioningError> {
        let mut ret = NamedPartitioning {
            map: HashMap::new(),
            partition_names: Vec::new()
        };

        for (k, vs) in iter {
            let partition_name: Arc<str> = Arc::from(&*k);
            if ret.partition_names.iter().any(|x| **x == *partition_name) {Err(MakeNamedPartitioningError::DuplicateName {name: partition_name.to_string()})?;}
            for v in vs {
                match ret.map.entry(v) {
                    Entry::Vacant(e) => {e.insert(partition_name.clone());},
                    Entry::Occupied(e) => {let (value, name) = e.remove_entry(); Err(MakeNamedPartitioningError::DuplicateValue {name: name.to_string(), value})?}
                }
            }
            ret.partition_names.push(partition_name)
        }

        Ok(ret)
    }

    /// If `element`] is in `self`, return the partition it belongs to.
    pub fn get_partition<'a>(&'a self, element: &str) -> Option<&'a str> {
        self.map.get(element).map(|x| &**x)
    }

    /// The list of partition names.
    pub fn partition_names(&self) -> &[Arc<str>] {
        &self.partition_names
    }
}

/// Serde helper for deserializing [`NamedPartitioning`].
struct NamedPartitioningVisitor;

impl<'de> Visitor<'de> for NamedPartitioningVisitor {
    type Value = NamedPartitioning;

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        NamedPartitioning::try_from_iter(std::iter::from_fn(|| map.next_entry::<String, Vec<String>>().transpose()).collect::<Result<Vec<_>, _>>()?).map_err(A::Error::custom)
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Expected a map")
    }
}

impl<'de> Deserialize<'de> for NamedPartitioning {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(NamedPartitioningVisitor)
    }
}

impl Serialize for NamedPartitioning {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut serializer = serializer.serialize_map(None)?;

        for name in self.partition_names() {
            let mut values = Vec::new();

            for (k, v) in self.map.iter() {
                if v == name {values.push(k);}
            }

            serializer.serialize_entry(&**name, &values)?;
        }

        serializer.end()
    }
}
