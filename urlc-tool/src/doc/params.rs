//! [`ParamsDocs`].

use super::prelude::*;

/// A Params's docs.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParamsDocs {
    /// The flags.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] pub flags        : Vec<(String, String)>,
    /// The vars.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] pub vars         : Vec<(String, VarDoc)>,
    /// The sets.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] pub sets         : Vec<(String, String)>,
    /// The lists.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] pub lists        : Vec<(String, String)>,
    /// The maps.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] pub maps         : Vec<(String, String)>,
    /// The partitionings.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] pub partitionings: Vec<(String, String)>
}

impl ParamsDocs {
    /// Check if there's anything to print.
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty() &&
            self.vars.is_empty() &&
            self.sets.is_empty() &&
            self.lists.is_empty() &&
            self.maps.is_empty() &&
            self.partitionings.is_empty()
    }

    /// Print the contents.
    pub fn print(&self, cleaner: &RelevantCleaner) {
        println!("## Params");
        println!();

        if !self.flags.is_empty() {
            println!("### Flags");
            println!();
            for (name, desc) in &self.flags {
                println!("- `{name}`: {desc}");
            }
            println!();
        }

        if !self.vars.is_empty() {
            println!("### Vars");
            println!();
            println!("Please note that the presence of required vars and validity of varianted vars is only checked when asserting suitability.");
            println!();
            println!("Cleaners that break the \"invariants\" here can be parsed as used, but will likely exhibit unintended behavior.");
            println!();
            for (name, VarDoc {desc, required, unset, variants}) in &self.vars {
                println!("- `{name}`: {desc}");
                println!("  - Required: {required}.");
                if let Some(default) = cleaner.params.vars.get(name) {
                    println!("  - Default: `{default}`.");
                }
                if let Some(desc) = unset {
                    println!("  - Unset: {desc}");
                }
                for (name, desc) in variants {
                    println!("  - `{name}`: {desc}");
                }
            }
            println!();
        }

        if !self.sets.is_empty() {
            println!("### Sets");
            println!();
            for (name, desc) in &self.sets {
                println!("- `{name}`: {desc}");
            }
            println!();
        }

        if !self.lists.is_empty() {
            println!("### Lists");
            println!();
            for (name, desc) in &self.lists {
                println!("- `{name}`: {desc}");
            }
            println!();
        }

        if !self.maps.is_empty() {
            println!("### Maps");
            println!();
            for (name, desc) in &self.maps {
                println!("- `{name}`: {desc}");
            }
            println!();
        }

        if !self.partitionings.is_empty() {
            println!("### Partitionings");
            println!();
            for (name, desc) in &self.partitionings {
                println!("- `{name}`: {desc}");
            }
            println!();
        }
    }
}
