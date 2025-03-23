//! Effectively a way to query multiple [`HashSet`]s at once.

use std::collections::{HashMap, hash_map::Entry};
use std::sync::Arc;
#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::collections::HashSet;

use serde::{Serialize, Deserialize, ser::{Serializer, SerializeMap}, de::{Visitor, MapAccess, Deserializer, Error}};

use crate::types::*;
use crate::util::*;

/// A [`NamedPartitioning`] effectively allows you to query multiple [`HashSet`]s at once and finding which one an element belongs to.
///
/// Semantically, this is done by joining the sets into one and partitioning them into regions. Technically this is just a fancy [`HashMap`] with basic optimizations.
///
/// Unfortunately (or fortunately depending on yuor use case) this does have the limitation that a value cannot be in multiple partitions at once.
///
/// For the math end of this idea, see [this Wikipedia article](https://en.wikipedia.org/wiki/Partition_of_a_set).
#[derive(Debug, Clone, PartialEq, Eq, Suitability)]
pub struct NamedPartitioning {
    /// The map from values to their partitions.
    map: HashMap<String, Arc<str>>,
    /// The list of partition names.
    ///
    /// Used for serialization.
    partition_names: Vec<Arc<str>>
}

/// Serde helper for deserializing [`NamedPartitioning`].
struct NamedPartitioningVisitor;

impl<'de> Visitor<'de> for NamedPartitioningVisitor {
    type Value = NamedPartitioning;

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut ret = NamedPartitioning {
            map: HashMap::new(),
            partition_names: Vec::new()
        };

        while let Some((k, vs)) = map.next_entry::<String, Vec<String>>()? {
            let partition_name: Arc<str> = Arc::from(&*k);
            if ret.partition_names.iter().any(|x| **x == *partition_name) {Err(A::Error::custom(format!("Duplicate partition name: {partition_name}")))?;}
            for v in vs {
                match ret.map.entry(v) {
                    Entry::Vacant(e) => {e.insert(partition_name.clone());},
                    Entry::Occupied(e) => Err(A::Error::custom(format!("Attempted to assign element {:?} to partitions {:?} and {:?}", e.key(), e.get(), partition_name)))?
                }
            }
            ret.partition_names.push(partition_name)
        }

        Ok(ret)
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

impl NamedPartitioning {
    /// If `element`] is in `self`, return the partition it belongs to.
    pub fn get_partition<'a>(&'a self, element: &str) -> Option<&'a str> {
        self.map.get(element).map(|x| &**x)
    }

    /// The list of partition names.
    pub fn partition_names(&self) -> &[Arc<str>] {
        &self.partition_names
    }
}
