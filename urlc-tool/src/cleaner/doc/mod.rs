//! Doc.

pub mod params;
pub mod job_context;
pub mod task_context;
pub mod environment_vars;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;

    pub use super::Print;
}

use prelude::*;

/// Generate a markdown document of a Cleaner's docs.
#[derive(Debug, Parser)]
pub struct Args {
    /// The Cleaner to document.
    #[arg(long)]
    pub cleaner: Option<PathBuf>
}

/// Print the thing as markdown.
pub trait Print {
    /// Print the thing as markdown.
    fn print(&self, cleaner: &Cleaner<'_>);
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let cleaner = Cleaner::load_or_get_bundled(self.cleaner).unwrap();

        cleaner.docs.print(&cleaner);
    }
}

impl Print for Docs {
    fn print(&self, cleaner: &Cleaner<'_>) {
        println!("# {}", self.name.as_deref().unwrap_or("Unnamed Cleaner"));
        println!();

        if let Some(description) = &self.description {
            for line in description {
                println!("{line}");
                println!();
            }
        }

        self.params          .print(cleaner);
        self.job_context     .print(cleaner);
        self.task_context    .print(cleaner);
        self.environment_vars.print(cleaner);
    }
}
