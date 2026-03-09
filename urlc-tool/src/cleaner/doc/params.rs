//! [`ParamsDocs`].

use super::prelude::*;

impl Print for ParamsDocs {
    /// Print the contents if there's anything to print.
    fn print(&self, cleaner: &Cleaner<'_>) {
        if is_default(self) {
            return;
        }

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
            for (name, VarDoc {desc, required, unset, variants}) in &self.vars {
                println!("- `{name}`: {desc}");
                println!("  - Required: {required}.");
                if let Some(default) = cleaner.params.vars.get(name) {
                    println!("  - Default: `{default}`.");
                }
                if let Some(desc) = unset {
                    println!("  - Unset: {desc}");
                }
                if let Some(variants) = variants {
                    for (name, desc) in variants {
                        println!("  - `{name}`: {desc}");
                    }
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
