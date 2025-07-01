use criterion::Criterion;

use crate::*;

use url_cleaner_engine::types::*;

group!(load, default, default_from_file_original, default_from_file_minified);

fn default(c: &mut Criterion) {
    c.bench_function("Cleaner::get_default_no_cache"           , |b| b.iter(|| Cleaner::get_default_no_cache()));
}

fn default_from_file_original(c: &mut Criterion) {
    c.bench_function("Cleaner::load_from_file default original", |b| b.iter(|| Cleaner::load_from_file("default-cleaner.json")));
}

fn default_from_file_minified(c: &mut Criterion) {
    c.bench_function("Cleaner::load_from_file default minified", |b| b.iter(|| Cleaner::load_from_file(concat!(env!("OUT_DIR"), "/default-cleaner.json.minified"))));
}
