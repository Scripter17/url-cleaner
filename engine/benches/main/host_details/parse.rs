use std::hint::black_box;
use criterion::Criterion;

use super::*;

use url_cleaner_engine::types::*;
use url::Host;

group!(parse, from_str, from_host);

fn from_str(c: &mut Criterion) {
    for host in DOMAIN_HOSTS.into_iter().chain(IP_HOSTS) {
        c.bench_function(
            &format!("HostDetails::from_host_str({host:?})"),
            |b| b.iter(|| HostDetails::parse(black_box(host)))
        );
    }
}

fn from_host(c: &mut Criterion) {
    for host in DOMAIN_HOSTS.into_iter().chain(IP_HOSTS) {
        let host = Host::parse(host).unwrap();
        c.bench_function(
            &format!("HostDetails::from_host({host:?})"),
            |b| b.iter(|| HostDetails::from_host(black_box(&host)))
        );
    }
}
