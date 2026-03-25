//! Hosts.

use std::collections::BTreeSet;

use super::prelude::*;

/// Hosts.
#[derive(Debug, Parser)]
pub struct Args {
    /// The Cleaner to use.
    #[arg(long)]
    pub cleaner: Option<PathBuf>,
    /// The part of the host to print.
    #[arg(long, default_value = "Host")]
    pub part: HostPart
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let cleaner: serde_json::Value = match self.cleaner {
            Some(path) => serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap(),
            None       => serde_json::from_str(BUNDLED_CLEANER_STR).unwrap()
        };

        let mut parts = BTreeSet::new();

        get_parts(&cleaner, self.part, &mut parts);

        for part in parts {
            println!("{part}");
        }
    }
}

/// Get parts.
fn get_parts<'a>(layer: &'a serde_json::Value, part: HostPart, out: &mut BTreeSet<&'a str>) {
    match layer {
        serde_json::Value::Array(arr) => {
            for x in arr {
                get_parts(x, part, out);
            }
        },
        serde_json::Value::Object(obj) => {
            for (k, v) in obj {
                if !out.contains(&**k) && let Ok(x) = k.try_into() && let Some(part) = part.get(x) {
                    out.insert(part);
                }
                get_parts(v, part, out);
            }
        },
        serde_json::Value::String(x) => {
            if !out.contains(&**x) && let Ok(x) = x.try_into() && let Some(part) = part.get(x) {
                out.insert(part);
            }
        },
        _ => {}
    }
}
