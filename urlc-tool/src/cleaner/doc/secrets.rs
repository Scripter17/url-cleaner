//! [`SecretsDocs`].

use super::prelude::*;

impl Print for SecretsDocs {
    /// Print the contents if there's anything to print.
    fn print(&self, _: &Cleaner<'_>) {
        if is_default(self) {
            return;
        }

        println!("## Secrets");
        println!();
        for (name, VarDoc {desc, required, unset, variants}) in self.vars.iter() {
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
