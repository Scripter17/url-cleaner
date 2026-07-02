//! [`ProfiledCleaner`].

use crate::prelude::*;

/// A [`Cleaner`] with a collection of alternate [`Params`]s.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfiledCleaner<'a> {
    /// The base [`Cleaner`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub base: Cleaner<'a>,
    /// The named profiles and their [`Params`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub named: HashMap<String, Params<'a>>,
}

impl<'a> ProfiledCleaner<'a> {
    /// An [`Iterator`] over all [`Cleaner`]s and their names.
    pub fn iter(&self) -> impl Iterator<Item = (Option<&str>, Cleaner<'_>)> {
        std::iter::once((None, self.base.borrowed()))
            .chain(self.named.iter().map(|(name, params)| {
                let mut ret = self.base.borrowed();
                ret.params = params.borrowed();
                (Some(&**name), ret)
            }))
    }

    /// Get a specific [`Cleaner`].
    pub fn get<'b>(&'b self, name: Option<&str>) -> Option<Cleaner<'b>> {
        let mut ret = self.base.borrowed();

        if let Some(name) = name {
            ret.params = self.named.get(name)?.borrowed();
        }

        Some(ret)
    }

    /// [`Cleaner::assert_suitability`] for each contained [`Cleaner`].
    pub fn assert_suitability(&self) {
        for (_, cleaner) in self.iter() {
            cleaner.assert_suitability();
        }
    }
}
