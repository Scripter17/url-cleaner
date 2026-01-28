//! Domains.

use super::prelude::*;

/// Get domains from a Cleaner.
#[derive(Debug, Parser)]
pub struct Args {
    /// The Cleaner to use.
    #[arg(long, default_value = "engine/src/cleaner/bundled-cleaner.json")]
    pub cleaner: PathBuf,
    /// Get RegDomains.
    #[arg(long)]
    pub reg_domains: bool
}

/// The regex to find strings that are domains.
static FILTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^[\w-]+(\.[\w-]+)+$"#).unwrap());

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let cleaner = serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(self.cleaner).unwrap()).unwrap();

        let mut strings = Vec::new();

        get_strings(&cleaner, &mut strings);

        let domains_iter = strings.into_iter().filter(|string| FILTER.is_match(string));

        let mut domains: Vec<_> = match self.reg_domains {
            true  => domains_iter.filter_map(psl::domain_str).collect(),
            false => domains_iter.collect()
        };

        domains.sort();

        for domain in domains {
            println!("{domain}");
        }
    }
}

/// Handle a layer.
fn get_strings<'a>(layer: &'a serde_json::Value, out: &mut Vec<&'a str>) {
    match layer {
        serde_json::Value::Array (x) => for     v  in x {             get_strings(v, out);},
        serde_json::Value::Object(x) => for (k, v) in x {out.push(k); get_strings(v, out);},
        serde_json::Value::String(x) => out.push(x),
        _ => {}
    }
}
