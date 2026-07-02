//! [`RegexConfig`].

#[expect(unused_imports, reason = "Used in doc comments.")]
use regex::RegexBuilder;

use crate::prelude::*;

/// Configuration given to [`RegexBuilder`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct RegexConfig {
    /// The line terminator to use.
    ///
    /// Defaults to `\n`'.
    #[serde(default = "nlu8", skip_serializing_if = "is_nlu8")]
    pub line_terminator: u8,
    /// The [`RegexFlags`].
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: RegexFlags,
}

/** Serde helper. **/ const fn    nlu8(      ) -> u8   {      b'\n'}
/** Serde helper. **/ const fn is_nlu8(x: &u8) -> bool {*x == b'\n'}

impl Default for RegexConfig {
    fn default() -> Self {
        Self {
            line_terminator: nlu8(),
            flags: Default::default()
        }
    }
}
