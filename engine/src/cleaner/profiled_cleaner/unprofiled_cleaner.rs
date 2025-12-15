//! [`UnprofiledCleaner`].

use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// A [`Cleaner`] with no [`Params`].
///
/// Mainly used in [`ProfiledCleaner`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnprofiledCleaner<'a> {
    /// The documentation.
    ///
    /// Defaults to an empty [`Docs`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: Cow<'a, Docs>,
    /// Basically functions.
    ///
    /// Defaults to an empty [`Functions`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub functions: Cow<'a, Functions>,
    /// The [`Action`]s to apply.
    ///
    /// Defaults to an empty [`Vec`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub action: Cow<'a, Action>
}

impl<'a> UnprofiledCleaner<'a> {
    /// Create a new [`Self`] that [`Cow::Borrowed`]s all fields.
    pub fn borrowed(&'a self) -> Self {
        Self {
            docs   : Cow::Borrowed(&*self.docs),
            functions: Cow::Borrowed(&*self.functions),
            action: Cow::Borrowed(&*self.action)
        }
    }

    /// Make a [`Cleaner`] borrowing each field of `self` and using the specified [`Params`].
    pub fn with_profile(self, profile: Profile<'a>) -> Cleaner<'a> {
        Cleaner {
            docs   : self.docs,
            params : profile.into(),
            functions: self.functions,
            action: self.action
        }
    }
}
