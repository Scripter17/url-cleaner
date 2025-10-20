//! [`UnprofiledCleaner`].

use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// A [`Cleaner`] with no [`Params`]. Primarily used internally by [`ProfiledCleaner`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnprofiledCleaner<'a> {
    /// The documentation.
    ///
    /// Defaults to an empty [`CleanerDocs`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: Cow<'a, CleanerDocs>,
    /// Basically functions.
    ///
    /// Defaults to an empty [`Commons`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub commons: Cow<'a, Commons>,
    /// The [`Action`]s to apply.
    ///
    /// Defaults to an empty [`Vec`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub actions: Cow<'a, [Action]>
}

impl<'a> UnprofiledCleaner<'a> {
    /// Create a new [`Self`] that [`Cow::Borrowed`]s all fields.
    pub fn borrowed(&'a self) -> Self {
        Self {
            docs   : Cow::Borrowed(&*self.docs),
            commons: Cow::Borrowed(&*self.commons),
            actions: Cow::Borrowed(&*self.actions)
        }
    }

    /// Make a [`Cleaner`] borrowing each field of `self` and using the specified [`Params`].
    pub fn with_profile(self, profile: Profile<'a>) -> Cleaner<'a> {
        Cleaner {
            docs   : self.docs,
            params : profile.into(),
            commons: self.commons,
            actions: self.actions
        }
    }
}
