//! [`Params`] and co.

use std::collections::{HashMap, HashSet};
use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use crate::prelude::*;

pub mod diff;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::diff::*;

    pub use super::Params;
}

/// A data store to fine tune the behavior of [`Cleaner`]s.
///
/// Has two main functions:
///
/// 1. [`Self::flags`] and [`Self::vars`] give easy ways for [`Cleaner`]s to let users enable/change behavior they only sometimes want, such as embed compatibility and unmobiling.
///
/// 2. [`Self::sets`], [`Self::lists`], [`Self::maps`], and [`Self::partitionings`] allow giving names to websites, query parameters, etc. that share a common property, such as redirect websites.
///
/// While neither have to be used for those reasons ([`ParamsDiff`]s will happily modify sets and maps), those are what those fields seem to do best.
///
/// There's also two main ways to apply situational/per-[`Job`] changes to [`Params`]:
///
/// 1. [`ProfiledCleanerConfig`] to give commonly used [`ParamsDiff`]s names.
///
/// 2. [`ParamsDiff`] for unnamed one-off changes.
///
/// For both methods, it's recommended to use [`Cleaner::borrowed`]/[`Params::borrowed`] beforehand to allow [`ParamsDiff`] to only clone data it modifies.
/// # Examples
/// ```
/// use url_cleaner_engine::prelude::*;
///
/// // Usually Params are in Cleaners, but this is example code.
///
/// let params = serde_json::from_str::<Params>(r#"
/// {
///     "sets": {
///         "redirect_websites": ["bit.ly", "t.co"]
///     }
/// }
/// "#).unwrap();
///
/// // Makes an equivalent Params where each field is just a reference to the respective field from the above Params.
///
/// let mut borrowed_params = params.borrowed();
///
/// let params_diff = serde_json::from_str::<ParamsDiff>(r#"
/// {
///     "flags": [
///         "This ParamsDiff doesn't touch the Params::sets field at all",
///         "and thus when this is applied to the borrowed Params,",
///         "won't duplicate it, saving some memory.",
///
///         "For simple example code the benefit is smaller than this explanation,",
///         "but for the bundled cleaner and the 4 profiles I use, borrowing saves 500KB."
///     ]
/// }
/// "#).unwrap();
///
/// params_diff.apply(&mut borrowed_params);
/// ```
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Params<'a> {
    /// Flags allow enabling and disabling certain behavior.
    ///
    /// Defaults to an empty [`HashSet`].
    #[serde_with = "SetPreventDuplicates<_>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: Cow<'a, HashSet<String>>,
    /// Vars allow setting strings used for certain behaviors.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: Cow<'a, HashMap<String, String>>,
    /// Sets allow quickly checking if a string is in a certain genre of possible values.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: Cow<'a, HashMap<String, Set<String>>>,
    /// Lists are a niche thing that lets you iterate over a set of values in a known order.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: Cow<'a, HashMap<String, Vec<String>>>,
    /// Maps allow mapping input values to output values.
    ///
    /// Please note that [`Map`]s make this more powerful than a normal [`HashMap`], notably including a default value.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: Cow<'a, HashMap<String, Map<String>>>,
    /// Named partitionings effectively let you check which if several sets a value is in.
    ///
    /// See [this Wikipedia article](https://en.wikipedia.org/wiki/Partition_of_a_set) for the math end of this idea.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub partitionings: Cow<'a, HashMap<String, Partitioning>>
}

impl<'a> Params<'a> {
    /// Create a new [`Self`] that [`Cow::Borrowed`]s all fields.
    ///
    /// Basically a very cheap [`Clone`] that you can apply [`ParamsDiff`]s to.
    pub fn borrowed(&'a self) -> Self {
        Self {
            flags        : Cow::Borrowed(&*self.flags),
            vars         : Cow::Borrowed(&*self.vars),
            sets         : Cow::Borrowed(&*self.sets),
            lists        : Cow::Borrowed(&*self.lists),
            maps         : Cow::Borrowed(&*self.maps),
            partitionings: Cow::Borrowed(&*self.partitionings)
        }
    }

    /// Become an owned [`Self`], cloning only what needed.
    pub fn into_owned(self) -> Params<'static> {
        Params {
            flags        : Cow::Owned(self.flags.into_owned()),
            vars         : Cow::Owned(self.vars.into_owned()),
            sets         : Cow::Owned(self.sets.into_owned()),
            lists        : Cow::Owned(self.lists.into_owned()),
            maps         : Cow::Owned(self.maps.into_owned()),
            partitionings: Cow::Owned(self.partitionings.into_owned())
        }
    }
}
