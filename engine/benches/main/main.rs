//! Benchmarking stuff.

#![allow(clippy::unwrap_used, reason = "Who cares?")]
#![allow(missing_docs, reason = "Who cares?")]
#![allow(clippy::missing_docs_in_private_items, reason = "Who cares?")]

pub(crate) use std::hint::black_box as bb;
pub(crate) use std::io::Read;
pub(crate) use std::fs::{read_to_string, File};

pub(crate) use url::Host;
pub(crate) use criterion::Criterion;

pub(crate) use url_cleaner_engine::prelude::*;

const DOMAIN_HOSTS: [&str; 12] = [
    "example.com",
    "example.com.",
    "example.co.uk",
    "example.co.uk.",
    "www.example.com",
    "www.example.com.",
    "www.example.co.uk",
    "www.example.co.uk.",
    "abc.www.example.com",
    "abc.www.example.com.",
    "abc.www.example.co.uk",
    "abc.www.example.co.uk."
];

const IP_HOSTS: [&str; 4] = [
    "127.0.0.1",
    "1.1.1.1",
    "255.255.255.255",
    "[::1]"
];

macro_rules! group {
    ($name:ident, $($(#[$a:meta])? $targets:path),+) => {
        pub fn $name(c: &mut criterion::Criterion) {
            $($(#[$a])? $targets(c);)+
        }
    }
}
pub(crate) use group;
macro_rules! group_mods {
    ($name:ident, $($(#[$a:meta])? $mods:ident),+) => {
        $($(#[$a])? mod $mods;)+
        group!($name, $($(#[$a])? $mods::$mods),+);
    }
}
pub(crate) use group_mods;

group_mods!(all, better_url, bundled_cleaner, host_details, #[cfg(feature = "cache")] caching);

fn main() {
    let mut c = criterion::Criterion::default()
        .configure_from_args();
    all(&mut c);
    c.final_summary();
}
