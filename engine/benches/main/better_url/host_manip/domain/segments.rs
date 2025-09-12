use std::hint::black_box;
use criterion::Criterion;

use super::*;

const INDICES: [isize; 7] = [-3, -2, -1, 0, 1, 2, 3];

const INSERTS: [&'static str; 2] = [
    "www",
    "somelongvalue"
];

const SETS: [Option<&'static str>; 3] = [
    Some("www"),
    Some("somelongvalue"),
    None
];

macro_rules! thing {
    ($group:ident, $(($funcs:ident, $values:ident)),+) => {
        group!($group, $($funcs),+);
        $(
            fn $funcs(c: &mut Criterion) {
                for host in DOMAIN_HOSTS {
                    let url = BetterUrl::parse(&format!("https://{host}")).unwrap();

                    for index in INDICES {
                        for value in $values {
                            c.bench_function(
                                &format!("BetterUrl::{}({index}, {value:?}): {url}", stringify!($funcs)),
                                |b| b.iter_batched_ref(
                                    || url.clone(),
                                    |url| black_box(url).$funcs(black_box(index), black_box(value)),
                                    criterion::BatchSize::SmallInput
                                )
                            );
                        }
                    }
                }
            }
        )+
    }
}

thing!(subdomain    , (insert_subdomain_segment    , INSERTS), (set_subdomain_segment    , SETS));
thing!(domain       , (insert_domain_segment       , INSERTS), (set_domain_segment       , SETS));
thing!(domain_suffix, (insert_domain_suffix_segment, INSERTS), (set_domain_suffix_segment, SETS));
group!(segments, domain, subdomain, domain_suffix);
