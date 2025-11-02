use crate::*;

group!(caching, rwr);

const SUBJECTS: [&str; 3] = ["", "redirect", "1234567890abcdefghijklmnopqrstuvwxyz"];
const KEYS    : [&str; 3] = ["", "https://example.com/", "https://example.com/sahdgkdjhgasdjhgsdafigsdbfksdjfsdfsdfsfg"];
const VALUES  : [Option<&str>; 4] = [None, Some(""), Some("https://example.com"), Some("https://example.com/sahdgkdjhgasdjhgsdafigsdbfksdjfsdfsdfsfg")];

fn rwr(c: &mut Criterion) {
    let handle = Cache {
        inner: &Default::default(),
        config: Default::default()
    };

    for pass in ["from empty", "from full"] {
        for subject in SUBJECTS {
            for key in KEYS {
                c.bench_function(
                    &format!("Cache::read ({pass}) (empty): {subject:?}, {key:?}"),
                    |b| b.iter(
                        || handle.read(bb(CacheEntryKeys {subject, key}))
                    )
                );
                for value in VALUES {
                    c.bench_function(
                        &format!("Cache::write ({pass}): {subject:?}, {key:?}, {value:?}"),
                        |b| b.iter(
                            || handle.write(bb(NewCacheEntry {subject, key, value, duration: Default::default()}))
                        )
                    );
                    c.bench_function(
                        &format!("Cache::read ({pass}) ({value:?}): {subject:?}, {key:?}"),
                        |b| b.iter(
                            || handle.read(bb(CacheEntryKeys {subject, key}))
                        )
                    );
                }
            }
        }
    }
}
