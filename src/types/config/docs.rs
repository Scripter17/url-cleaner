//! Allows placing documentation right into a Config.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[allow(unused_imports, reason = "Used in a doc comment.")]
use crate::types::*;
use crate::util::*;

/// In-config documentation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigDocs {
    /// The basic description of the config.
    #[serde(default, skip_serializing_if = "is_default")]
    pub description: Option<Vec<String>>,
    /// The descriptions of the [`Params::flags`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashMap<String, String>,
    /// The descriptions of the [`Params::vars`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// The descriptions of the environment variables used.
    #[serde(default, skip_serializing_if = "is_default")]
    pub environment_vars: HashMap<String, String>,
    /// The descriptions of the [`Params::sets`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: HashMap<String, String>,
    /// The descriptions of the [`Params::lists`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: HashMap<String, String>,
    /// The descriptions of the [`Params::maps`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: HashMap<String, String>
}

impl ConfigDocs {
    /// Generates markdown. Used for generating the README.
    pub fn to_markdown(&self) -> String {
        let mut ret = String::new();

        ret.push_str("#### Flags\n\n");
        ret.push_str("Flags let you toggle behavior, for example to replace x.com links with vxtwitter.com links for discord embeds..\n\n");
        ret.push_str(&self.flags.iter().map(|(k, v)| format!("- `{k}`: {v}\n")).collect::<Vec<String>>().join("\n"));
        ret.push_str("Flags can be set via command line arguments with `--flag flag1 --flag flag2`.\n\n");
        ret.push_str("If a flag is enabled in a config's `params` field, it can be disabled using `--unflag flag1 --unflag flag1`.\n\n");

        ret.push_str("#### Variables\n\n");
        ret.push_str("Variables let you choose what strings certain rules should use. For example choosing which BreezeWiki frontend to use..\n\n");
        ret.push_str(&self.vars.iter().map(|(k, v)| format!("- `{k}`: {v}\n")).collect::<Vec<String>>().join("\n"));
        ret.push_str("Variables can be set via command line arguments with `--var name1 value1 --var name2 value2`.\n\n");
        ret.push_str("If a variable is specified in a config's `params` field, it can be unspecified using `--unvar var1 --unvar var2`.\n\n");

        ret.push_str("#### Environment variables\n\n");
        ret.push_str("Environment variables let you not need to repeatedly specify variables and keep things like API keys out of configs.");
        ret.push_str(&self.environment_vars.iter().map(|(k, v)| format!("- `{k}`: {v}\n")).collect::<Vec<String>>().join("\n"));

        ret.push_str("#### Sets\n\n");
        ret.push_str("Sets let you check if a string is one of many specific strings very quickly.\n\n");
        ret.push_str(&self.sets.iter().map(|(k, v)| format!("- `{k}`: {v}\n")).collect::<Vec<String>>().join("\n"));
        ret.push_str("Sets can have elements inserted into them using `--insert-into-set name1 value1 value2 --insert-into-set name2 value3 value4`.\n\n");
        ret.push_str("Sets can have elements removed  from them using `--remove-from-set name1 value1 value2 --remove-from-set name2 value3 value4`.\n\n");

        ret.push_str("#### Lists\n\n");
        ret.push_str("Lists allow you to iterate over strings for things like checking if another string contains any of them.\n\n");
        ret.push_str(&self.lists.iter().map(|(k, v)| format!("- `{k}`: {v}\n")).collect::<Vec<String>>().join("\n"));
        ret.push_str("Currently there is no command line syntax for them. There really should be.\n");

        ret
    }
}
