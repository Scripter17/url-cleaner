//! [`VarDoc`].

use crate::prelude::*;

use indexmap::IndexMap;

/// Documentation for a var.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VarDoc {
    /// The description.
    pub desc: String,
    /// The behavior if it's unset.
    pub unset: Option<String>,
    /// If the var is required.
    pub required: bool,
    /// If [`Some`], the finite set of set values the var can have.
    ///
    /// If [`None`] is a valid variant, see [`Self::unset`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub variants: Option<IndexMap<String, String>>,
}

impl Suitability for VarDoc {
    fn assert_suitability(&self, _: &Cleaner<'_>) {
        match (self.required, self.unset.is_some()) {
            (false, false) => {},
            (false, true ) => {},
            (true , false) => {},
            (true , true ) => panic!("Required Var can't have unset behavior."),
        }
    }
}
