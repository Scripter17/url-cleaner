use crate::*;

group!(parse, from_str, from_host);

fn from_str(c: &mut Criterion) {
    for host in DOMAIN_HOSTS.into_iter().chain(IP_HOSTS) {
        c.bench_function(
            &format!("HostDetails::from_host_str({host:?})"),
            |b| b.iter(|| HostDetails::parse(bb(host)))
        );
    }
}

fn from_host(c: &mut Criterion) {
    for host in DOMAIN_HOSTS.into_iter().chain(IP_HOSTS) {
        let host = Host::parse(host).unwrap();
        c.bench_function(
            &format!("HostDetails::from_host({host:?})"),
            |b| b.iter(|| HostDetails::from_host(bb(&host)))
        );
    }
}
