//! [`JobContext`].

use std::path::Path;
use std::fs::read_to_string;

use crate::prelude::*;

/// The context of a [`Job`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobContext {
    /// The host of the page the tasks come from.
    #[serde(default, skip_serializing_if = "is_default", deserialize_with = "deserialize_owned_url_host")]
    pub source_host: Option<Host<'static>>,
    /// The flags to use.
    ///
    /// Defaults to an empty [`HashSet`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// The vars to use.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}

/// Deserialize an owned [`Host`].
fn deserialize_owned_url_host<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Host<'static>>, D::Error> {
    Ok(<Option<Host>>::deserialize(deserializer)?.map(Host::into_owned))
}

impl JobContext {
    /// Load [`Self`] from a JSON file.
    /// # Errors
    /// If the call to [`read_to_string`] returns an error, that error is returned.
    ///
    /// If the call to [`serde_json::from_str`] returns an error, that error is returned.
    pub fn load<T: AsRef<Path>>(path: T) -> Result<(String, JobContext), LoadJobContextError> {
        let string = read_to_string(path)?;
        let job_context = serde_json::from_str(&string)?;
        Ok((string, job_context))
    }

    /// Either [`Self::load`] or [`Default::default`].
    /// # Errors
    /// If the call to [`Self::load`] returns an error that error is returned.
    pub fn load_or_default<T: AsRef<Path>>(path: Option<T>) -> Result<(Cow<'static, str>, JobContext), LoadJobContextError> {
        match path {
            Some(path) => {let (x, y) = Self::load(path)?; Ok((x.into(), y))},
            None       => Ok(("{}".into(), Default::default())),
        }
    }
}

impl Suitability for JobContext {
    fn assert_suitability(&self, cleaner: &Cleaner<'_>) {
        for name in self.flags.iter() {assert!(cleaner.docs.job_context.flags.contains_key(name), "Undocumented JobContext Flag {name:?}");}

        for (name, value) in self.vars.iter() {
            match cleaner.docs.job_context.vars.get(name) {
                Some(doc) => {
                    if let Some(variants) = &doc.variants && !variants.contains_key(value) {
                        panic!("JobContext Var {name:?} set to undocumented value {value:?}.");
                    }
                },
                None => panic!("Undocumented JobContext Var {name:?}.")
            }
        }
    }
}
