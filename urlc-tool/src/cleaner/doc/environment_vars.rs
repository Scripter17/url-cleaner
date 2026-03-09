//! [`EnvironmentVarsDocs`].

use super::prelude::*;

impl Print for EnvironmentVarsDocs {
    /// Print the contents if there's anything to print.
    fn print(&self, _: &Cleaner<'_>) {
        if is_default(self) {
            return;
        }

        println!("## Environment Vars");
        println!();
        for (name, desc) in &self.0 {
            println!("- `{name}`: {desc}");
        }
        println!();
    }
}
