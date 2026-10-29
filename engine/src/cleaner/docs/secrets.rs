//! [`SecretsDocs`].

use crate::prelude::*;

use indexmap::IndexMap;

/// Documentation for a [`Job`]'s [`Secrets`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct SecretsDocs {
    /// [`Secrets::vars`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: IndexMap<String, VarDoc>,
}
