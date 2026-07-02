//! [`List`].

use crate::prelude::*;

mod source;
mod iter;
mod into_iter;

pub use source::*;
pub use iter::*;
pub use into_iter::*;

/// A [`Vec`] of [`String`]s.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct List<T>(pub Vec<T>);

impl<T> Deref for List<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> From<List<T>> for Vec<T> {
    fn from(value: List<T>) -> Self {
        value.0
    }
}
