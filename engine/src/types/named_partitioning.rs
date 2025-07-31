//! Effectively a way to query multiple [`HashSet`]s at once.

use std::collections::{HashMap, hash_map::Entry};
use std::sync::Arc;
use std::collections::HashSet;

use serde::{Serialize, Deserialize, ser::{Serializer, SerializeMap}, de::{Visitor, MapAccess, Deserializer, Error}};
use thiserror::Error;

use crate::util::*;

/// A [`NamedPartitioning`] effectively allows you to query multiple disjoint [`HashSet`]s at once and finding which one an element belongs to.
///
/// Semantically, this is done by joining the sets into one and partitioning them into regions. Technically this is just a fancy [`HashMap`] with basic optimizations and a brief JSON representation.
///
/// For the math end of this idea, see [this Wikipedia article](https://en.wikipedia.org/wiki/Partition_of_a_set).
/// ```
/// use serde_json::from_str;
/// use url_cleaner_engine::types::*;
///
/// let digits = serde_json::from_str::<NamedPartitioning>(r#"{"even": ["0", "2", "4", "6", "8"], "odd": ["1", "3", "5", "7", "9"]}"#).unwrap();
///
/// assert_eq!(digits.get_partition_of(Some("0")), Some("even"));
/// assert_eq!(digits.get_partition_of(Some("1")), Some("odd" ));
/// assert_eq!(digits.get_partition_of(Some("2")), Some("even"));
/// assert_eq!(digits.get_partition_of(Some("3")), Some("odd" ));
/// assert_eq!(digits.get_partition_of(Some("4")), Some("even"));
/// assert_eq!(digits.get_partition_of(Some("5")), Some("odd" ));
/// assert_eq!(digits.get_partition_of(Some("6")), Some("even"));
/// assert_eq!(digits.get_partition_of(Some("7")), Some("odd" ));
/// assert_eq!(digits.get_partition_of(Some("8")), Some("even"));
/// assert_eq!(digits.get_partition_of(Some("9")), Some("odd" ));
/// assert_eq!(digits.get_partition_of(Some("a")), None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Suitability)]
pub struct NamedPartitioning {
    /// The map from values to their partitions.
    map: HashMap<String, Arc<str>>,
    /// The partition to put [`None`] into.
    if_none: Option<Arc<str>>
}

/// The enum of errors that can happen when making a [`NamedPartitioning`].
#[derive(Debug, Error)]
pub enum MakeNamedPartitioningError {
    /// Returned when attempting to make multiple partitions named [`Self::DuplicatePartition::name`].
    #[error("Attempted to make multiple partitions named {name:?}.")]
    DuplicatePartition {
        /// The name of the duplicate partition.
        name: String
    },
    /// Returned when attempting to assign [`Self::DuplicateElement::element`] to partition [`Self::DuplicateElement::second_partition`] when it's already in partition [`Self::DuplicateElement::first_partition`].
    #[error("Attempted to assign element {element:?} to partition {second_partition:?} when it's already in partition {first_partition:?}.")]
    DuplicateElement {
        /// The name of the first partition.
        first_partition: String,
        /// The name of the second partition.
        second_partition: String,
        /// The element.
        element: Option<String>
    }
}

impl NamedPartitioning {
    /// Collects an iterator of `(String, Vec<String>)` into a [`NamedPartitioning`].
    /// # Errors
    /// If multiple partitions have the same name (the first value in the tuple), returns the error [`MakeNamedPartitioningError::DuplicatePartition`].
    ///
    /// If multiple partitions contain the same element (the elements in the [`Vec`] in the second value of the tuple), returns the error [`MakeNamedPartitioningError::DuplicateElement`].
    pub fn try_from_iter<I: IntoIterator<Item = (String, Vec<Option<String>>)>>(iter: I) -> Result<Self, MakeNamedPartitioningError> {
        let mut ret = NamedPartitioning {
            map: HashMap::new(),
            if_none: None
        };

        let mut partition_names = HashSet::<Arc<str>>::new();

        for (partition_name, elements) in iter {
            let partition_name: Arc<str> = Arc::from(&*partition_name);
            if partition_names.insert(partition_name.clone()) {
                for element in elements {
                    match element {
                        Some(element) => match ret.map.entry(element) {
                            Entry::Vacant(e) => {e.insert(partition_name.clone());},
                            Entry::Occupied(e) => {
                                let (element, name) = e.remove_entry();
                                Err(MakeNamedPartitioningError::DuplicateElement {
                                    first_partition: name.to_string(),
                                    second_partition: partition_name.to_string(),
                                    element: Some(element)
                                })?
                            }
                        },
                        None => if let Some(ref first_partition) = ret.if_none {
                            Err(MakeNamedPartitioningError::DuplicateElement {
                                first_partition: first_partition.to_string(),
                                second_partition: partition_name.to_string(),
                                element: None
                            })?
                        } else {
                            ret.if_none = Some(partition_name.clone());
                        }
                    }
                }
            } else {
                return Err(MakeNamedPartitioningError::DuplicatePartition {name: partition_name.to_string()});
            }
        }

        Ok(ret)
    }

    /// If `element`] is in `self`, return [`true`].
    pub fn contains(&self, element: Option<&str>) -> bool {
        debug!(NamedPartitioning::contains, self, element);
        match element {
            Some(element) => self.map.contains_key(element),
            None => self.if_none.is_some()
        }
    }

    /// If `element`] is in `self`, return the partition it belongs to.
    pub fn get_partition_of<'a>(&'a self, element: Option<&str>) -> Option<&'a str> {
        debug!(NamedPartitioning::get_partition_of, self, element);
        match element {
            Some(element) => self.map.get(element).map(|x| &**x),
            None => self.if_none.as_deref()
        }
    }
}

/// Serde helper for deserializing [`NamedPartitioning`].
struct NamedPartitioningVisitor;

impl<'de> Visitor<'de> for NamedPartitioningVisitor {
    type Value = NamedPartitioning;

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        NamedPartitioning::try_from_iter(std::iter::from_fn(|| map.next_entry::<String, Vec<Option<String>>>().transpose()).collect::<Result<Vec<_>, _>>()?).map_err(A::Error::custom)
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
        let mut x = HashMap::<&str, Vec<Option<&str>>>::new();

        for (element, partition_name) in self.map.iter() {
            x.entry(partition_name).or_default().push(Some(element));
        }

        if let Some(ref partition_name) = self.if_none {
            x.entry(partition_name).or_default().push(None);
        }

        let mut serializer = serializer.serialize_map(Some(x.len()))?;

        for (name, values) in x {
            serializer.serialize_entry(name, &values)?;
        }

        serializer.end()
    }
}
