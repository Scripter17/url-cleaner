//! [`Partitioning`].

use std::sync::Arc;

use crate::prelude::*;

mod source;
pub use source::*;

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
#[derive(Debug, Default, Clone, PartialEq, Eq, Suitability)]
pub struct Partitioning(pub Map<Arc<str>>);

impl Partitioning {
    /// If `element` is in `self`, return [`true`].
    pub fn contains(&self, element: Option<&str>) -> bool {
        self.0.contains(element)
    }

    /// If `element` is in `self`, return the partition it belongs to.
    pub fn get<'a>(&'a self, element: Option<&str>) -> Option<&'a str> {
        self.0.get(element).map(|x| &**x)
    }
}

impl FromIterator<(String, Vec<Option<String>>)> for Partitioning {
    fn from_iter<I: IntoIterator<Item = (String, Vec<Option<String>>)>>(iter: I) -> Self {
        let mut ret = Self::default();
        ret.extend(iter);
        ret
    }
}

impl Extend<(String, Vec<Option<String>>)> for Partitioning {
    fn extend<I: IntoIterator<Item = (String, Vec<Option<String>>)>>(&mut self, iter: I) {
        for (name, items) in iter {
            let name = Arc::<str>::from(name);

            for item in items {
                self.0.insert(item, name.clone());
            }
        }
    }
}

/// Serde helper for deserializing [`Partitioning`].
struct PartitioningVisitor;

impl<'de> Visitor<'de> for PartitioningVisitor {
    type Value = Partitioning;

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut ret = Self::Value::default();

        while let Some(key) = map.next_key::<String>()? {
            map.next_value_seed(ExtendPartitioning {
                partitioning: &mut ret,
                name: Arc::from(key),
            })?;
        }

        Ok(ret)
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Expected a map")
    }
}

/// A thing to allow deserializing [`Partitioning`]s more efficiently.
struct ExtendPartitioning<'a> {
    /// The [`Partitioning`] being deserialized.
    partitioning: &'a mut Partitioning,
    /// The name of the current partition.
    name: Arc<str>,
}

impl<'de, 'a> serde::de::DeserializeSeed<'de> for ExtendPartitioning<'a> {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_seq(self)
    }
}

impl<'de, 'a> Visitor<'de> for ExtendPartitioning<'a> {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "an array of strings")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        while let Some(x) = seq.next_element()? {
            self.partitioning.0.insert(x, self.name.clone());
        }

        Ok(())
    }
}

impl<'de> Deserialize<'de> for Partitioning {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(PartitioningVisitor)
    }
}

impl Serialize for Partitioning {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut x = FxHashMap::<&str, Vec<Option<&str>>>::with_hasher(Default::default());

        for (element, partition) in &self.0 {
            x.entry(partition).or_default().push(element.map(|x| &**x));
        }

        let mut serializer = serializer.serialize_map(Some(x.len()))?;

        for (name, values) in x {
            serializer.serialize_entry(name, &values)?;
        }

        serializer.end()
    }
}
