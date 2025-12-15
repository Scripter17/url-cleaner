//! Benchmarking stuff.

#![allow(clippy::unwrap_used, reason = "Who cares?")]
#![allow(missing_docs, reason = "Who cares?")]
#![allow(clippy::missing_docs_in_private_items, reason = "Who cares?")]

macro_rules! group {
    ($name:ident, $($targets:path),+) => {
        pub fn $name(c: &mut criterion::Criterion) {
            $($targets(c);)+
        }
    }
}
macro_rules! group_mods {
    ($name:ident, $($mods:ident),+) => {
        $(mod $mods;)+
        group!($name, $($mods::$mods),+);
    }
}

group_mods!(all, segments);

fn main() {
    let mut c = criterion::Criterion::default()
        .configure_from_args();
    all(&mut c);
    c.final_summary();
}
