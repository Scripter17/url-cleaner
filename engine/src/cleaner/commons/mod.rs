//! [`Commons`] and co.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::prelude::*;

pub mod common_call;
pub use common_call::*;
pub mod common_call_args;
pub use common_call_args::*;
pub mod common_call_args_config;
pub use common_call_args_config::*;

/// Common snippets used throughout a [`Cleaner::actions`].
///
/// For example, an [`Action`] for removing universal tracking parameters before both expanding redirects and returning the final URL.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Commons {
    /// Common [`Condition`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub conditions: HashMap<String, Condition>,
    /// Common [`Action`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub actions: HashMap<String, Action>,
    /// Common [`StringSource`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_sources: HashMap<String, StringSource>,
    /// Common [`StringModification`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_modifications: HashMap<String, StringModification>,
    /// Common [`StringMatcher`]s.
    #[serde(default, skip_serializing_if = "is_default")]
    pub string_matchers: HashMap<String, StringMatcher>
}
