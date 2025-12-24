//! Doc.

pub mod params;
pub mod job_context;
pub mod task_context;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;

    pub use serde::Deserialize;
    pub use serde_with::{serde_as, Map};
    pub use std::collections::HashMap;
    
    pub use super::params::*;
    pub use super::job_context::*;
    pub use super::task_context::*;

    pub use super::{RelevantCleaner, RelevantParams, VarDoc};
}

use prelude::*;

/// Generate a markdown document of a Cleaner's docs.
#[derive(Debug, Parser)]
pub struct Args {
    /// The Cleaner to document.
    #[arg(long, default_value = "engine/src/cleaner/bundled-cleaner.json")]
    pub cleaner: PathBuf
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        serde_json::from_str::<RelevantCleaner>(
            &std::fs::read_to_string(self.cleaner).unwrap()
        ).unwrap().print();
    }
}

/// The relevant parts of a Cleaner.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct RelevantCleaner {
    /// The [`RelevantParams`].
    pub params: RelevantParams,
    /// The [`Docs`].
    pub docs: Docs
}

impl RelevantCleaner {
    /// Print the contents.
    pub fn print(&self) {
        self.docs.print(self)
    }
}

/// The relevant part of a Params.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct RelevantParams {
    /// The vars.
    pub vars: HashMap<String, String>
}

/// A Cleaner's docs.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Docs {
    /// The name.
    name: String,
    /// The description.
    #[serde(default)] pub description: Vec<String>,
    /// The flags.
    #[serde(default)] pub params: ParamsDocs,
    /// The [`TaskContextDocs`].
    #[serde(default)] pub task_context: TaskContextDocs,
    /// The [`JobContextDocs`].
    #[serde(default)] pub job_context : JobContextDocs,
    /// The environment vars.
    #[serde(default)]
    #[serde_as(as = "Map<_, _>")]
    pub environment_vars: Vec<(String, String)>
}

impl Docs {
    /// Print the contents.
    pub fn print(&self, cleaner: &RelevantCleaner) {
        println!("# {}", self.name);
        println!();

        for line in &self.description {
            println!("{line}");
            println!();
        }

        if !self.params      .is_empty() {self.params      .print(cleaner);}
        if !self.task_context.is_empty() {self.task_context.print();}
        if !self.job_context .is_empty() {self.job_context .print();}

        if !self.environment_vars.is_empty() {
            println!("## Environment Vars");
            println!();
            for (name, desc) in &self.environment_vars {
                println!("- `{name}`: {desc}");
            }
            println!();
        }
    }
}

/// Documentation for a var.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VarDoc {
    /// The description.
    pub desc: String,
    /// If it's required.
    pub required: bool,
    /// If [`Some`], what it means for it to be unset.
    ///
    /// Defaults to [`None`].
    #[serde(default)]
    pub unset: Option<String>,
    /// If non-empty, the list of valid values and their descriptions.
    ///
    /// Defaults to an empty [`Vec`].
    #[serde_as(as = "Map<_, _>")]
    #[serde(default)]
    pub variants: Vec<(String, String)>
}
