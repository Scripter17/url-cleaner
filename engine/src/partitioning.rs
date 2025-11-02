//! [`Partitioning`].

use std::collections::{HashMap, hash_map::Entry};
use std::sync::Arc;
use std::collections::HashSet;

use serde::{Serialize, Deserialize, ser::{Serializer, SerializeMap}, de::{Visitor, MapAccess, Deserializer, Error}};
use thiserror::Error;

use crate::prelude::*;

/// Named and joined disjoint sets to allow finding which subset a value is in with only one hash and lookup.
///
/// Internally just a specialized [`Map`] with a fancy [`Serialize`] and [`Deserialize`].
///
/// While [`Arc`] is used for memory efficiency, nothing assumes there's only one [`Arc`] of a certain value.
///
/// `1` and `3` can point to different [`Arc`]s that are both `"odd"`.
///
/// For the math end of this idea, see [this Wikipedia article](https://en.wikipedia.org/wiki/Partition_of_a_set).
/// ```
/// use url_cleaner_engine::prelude::*;
///
/// let digits = serde_json::from_str::<Partitioning>(r#"
/// {
///     "even": ["0", "2", "4", "6", "8"],
///     "odd" : ["1", "3", "5", "7", "9"]
/// }
/// "#).unwrap();
///
/// assert_eq!(digits.get(Some("0")), Some("even"));
/// assert_eq!(digits.get(Some("1")), Some("odd" ));
/// assert_eq!(digits.get(Some("2")), Some("even"));
/// assert_eq!(digits.get(Some("3")), Some("odd" ));
/// assert_eq!(digits.get(Some("4")), Some("even"));
/// assert_eq!(digits.get(Some("5")), Some("odd" ));
/// assert_eq!(digits.get(Some("6")), Some("even"));
/// assert_eq!(digits.get(Some("7")), Some("odd" ));
/// assert_eq!(digits.get(Some("8")), Some("even"));
/// assert_eq!(digits.get(Some("9")), Some("odd" ));
/// assert_eq!(digits.get(Some("a")), None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Suitability)]
pub struct Partitioning {
    /// The map from values to their partitions.
    pub map: HashMap<String, Arc<str>>,
    /// The partition to put [`None`] into.
    pub if_none: Option<Arc<str>>
}

/// The enum of errors that can happen when making a [`Partitioning`].
#[derive(Debug, Error)]
pub enum MakePartitioningError {
    /// Returned when attempting to make multiple partitions named [`Self::DuplicatePartition::name`].
    #[error("Attempted to make multiple partitions named {name:?}.")]
    DuplicatePartition {
        /// The name of the duplicate partition.
        name: String
    },
    /// Returned when attempting to assign [`Self::ElementAlreadyInPartition::element`] to partition [`Self::ElementAlreadyInPartition::tried_putting_in`] when it's already in partition [`Self::ElementAlreadyInPartition::already_in`].
    #[error("Attempted to assign element {element:?} to partition {tried_putting_in:?} when it's already in partition {already_in:?}.")]
    ElementAlreadyInPartition {
        /// The element.
        element: Option<String>,
        /// The partition [`Self::ElementAlreadyInPartition::element`] was already in.
        already_in: Arc<str>,
        /// The partition you tried to put [`Self::ElementAlreadyInPartition::element`] in.
        tried_putting_in: Arc<str>
    }
}

impl Partitioning {
    /// Collects an iterator of `(String, Vec<String>)` into a [`Partitioning`].
    /// # Errors
    /// If multiple partitions have the same name (the first value in the tuple), returns the error [`MakePartitioningError::DuplicatePartition`].
    ///
    /// If multiple partitions contain the same element (the elements in the [`Vec`] in the second value of the tuple), returns the error [`MakePartitioningError::ElementAlreadyInPartition`].
    pub fn try_from_iter<I: IntoIterator<Item = (String, Vec<Option<String>>)>>(iter: I) -> Result<Self, MakePartitioningError> {
        let mut ret = Partitioning {
            map: HashMap::new(),
            if_none: None
        };

        let mut partitions = HashSet::<Arc<str>>::new();

        for (partition, elements) in iter {
            let partition: Arc<str> = Arc::from(&*partition);
            if partitions.insert(partition.clone()) {
                for element in elements {
                    match element {
                        Some(element) => match ret.map.entry(element) {
                            Entry::Vacant(e) => {e.insert(partition.clone());},
                            Entry::Occupied(e) => {
                                let (element, name) = e.remove_entry();
                                Err(MakePartitioningError::ElementAlreadyInPartition {
                                    element: Some(element),
                                    already_in: name,
                                    tried_putting_in: partition.clone()
                                })?
                            }
                        },
                        None => if let Some(ref name) = ret.if_none {
                            Err(MakePartitioningError::ElementAlreadyInPartition {
                                element: None,
                                already_in: name.clone(),
                                tried_putting_in: partition.clone()
                            })?
                        } else {
                            ret.if_none = Some(partition.clone());
                        }
                    }
                }
            } else {
                return Err(MakePartitioningError::DuplicatePartition {name: partition.to_string()});
            }
        }

        Ok(ret)
    }

    /// If `element` is in `self`, return [`true`].
    pub fn contains(&self, element: Option<&str>) -> bool {
        match element {
            Some(element) => self.map.contains_key(element),
            None => self.if_none.is_some()
        }
    }

    /// If `element` is in `self`, return the partition it belongs to.
    pub fn get<'a>(&'a self, element: Option<&str>) -> Option<&'a str> {
        match element {
            Some(element) => self.map.get(element).map(|x| &**x),
            None => self.if_none.as_deref()
        }
    }
}

/// Serde helper for deserializing [`Partitioning`].
struct PartitioningVisitor;

impl<'de> Visitor<'de> for PartitioningVisitor {
    type Value = Partitioning;

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        Partitioning::try_from_iter(std::iter::from_fn(|| map.next_entry::<String, Vec<Option<String>>>().transpose()).collect::<Result<Vec<_>, _>>()?).map_err(A::Error::custom)
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Expected a map")
    }
}

impl<'de> Deserialize<'de> for Partitioning {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(PartitioningVisitor)
    }
}

impl Serialize for Partitioning {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut x = HashMap::<&str, Vec<Option<&str>>>::new();

        for (element, partition) in self.map.iter() {
            x.entry(partition).or_default().push(Some(element));
        }

        if let Some(ref partition) = self.if_none {
            x.entry(partition).or_default().push(None);
        }

        let mut serializer = serializer.serialize_map(Some(x.len()))?;

        for (name, values) in x {
            serializer.serialize_entry(name, &values)?;
        }

        serializer.end()
    }
}
