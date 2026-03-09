//! Hosts.

use std::collections::BTreeSet;
use better_url::prelude::RefBetterHost;

use super::prelude::*;

/// Hosts.
#[derive(Debug, Parser)]
pub struct Args {
    /// The Cleaner to use.
    #[arg(long)]
    pub cleaner: Option<PathBuf>,
    /// The part of the host to print.
    #[arg(long, default_value = "host")]
    pub part: HostPart
}

/// The part of the host to print.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum HostPart {
    /// The whole host.
    Host,
    /// The normalized host.
    NormalizedHost,
    /// The domain.
    Domain,
    /// The subdomain.
    Subdomain,
    /// The not domain suffix.
    NotDomainSuffix,
    /// The domain middle.
    DomainMiddle,
    /// The reg domain.
    RegDomain,
    /// The domain suffix.
    DomainSuffix,
}

/// The regex to find strings that are hosts.
static FILTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^(?:[\w-]+(\.[\w-]+)+|\d+\.\d+\.\d+\.\d+|\[[:\da-fA-F]+\])$"#).unwrap());

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let cleaner: serde_json::Value = match self.cleaner {
            Some(path) => serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap(),
            None       => serde_json::from_str(BUNDLED_CLEANER_STR).unwrap()
        };

        let mut hosts = Vec::new();

        get_hosts(cleaner, &mut hosts);

        let mut parts = BTreeSet::new();

        for host in hosts.iter().filter_map(|x| RefBetterHost::parse(x).ok()) {
            match self.part {
                HostPart::Host            => {parts.insert(host.as_str());},
                HostPart::NormalizedHost  => {parts.insert(host.normalized_host());},
                HostPart::Domain          => if let Some(part) = host.domain           () {parts.insert(part);},
                HostPart::Subdomain       => if let Some(part) = host.subdomain        () {parts.insert(part);},
                HostPart::NotDomainSuffix => if let Some(part) = host.not_domain_suffix() {parts.insert(part);},
                HostPart::DomainMiddle    => if let Some(part) = host.domain_middle    () {parts.insert(part);},
                HostPart::RegDomain       => if let Some(part) = host.reg_domain       () {parts.insert(part);},
                HostPart::DomainSuffix    => if let Some(part) = host.domain_suffix    () {parts.insert(part);},
            }
        }

        for part in parts {
            println!("{part}");
        }
    }
}

/// Get hosts.
fn get_hosts(layer: serde_json::Value, out: &mut Vec<String>) {
    match layer {
        serde_json::Value::Array (arr) => for x in arr {
            get_hosts(x, out);
        },
        serde_json::Value::Object(obj) => for (k, v) in obj {
            if FILTER.is_match(&k) {
                out.push(k);
            }
            get_hosts(v, out);
        },
        serde_json::Value::String(x) => if FILTER.is_match(&x) {
            out.push(x);
        },
        _ => {}
    }
}
