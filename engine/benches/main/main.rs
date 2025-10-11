//! Benchmarking stuff.

const DOMAIN_HOSTS: [&'static str; 12] = [
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

const IP_HOSTS: [&'static str; 4] = [
    "127.0.0.1",
    "1.1.1.1",
    "255.255.255.255",
    "[::1]"
];

macro_rules! group {
    ($name:ident, $($targets:path),+) => {
        pub fn $name(c: &mut criterion::Criterion) {
            $($targets(c);)+
        }
    }
}
pub(crate) use group;
macro_rules! group_mods {
    ($name:ident, $($mods:ident),+) => {
        $(mod $mods;)+
        group!($name, $($mods::$mods),+);
    }
}
pub(crate) use group_mods;

group_mods!(all, better_url, default_cleaner, host_details, caching);

fn main() {
    let mut c = criterion::Criterion::default()
        .configure_from_args();
    all(&mut c);
    c.final_summary();
}
