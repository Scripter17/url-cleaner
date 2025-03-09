//! Effectively a [`HashSet`] with named partitions.

use std::collections::{HashMap, hash_map::Entry};
use std::sync::Arc;
#[expect(unused_imports, reason = "Used in a doc comment.")]
use std::collections::HashSet;

use serde::{Serialize, Deserialize, ser::{Serializer, SerializeMap}, de::{Visitor, MapAccess, Deserializer, Error}};

/// Maps elements of a set to the name of the partition it belongs to.
///
/// For example, websites into the names of the company that owns them or the software they're an instance of.
/// 
/// See [the Wikipedia article](https://en.wikipedia.org/wiki/Partition_of_a_set) for precise math stuff.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedPartitioning {
    /// The maps of values to their class names.
    map: HashMap<String, Arc<str>>,
    /// The class names.
    partition_names: Vec<Arc<str>>
}

/// [`Visitor`] to [`Deserialize`] [`NamedPartitioning`]s.
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
            for v in vs {
                match ret.map.entry(v) {
                    Entry::Vacant(e) => {e.insert(partition_name.clone());},
                    Entry::Occupied(e) => Err(A::Error::custom(format!("Duplicate element: {}", e.key())))?
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
    /// Gets the name of the partition `element` belongs to.
    pub fn get_partition<'a>(&'a self, element: &str) -> Option<&'a str> {
        self.map.get(element).map(|x| &**x)
    }

    /// Gets the names of the partitions.
    ///
    /// Once <https://github.com/rust-lang/rust/pull/132553> lands, this will probably be changed to return `&[&str]`.
    pub fn partition_names(&self) -> &[Arc<str>] {
        &self.partition_names
    }
}
