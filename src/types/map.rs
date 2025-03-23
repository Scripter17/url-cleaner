//! [`HashMap`] with fallbacks.

use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use serde_with::{serde_as, MapPreventDuplicates, SetPreventDuplicates};

use crate::types::*;
use crate::util::*;

#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct Map<T> {
    #[serde_as(as = "MapPreventDuplicates<_, _>")]
    pub map: HashMap<String, T>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub if_null: Option<Box<T>>,
    #[serde(default = "Option::default", skip_serializing_if = "Option::is_none")]
    pub r#else: Option<Box<T>>
}

impl<T> Map<T> {
    pub fn get<U: AsRef<str>>(&self, value: Option<U>) -> Option<&T> {
        value.map(|x| self.map.get(x.as_ref())).unwrap_or(self.if_null.as_deref()).or(self.r#else.as_deref())
    }
}

#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct MapDiff<T> {
    #[serde_as(as = "MapPreventDuplicates<_, _>")]
    #[serde(default, bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))]
    pub insert_into_map: HashMap<String, T>,
    #[serde_as(as = "SetPreventDuplicates<_>")]
    #[serde(default)]
    pub remove_from_map: HashSet<String>
}

impl<T> MapDiff<T> {
    pub fn apply(self, to: &mut Map<T>) {
        to.map.extend(self.insert_into_map);
        to.map.retain(|k, _| !self.remove_from_map.contains(k));
    }
}
