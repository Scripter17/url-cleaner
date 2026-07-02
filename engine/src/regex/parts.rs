//! [`RegexParts`].

use regex::{Regex, RegexBuilder};

use crate::prelude::*;

/// Config on how to make a [`Regex`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[serde(remote = "Self")]
pub struct RegexParts {
    /// The regex pattern to use.
    ///
    /// For performance, validity is only checked at [`Self::build`] time.
    pub pattern: String,
    /// Config to pass into [`RegexBuilder`].
    ///
    /// Flattened.
    #[serde(flatten)]
    pub config: RegexConfig
}

crate::util::string_or_struct_magic!(RegexParts);

impl RegexParts {
    /// Compile the regex.
    /// # Errors
    /// If the call to [`RegexBuilder::build`] returns an error, that error is returned.
    pub fn build(&self) -> Result<Regex, regex::Error> {
        RegexBuilder::new(&self.pattern)
            .line_terminator     (self.config.line_terminator)
            .case_insensitive    (self.config.flags.case_insensitive    ())
            .crlf                (self.config.flags.crlf                ())
            .dot_matches_new_line(self.config.flags.dot_matches_new_line())
            .ignore_whitespace   (self.config.flags.ignore_whitespace   ())
            .multi_line          (self.config.flags.multi_line          ())
            .octal               (self.config.flags.octal               ())
            .swap_greed          (self.config.flags.swap_greed          ())
            .unicode             (self.config.flags.unicode             ())
            .build()
    }
}

impl FromStr for RegexParts {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for RegexParts {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl From<String> for RegexParts {
    fn from(value: String) -> Self {
        Self {
            pattern: value,
            config: Default::default()
        }
    }
}

impl Suitability for RegexParts {
    fn assert_suitability(&self, config: &Cleaner) {
        self.build().unwrap_or_else(|_| panic!("Regex to be buildable: {self:?}"));
        self.config.assert_suitability(config);
    }
}

impl TryFrom<&RegexParts> for Regex {
    type Error = regex::Error;

    fn try_from(value: &RegexParts) -> Result<Self, Self::Error> {
        value.build()
    }
}

impl TryFrom<RegexParts> for Regex {
    type Error = regex::Error;

    fn try_from(value: RegexParts) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}
