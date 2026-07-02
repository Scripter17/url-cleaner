//! [`Cleaner`] and co.

use std::fs::read_to_string;
use std::path::Path;
#[cfg(feature = "bundled-cleaner")]
use std::sync::OnceLock;

use crate::prelude::*;

mod profiled;
mod profiles_config;
mod params;
mod params_diff;
mod docs;

pub use profiled::*;
pub use profiles_config::*;
pub use params::*;
pub use params_diff::*;
pub use docs::*;

/// The main unit describing how to clean URLs.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct Cleaner<'a> {
    /// The [`Docs`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: Cow<'a, Docs>,
    /// The [`Params`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub params: Params<'a>,
    /// The [`Functions`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub functions: Cow<'a, Functions>,
    /// The [`Action`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub action: Cow<'a, Action>
}

/// The JSON text of the bundled cleaner.
///
/// Please see [`Cleaner::get_bundled`] and co.
#[cfg(all(feature = "bundled-cleaner", not(test)))]
pub const BUNDLED_CLEANER_STR: &str = include_str!(concat!(env!("OUT_DIR"), "/bundled-cleaner.json.minified"));

/// The JSON text of the bundled cleaner.
///
/// Please see [`Cleaner::get_bundled`] and co.
#[cfg(all(feature = "bundled-cleaner", test))]
pub const BUNDLED_CLEANER_STR: &str = include_str!("bundled-cleaner.json");

/// The cached deserialization of the bundled cleaner.
#[cfg(feature = "bundled-cleaner")]
static BUNDLED_CLEANER: OnceLock<Cleaner<'static>> = OnceLock::new();

impl<'a> Cleaner<'a> {
    /// Load [`Self`] from a JSON file.
    /// # Errors
    /// If the call to [`read_to_string`] returns an error, that error is returned.
    ///
    /// If the call to [`serde_json::from_str`] returns an error, that error is returned.
    pub fn load<T: AsRef<Path>>(path: T) -> Result<(String, Cleaner<'static>), LoadCleanerError> {
        let string = read_to_string(path)?;
        let cleaner = serde_json::from_str(&string)?;
        Ok((string, cleaner))
    }



    /// Make a new instance of the bundled cleaner.
    /// # Errors
    /// If the call to [`serde_json::from_str`] returns an error, that error is returned.
    #[cfg(feature = "bundled-cleaner")]
    pub fn new_bundled() -> Result<Cleaner<'static>, LoadCleanerError> {
        Ok(serde_json::from_str(BUNDLED_CLEANER_STR)?)
    }

    /// Gets the cached bundled cleaner, parsing it if not already cached.
    /// # Errors
    /// If the call to [`Self::new_bundled`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use url_cleaner_engine::prelude::*;
    ///
    /// let (_, cleaner) = Cleaner::get_bundled().unwrap();
    ///
    /// cleaner.assert_suitability();
    /// ```
    #[cfg(feature = "bundled-cleaner")]
    pub fn get_bundled() -> Result<(&'static str, &'static Cleaner<'static>), LoadCleanerError> {
        if let Some(cleaner) = BUNDLED_CLEANER.get() {
            Ok((BUNDLED_CLEANER_STR, cleaner))
        } else {
            let cleaner = Self::new_bundled()?;
            Ok((BUNDLED_CLEANER_STR, BUNDLED_CLEANER.get_or_init(|| cleaner)))
        }
    }



    /// Either [`Self::load`] or [`Self::new_bundled`].
    /// # Errors
    /// If the call to [`Self::load`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::new_bundled`] returns an error, that error is returned.
    #[cfg(feature = "bundled-cleaner")]
    pub fn load_or_new_bundled<T: AsRef<Path>>(path: Option<T>) -> Result<(Cow<'static, str>, Cleaner<'static>), LoadCleanerError> {
        match path {
            Some(path) => Self::load(path).map(|(s, c)| (s.into(), c)),
            None       => Ok((BUNDLED_CLEANER_STR.into(), Self::new_bundled()?)),
        }
    }

    /// Either [`Self::load`] or [`Self::get_bundled`].
    /// # Errors
    /// If the call to [`Self::load`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::get_bundled`] returns an error, that error is returned.
    #[cfg(feature = "bundled-cleaner")]
    pub fn load_or_get_bundled<T: AsRef<Path>>(path: Option<T>) -> Result<(Cow<'static, str>, Cow<'static, Cleaner<'static>>), LoadCleanerError> {
        Ok(match path {
            Some(path) => {let (x, y) = Self::load       (path)?; (Cow::Owned   (x), Cow::Owned   (y))},
            None       => {let (x, y) = Self::get_bundled(    )?; (Cow::Borrowed(x), Cow::Borrowed(y))},
        })
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Cleaner<'_> {
        Cleaner {
            docs     : Cow::Borrowed(&*self.docs             ),
            params   :                 self.params.borrowed() ,
            functions: Cow::Borrowed(&*self.functions        ),
            action   : Cow::Borrowed(&*self.action           ),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Cleaner<'static> {
        Cleaner {
            docs     : Cow::Owned(self.docs     .into_owned()),
            params   :            self.params   .into_owned() ,
            functions: Cow::Owned(self.functions.into_owned()),
            action   : Cow::Owned(self.action   .into_owned()),
        }
    }

    /// Asserts the suitability of `self` to be URL Cleaner's bundled cleaner.
    ///
    /// Exact behavior is unspecified and changes are not considered breaking.
    /// # Panics
    /// If `self` is deemed unsuitable to be URL Cleaner's bundled cleaner, panics.
    pub fn assert_suitability(&self) {
        Suitability::assert_suitability(self, self)
    }
}

impl<'j> Cleaner<'j> {
    /// [`Action::apply`].
    /// # Errors
    /// If the call to [`Action::apply`] returns an error, that error is returned.
    pub fn apply(&'j self, task_state: &mut TaskState<'j>) -> Result<bool, ApplyCleanerError> {
        Ok(self.action.apply(task_state, None)?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "bundled-cleaner")]
    fn minificiation_test() {
        use super::*;

        let minified   = include_str!(concat!(env!("OUT_DIR"), "/bundled-cleaner.json.minified"));
        let unminified = include_str!("bundled-cleaner.json");

        assert_eq!(
            serde_json::from_str::<Cleaner>(minified  ).expect("Deserializing the minified bundled cleaner to work"),
            serde_json::from_str::<Cleaner>(unminified).expect("Deserializing the unminified bundled cleaner to work"),
            "The minified version was improperly made."
        );

        assert!(minified.len() < unminified.len(), "The minified version isn't minified???");
    }
}
