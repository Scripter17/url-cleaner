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

group_mods!(all, segments);

fn main() {
    let mut c = criterion::Criterion::default()
        .configure_from_args();
    all(&mut c);
    c.final_summary();
}
