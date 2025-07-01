use std::hint::black_box;
use criterion::Criterion;

use crate::*;

use url_cleaner_engine::types::*;

group!(path, set);

fn set(c: &mut Criterion) {
    let url = BetterUrl::parse("https://example.com").unwrap();
    for path in ["", "/", "a", "/a", "abcdef", "/abcdef", "abcdef/ghijk", "/abcdef/ghijk"] {
        c.bench_function(
            &format!("BetterUrl::set_path({path:?}): {url}"),
            |b| b.iter_batched_ref(
                || url.clone(),
                |url| black_box(url).set_path(black_box(path)),
                criterion::BatchSize::SmallInput
            )
        );
    }
}
