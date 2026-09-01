//! [`Secrets`].

use std::path::Path;
use std::fs::read_to_string;

use crate::prelude::*;

/// Secret values that you don't want to expose to the world, such as API keys.
///
/// The [`std::fmt::Debug`] impl just prints `Secrets`.
#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Secrets {
    /// The vars.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: FxHashMap<String, String>,
    /// Optionally the passwords required to access cleaning.
    ///
    /// Used mainly by Site for basic account control.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub passwords: Option<FxHashSet<String>>,
}

impl std::fmt::Debug for Secrets {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Secrets")
    }
}

impl Secrets {
    /// Load [`Self`] from a JSON file.
    /// # Errors
    /// If [`read_to_string`] returns an error, that error is returned.
    ///
    /// If [`serde_json::from_str`] returns an error, that error is returned.
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Self, LoadSecretsError> {
        Ok(serde_json::from_str(&read_to_string(path)?)?)
    }

    /// If [`Some`], [`Self::load`], else [`Self::default`].
    /// # Errors
    /// If [`Self::load`] returns an error, that error is returned.
    pub fn load_or_default<T: AsRef<Path>>(path: Option<T>) -> Result<Self, LoadSecretsError> {
        match path {
            Some(path) => Self::load(path),
            None       => Ok(Default::default()),
        }
    }

    /// If [`Self::passwords`] is [`Some`].
    pub fn requires_password(&self) -> bool {
        self.passwords.is_some()
    }

    /// Check if `password` is valid.
    ///
    /// If [`Self::passwords`] is [`Some`], `password` must be [`Some`] and in [`Self::passwords`].
    ///
    /// If [`Self::passwords`] is [`None`], `password` must be [`None`].
    pub fn check_password(&self, password: Option<&str>) -> bool {
        match (self.passwords.as_ref(), password) {
            (Some(passwords), Some(password)) => passwords.contains(password),
            (None, None) => true,
            _ => false
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

        self.passwords.assert_suitability(cleaner);
    }
}
