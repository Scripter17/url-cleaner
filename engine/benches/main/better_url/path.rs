use crate::*;

group!(path, set);

fn set(c: &mut Criterion) {
    let url = BetterUrl::parse("https://example.com").unwrap();
    for path in ["", "/", "a", "/a", "abcdef", "/abcdef", "abcdef/ghijk", "/abcdef/ghijk"] {
        c.bench_function(
            &format!("BetterUrl::set_path({path:?}): {url}"),
            |b| b.iter_batched_ref(
                || url.clone(),
                |url| bb(url).set_path(bb(path)),
                criterion::BatchSize::SmallInput
            )
        );
    }
}
