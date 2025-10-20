use crate::*;

const SHORTS: [&str; 7] = [
    "https://example.com",
    "https://example.com?",
    "https://example.com?a=2",
    "https://example.com?a=2&a=3",
    "https://example.com?a=2&a=3&b=4",
    "https://example.com?a=2&a=3&b=4&a=5",
    "https://example.com?a=2&a=3&b=4&a=5&a=6"
];
const SHORT: &str = "a";

const LONGS: [&str; 7] = [
    "https://example.com",
    "https://example.com?",
    "https://example.com?abcdefghi=2",
    "https://example.com?abcdefghi=2&abcdefghi=3",
    "https://example.com?abcdefghi=2&abcdefghi=3&jklmnopqr=4",
    "https://example.com?abcdefghi=2&abcdefghi=3&jklmnopqr=4&abcdefghi=5",
    "https://example.com?abcdefghi=2&abcdefghi=3&jklmnopqr=4&abcdefghi=5&abcdefghi=6"
];
const LONG: &str = "abcdefghi";

const INDICES: [usize; 5] = [1, 2, 3, 4, 5];

macro_rules! thing {
    ($($funcs:ident),+) => {
        group!(query, $($funcs),+);
        $(
            fn $funcs(c: &mut Criterion) {
                for url in SHORTS {
                    let url = BetterUrl::parse(url).unwrap();

                    for index in INDICES {
                        c.bench_function(
                            &format!("BetterUrl::{}({SHORT:?}, {index}): {url}", stringify!($funcs)),
                            |b| b.iter(|| bb(&url).$funcs(bb(SHORT), bb(index)))
                        );
                    }
                }

                for url in LONGS {
                    let url = BetterUrl::parse(url).unwrap();

                    for index in INDICES {
                        c.bench_function(
                            &format!("BetterUrl::{}({LONG:?}, {index}): {url}", stringify!($funcs)),
                            |b| b.iter(|| bb(&url).$funcs(bb(LONG), bb(index)))
                        );
                    }
                }
            }
        )+
    }
}

thing!(raw_query_param, query_param);
