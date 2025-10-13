use std::hint::black_box;
use criterion::Criterion;

use crate::*;

use url_cleaner_engine::types::*;
use url_cleaner_engine::glue::prelude::*;

group!(caching, rwr);

const SUBJECTS: [&str; 3] = ["", "redirect", "1234567890abcdefghijklmnopqrstuvwxyz"];
const KEYS    : [&str; 3] = ["", "https://example.com/", "https://example.com/sahdgkdjhgasdjhgsdafigsdbfksdjfsdfsdfsfg"];
const VALUES  : [Option<&str>; 4] = [None, Some(""), Some("https://example.com"), Some("https://example.com/sahdgkdjhgasdjhgsdafigsdbfksdjfsdfsdfsfg")];

fn rwr(c: &mut Criterion) {
    let handle = CacheHandle {
        cache: &Cache::from(CachePath::Memory),
        config: CacheHandleConfig::default()
    };

    for pass in ["from empty", "from full"] {
        for subject in SUBJECTS {
            for key in KEYS {
                c.bench_function(
                    &format!("CacheHandle::read ({pass}) (empty): {subject:?}, {key:?}"),
                    |b| b.iter(
                        || handle.read(black_box(CacheEntryKeys {subject, key}))
                    )
                );
                for value in VALUES {
                    c.bench_function(
                        &format!("CacheHandle::write ({pass}): {subject:?}, {key:?}, {value:?}"),
                        |b| b.iter(
                            || handle.write(black_box(NewCacheEntry {subject, key, value, duration: Default::default()}))
                        )
                    );
                    c.bench_function(
                        &format!("CacheHandle::read ({pass}) ({value:?}): {subject:?}, {key:?}"),
                        |b| b.iter(
                            || handle.read(black_box(CacheEntryKeys {subject, key}))
                        )
                    );
                }
            }
        }
    }
}
