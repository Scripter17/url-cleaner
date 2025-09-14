use std::hint::black_box;
use criterion::Criterion;

use crate::*;

use url_cleaner_engine::types::*;

macro_rules! thing {
    ($group:ident, $get:ident, $set:ident, $sets:expr) => {
        group!($group, $group::$get, $group::$set);

        mod $group {
            use super::*;

            pub(super) fn $get(c: &mut Criterion) {
                for host in DOMAIN_HOSTS {
                    let url = BetterUrl::parse(&format!("https://{host}")).unwrap();

                    c.bench_function(
                        &format!("BetterUrl::{}(): {url}", stringify!($get)),
                        |b| b.iter(|| black_box(&url).$get())
                    );
                }
            }

            pub(super) fn $set(c: &mut Criterion) {
                for host in DOMAIN_HOSTS {
                    let url = BetterUrl::parse(&format!("https://{host}")).unwrap();

                    for set in $sets {
                        c.bench_function(
                            &format!("BetterUrl::{}({set:?}): {url}", stringify!($set)),
                            |b| b.iter_batched_ref(
                                || url.clone(),
                                |url| black_box(url).$set(black_box(set)),
                                criterion::BatchSize::SmallInput
                            )
                        );
                    }
                }
            }
        }
    }
}

thing!(domain_part  , domain       , set_domain       , [None, Some("example.com"), Some("www.example.com"), Some("example.co.uk"), Some("www.example.co.uk")]);
thing!(subdomain    , subdomain    , set_subdomain    , [None, Some("www"    ), Some("abc.def" )]);
thing!(domain_middle, domain_middle, set_domain_middle, [None, Some("example"), Some("example2")]);
thing!(domain_suffix, domain_suffix, set_domain_suffix, [None, Some("com"    ), Some("co.uk"   )]);
group!(domain, domain_part, subdomain, domain_middle, domain_suffix);
