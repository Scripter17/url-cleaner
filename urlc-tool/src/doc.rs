//! Doc.

use serde::Deserialize;
use serde_with::{serde_as, Map};

use super::prelude::*;

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
        let mut cleaner = serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(self.cleaner).unwrap()).unwrap();

        let docs = serde_json::from_value::<Docs>(cleaner["docs"].take()).unwrap();

        println!("# {}", docs.name);
        println!();

        for line in docs.description {
            println!("{line}");
            println!();
        }

        println!(  "## Params"             );
        println!("\n### Flags\n"           ); for (name, desc) in docs.flags              {println!("- `{name}`: {desc}");}
        println!("\n### Vars\n"            ); for (name, desc) in docs.vars               {println!("- `{name}`: {desc}");}
        println!("\n### Environment Vars\n"); for (name, desc) in docs.environment_vars   {println!("- `{name}`: {desc}");}
        println!("\n### Sets\n"            ); for (name, desc) in docs.sets               {println!("- `{name}`: {desc}");}
        println!("\n### Lists\n"           ); for (name, desc) in docs.lists              {println!("- `{name}`: {desc}");}
        println!("\n### Maps\n"            ); for (name, desc) in docs.maps               {println!("- `{name}`: {desc}");}
        println!("\n### Partitionings\n"   ); for (name, desc) in docs.partitionings      {println!("- `{name}`: {desc}");}

        println!("\n## Job context\n"      );
        println!("\n### Flags\n"           ); for (name, desc) in docs.job_context.flags  {println!("- `{name}`: {desc}");}
        println!("\n### Vars\n"            ); for (name, desc) in docs.job_context.vars   {println!("- `{name}`: {desc}");}

        println!("\n## Task context\n"     );
        println!("\n### Flags\n"           ); for (name, desc) in docs.task_context.flags {println!("- `{name}`: {desc}");}
        println!("\n### Vars\n"            ); for (name, desc) in docs.task_context.vars  {println!("- `{name}`: {desc}");}
    }
}

/// A Cleaner's docs.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
struct Docs {
    /// The name.
    name: String,
    /// The description.
    #[serde(default)] description: Vec<String>,
    /// The flags.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] flags           : Vec<(String, String)>,
    /// The vars.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] vars            : Vec<(String, String)>,
    /// The environment vars.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] environment_vars: Vec<(String, String)>,
    /// The sets.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] sets            : Vec<(String, String)>,
    /// The lists.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] lists           : Vec<(String, String)>,
    /// The maps.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] maps            : Vec<(String, String)>,
    /// The partitionings.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] partitionings   : Vec<(String, String)>,
    /// The [`TaskContextDocs`].
    #[serde(default)] task_context: TaskContextDocs,
    /// The [`JobContextDocs`].
    #[serde(default)] job_context : JobContextDocs
}

/// A Cleaner's TaskContext docs.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
struct TaskContextDocs {
    /// The flags.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] flags: Vec<(String, String)>,
    /// The vars.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] vars : Vec<(String, String)>,
}

/// A Cleaner's JobContext docs.
#[serde_as]
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
struct JobContextDocs {
    /// The flags.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] flags: Vec<(String, String)>,
    /// The vars.
    #[serde(default)] #[serde_as(as = "Map<_, _>")] vars : Vec<(String, String)>,
}
