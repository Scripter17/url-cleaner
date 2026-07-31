//! [`NonEmptyList`].

use crate::prelude::*;

/// A [`List`] that can't be empty.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct NonEmptyList<T>(List<T>);

impl<T> Deref for NonEmptyList<T> {
    type Target = List<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for NonEmptyList<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let ret = List::<T>::deserialize(deserializer)?;

        if ret.is_empty() {
            Err(D::Error::custom("Can't be empty."))?;
        }

        Ok(Self(ret))
    }
}

impl<T> IntoIterator for NonEmptyList<T> {
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyList<T> {
    type IntoIter = std::slice::Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
