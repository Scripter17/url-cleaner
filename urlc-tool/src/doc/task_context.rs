//! [`TaskContextDocs`].

use super::prelude::*;

/// A Cleaner's TaskContext docs.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskContextDocs {
    /// The flags.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] pub flags: Vec<(String, String)>,
    /// The vars.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] pub vars : Vec<(String, String)>,
}

impl TaskContextDocs {
    /// Check if there's anything to print.
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty() && self.vars.is_empty()
    }

    /// Print the contents.
    pub fn print(&self) {
        println!("## Task context");
        println!();

        if !self.flags.is_empty() {
            println!("### Flags");
            println!();
            for (name, desc) in &self.flags {
                println!("- `{name}`: {desc}");
            }
            println!();
        }

        if !self.vars .is_empty() {
            println!("### Vars" );
            println!();
            for (name, desc) in &self.vars  {
                println!("- `{name}`: {desc}");
            }
            println!();
        }
    }
}
