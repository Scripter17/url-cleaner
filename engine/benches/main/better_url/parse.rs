use crate::*;

group!(parse, parse1);

pub(super) fn parse1(c: &mut Criterion) {
    for host in DOMAIN_HOSTS.into_iter().chain(IP_HOSTS) {
        let url = format!("https://{host}");
        c.bench_function(&format!("BetterUrl::parse({url:?})"), |b| b.iter(|| BetterUrl::parse(bb(&url))));
    }
}
