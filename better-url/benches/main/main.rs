mod segments;

macro_rules! group {
    ($name:ident, $($targets:path),+) => {
        pub(super) fn $name(c: &mut criterion::Criterion) {
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

macro_rules! main {
    ($($groups:path),+) => {
        fn main() {
            let mut c = criterion::Criterion::default()
                .configure_from_args();
            $($groups(&mut c);)+
            c.final_summary();
        }
    }
}

main!(segments::segments);

