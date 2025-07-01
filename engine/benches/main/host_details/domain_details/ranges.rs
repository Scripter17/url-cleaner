use std::hint::black_box;
use criterion::Criterion;

use super::*;

use url_cleaner_engine::types::*;

macro_rules! bounds {
    ($($funcs:ident),+) => {
        group!(ranges, $($funcs),+);
        $(
            fn $funcs(c: &mut Criterion) {
                for host in DOMAIN_HOSTS {
                    let domain_details = DomainDetails::parse(host).unwrap();
                    c.bench_function(
                        &format!("DomainDetails::{}(): {domain_details:?}", stringify!($funcs)),
                        |b| b.iter(|| black_box(domain_details).$funcs())
                    );
                }
            }
        )+
    };
}

bounds!(domain_bounds, subdomain_bounds, not_domain_suffix_bounds, domain_middle_bounds, reg_domain_bounds, domain_suffix_bounds);
