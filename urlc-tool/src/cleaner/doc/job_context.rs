//! [`JobContextDocs`].

use super::prelude::*;

impl Print for JobContextDocs {
    /// Print the contents if there's anything to print.
    fn print(&self, _: &Cleaner<'_>) {
        if is_default(self) {
            return;
        }

        println!("## Job context");
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
            for (name, VarDoc {desc, required, unset, variants}) in &self.vars {
                println!("- `{name}`: {desc}");
                println!("  - Required: {required}.");
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
    }
}
