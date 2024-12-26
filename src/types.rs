//! The core tools of URL Cleaner.

use std::fmt::Debug;
use std::collections::HashMap;

use thiserror::Error;
#[expect(unused_imports, reason = "Doc comment.")]
use serde::{Serialize, Deserialize};

pub mod url_part;
pub use url_part::*;
pub mod config;
pub use config::*;
pub mod tests;
pub use tests::*;
pub mod rules;
pub use rules::*;
pub mod string_location;
pub use string_location::*;
pub mod string_modification;
pub use string_modification::*;
pub mod string_source;
pub use string_source::*;
pub mod string_matcher;
pub use string_matcher::*;
pub mod char_matcher;
pub use char_matcher::*;
pub mod jobs;
pub use jobs::*;
pub mod stop_loop_condition;
pub use stop_loop_condition::*;

/// Wrapper around a function pointer that fakes [`Serialize`] and [`Deserialize`] implementations.
/// 
/// Please note that, once it's stabilized, this will require [`T: FnPtr`](FnPtr) and that will not be considered a breaking change.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
#[cfg(feature = "custom")]
pub struct FnWrapper<T>(pub T);

#[cfg(feature = "custom")]
impl<T> std::ops::Deref for FnWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "custom")]
impl<T> std::ops::DerefMut for FnWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "custom")]
impl<T> serde::Serialize for FnWrapper<T> {
    /// Always returns [`Err`].
    fn serialize<S: serde::ser::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
        use serde::ser::Error;
        Err(S::Error::custom("FnWrapper fakes its Serialize impl."))
    }
}

#[cfg(feature = "custom")]
impl<'de, T> serde::Deserialize<'de> for FnWrapper<T> {
    /// Always returns [`Err`].
    fn deserialize<D: serde::de::Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        Err(D::Error::custom("FnWrapper fakes its Deserialize impl."))
    }
}
