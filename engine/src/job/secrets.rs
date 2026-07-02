//! [`Secrets`].

use std::path::Path;
use std::fs::read_to_string;

use crate::prelude::*;

/// Secret values that you don't want to expose to the world, such as API keys.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Secrets {
    /// The vars.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// The [`AuthInfo`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub auth_info: AuthInfo,
}

impl Secrets {
    /// Load [`Self`] from a JSON file.
    /// # Errors
    /// If the call to [`read_to_string`] returns an error, that error is returned.
    ///
    /// If the call to [`serde_json::from_str`] returns an error, that error is returned.
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Self, LoadSecretsError> {
        Ok(serde_json::from_str(&read_to_string(path)?)?)
    }

    /// If [`Some`], [`Self::load`], else [`Self::default`].
    /// # Errors
    /// If the call to [`Self::load`] returns an error, that error is returned.
    pub fn load_or_default<T: AsRef<Path>>(path: Option<T>) -> Result<Self, LoadSecretsError> {
        match path {
            Some(path) => Self::load(path),
            None       => Ok(Default::default()),
        }
    }
}

impl Suitability for Secrets {
    fn assert_suitability(&self, cleaner: &Cleaner<'_>) {
        for (name, value) in self.vars.iter() {
            match cleaner.docs.secrets.vars.get(name) {
                Some(doc) => {
                    if let Some(variants) = &doc.variants && !variants.contains_key(value) {
                        panic!("Secrets Var {name:?} set to undocumented value {value:?}.");
                    }
                },
                None => panic!("Undocumented Secrets Var {name:?}.")
            }
        }

        self.auth_info.assert_suitability(cleaner);
    }
}
