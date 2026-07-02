//! Hosts.

use std::collections::BTreeSet;

use super::prelude::*;

/// Hosts.
#[derive(Debug, Parser)]
pub struct Args {
    /// The [`Cleaner`] to use.
    #[arg(long)]
    pub cleaner: Option<PathBuf>,
    /// The [`HostPart`] to get.
    #[arg(long)]
    pub host_part: HostPart,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let cleaner: serde_json::Value = match self.cleaner {
            Some(path) => serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap(),
            None       => serde_json::from_str(BUNDLED_CLEANER_STR).unwrap()
        };

        let mut hosts = BTreeSet::new();

        get_parts(&cleaner, &mut hosts);

        let parts = hosts.iter().flat_map(|host| self.host_part.get(host)).collect::<BTreeSet<_>>();

        for part in parts {
            println!("{part}");
        }
    }
}

/// Get parts.
fn get_parts<'a>(layer: &'a serde_json::Value, out: &mut BTreeSet<Host<'a>>) {
    match layer {
        serde_json::Value::Array(arr) => {
            for x in arr {
                get_parts(x, out);
            }
        },
        serde_json::Value::Object(obj) => {
            for (k, v) in obj {
                if !out.contains(&**k) && let Ok(host) = (&**k).try_into() {
                    out.insert(host);
                }
                get_parts(v, out);
            }
        },
        serde_json::Value::String(x) => {
            if !out.contains(&**x) && let Ok(host) = (&**x).try_into() {
                out.insert(host);
            }
        },
        _ => {}
    }
}
