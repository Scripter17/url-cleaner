//! [`SecretsDocs`].

use crate::prelude::*;

use indexmap::IndexMap;

/// Documentation for a [`Job`]'s [`Secrets`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub struct SecretsDocs {
    /// [`Secrets::vars`].
    pub vars: IndexMap<String, VarDoc>,
}
