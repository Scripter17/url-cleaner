//! Helper type to fake [`Serialize`] and [`Deserialize`] impls for [`fn`] types.

use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::util::*;

/// Wrapper around a function pointer that fakes [`Serialize`] and [`Deserialize`] implementations.
/// 
/// Please note that, once it's stabilized, this will require [`T: FnPtr`](FnPtr) and that will not be considered a breaking change.
#[derive(Debug, Clone, PartialEq, Eq, Suitability)]
#[suitable(never)]
#[repr(transparent)]
pub struct FnWrapper<T>(pub T);

impl<T> std::ops::Deref for FnWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for FnWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> serde::Serialize for FnWrapper<T> {
    /// Always returns [`Err`].
    fn serialize<S: serde::ser::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
        use serde::ser::Error;
        Err(S::Error::custom("FnWrapper fakes its Serialize impl."))
    }
}

impl<'de, T> serde::Deserialize<'de> for FnWrapper<T> {
    /// Always returns [`Err`].
    fn deserialize<D: serde::de::Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        Err(D::Error::custom("FnWrapper fakes its Deserialize impl."))
    }
}
