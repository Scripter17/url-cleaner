//! [`ProfilesConfig`].

use std::path::Path;
use std::fs::read_to_string;

use crate::prelude::*;

/// A config to turn a [`Cleaner`] into a [`ProfiledCleaner`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfilesConfig {
    /// The base [`ParamsDiff`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub base: ParamsDiff,
    /// The named profiles to apply on top of [`Self::base`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub named: HashMap<String, ParamsDiff>,
}

impl ProfilesConfig {
    /// Load [`Self`] from a JSON file.
    /// # Errors
    /// If the call to [`read_to_string`] returns an error, that error is returned.
    ///
    /// If the call to [`serde_json::from_str`] returns an error, that error is returned.
    pub fn load<T: AsRef<Path>>(path: T) -> Result<(String, Self), LoadProfilesConfigError> {
        let string = read_to_string(path)?;
        let config = serde_json::from_str(&string)?;
        Ok((string, config))
    }

    /// Either [`Self::load`] or [`Default::default`].
    /// # Errors
    /// If the call to [`Self::load`] returns an error, that error is returned.
    pub fn load_or_default<T: AsRef<Path>>(path: Option<T>) -> Result<(Cow<'static, str>, Self), LoadProfilesConfigError> {
        Ok(match path {
            Some(path) => {
                let (string, config) = Self::load(path)?;
                (string.into(), config)
            },
            None => ("{}".into(), Default::default())
        })
    }

    /// Make a [`ProfiledCleaner`].
    pub fn make<'a>(self, cleaner: &'a Cleaner<'_>) -> ProfiledCleaner<'a> {
        let mut named = HashMap::with_capacity(self.named.len());

        let mut base = cleaner.borrowed();
        self.base.apply(&mut base.params);

        for (name, diff) in self.named {
            let mut params = base.params.clone();
            diff.apply(&mut params);
            named.insert(name, params);
        }

        ProfiledCleaner {
            base,
            named
        }
    }
}
