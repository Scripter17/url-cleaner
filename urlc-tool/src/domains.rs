//! Domains.

use std::collections::BTreeSet;

use super::prelude::*;

/// Get domains from a Cleaner.
#[derive(Debug, Parser)]
pub struct Args {
    /// The Cleaner to use.
    #[arg(long, default_value = "engine/src/cleaner/bundled-cleaner.json")]
    pub cleaner: PathBuf,
    /// Output only registerable domains.
    #[arg(long)]
    pub reg_domains: bool
}

/// The regex to find strings that are domains.
static FILTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^[\w-]+(\.[\w-]+)+$"#).unwrap());

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let cleaner = serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(self.cleaner).unwrap()).unwrap();

        let mut ret = BTreeSet::new();

        handle_layer(cleaner, self.reg_domains, &mut ret);

        for domain in ret {
            println!("{domain}");
        }
    }
}

/// Handle a layer.
fn handle_layer(layer: serde_json::Value, reg_domains: bool, ret: &mut BTreeSet<String>) {
    match layer {
        serde_json::Value::Array (x) => for y      in x {handle_layer(y, reg_domains, ret);},
        serde_json::Value::Object(x) => for (_, v) in x {handle_layer(v, reg_domains, ret);},
        serde_json::Value::String(x) => if FILTER.find(&x).is_some() {
            if reg_domains && let Some(x) = psl::domain_str(&x) {
                ret.insert(x.into());
            } else {
                ret.insert(x);
            }
        },
        _ => {}
    }
}
