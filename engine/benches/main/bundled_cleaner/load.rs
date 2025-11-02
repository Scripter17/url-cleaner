use crate::*;

group!(load, fns, #[cfg(feature = "bundled-cleaner")] parse_bundled_included, parse_bundled_file_original, parse_bundled_file_minified);
group!(fns, #[cfg(feature = "bundled-cleaner")] get_bundled_no_cache, load_from_file_bundled_original, load_from_file_bundled_minified);
#[cfg(feature = "bundled-cleaner")]
group!(parse_bundled_included     , parse_bundled_included_str     , parse_bundled_included_bytes     , parse_bundled_included_bytes_to_str     );
group!(parse_bundled_file_original, parse_bundled_file_original_str, parse_bundled_file_original_bytes, parse_bundled_file_original_bytes_to_str, parse_bundled_file_original_reader);
group!(parse_bundled_file_minified, parse_bundled_file_minified_str, parse_bundled_file_minified_bytes, parse_bundled_file_minified_bytes_to_str, parse_bundled_file_minified_reader);

const DCMF: &str = concat!(env!("OUT_DIR"), "/bundled-cleaner.json.minified");

#[cfg(feature = "bundled-cleaner")]
fn get_bundled_no_cache(c: &mut Criterion) {
    c.bench_function("Cleaner::get_bundled_no_cache", |b| b.iter(|| {
        Cleaner::get_bundled_no_cache().unwrap()
    }));
}

fn load_from_file_bundled_original(c: &mut Criterion) {
    c.bench_function("Cleaner::load_from_file bundled original", |b| b.iter(|| {
        Cleaner::load_from_file("bundled-cleaner.json").unwrap()
    }));
}

fn load_from_file_bundled_minified(c: &mut Criterion) {
    c.bench_function("Cleaner::load_from_file bundled minified", |b| b.iter(|| {
        Cleaner::load_from_file(DCMF).unwrap()
    }));
}

#[cfg(feature = "bundled-cleaner")]
fn parse_bundled_included_str(c: &mut Criterion) {
    c.bench_function("Cleaner parse bundled included str", |b| b.iter(|| {
        serde_json::from_str::<Cleaner>(bb(BUNDLED_CLEANER_STR)).unwrap()
    }));
}

#[cfg(feature = "bundled-cleaner")]
fn parse_bundled_included_bytes(c: &mut Criterion) {
    c.bench_function("Cleaner parse bundled included bytes", |b| b.iter(|| {
        serde_json::from_slice::<Cleaner>(bb(BUNDLED_CLEANER_STR.as_bytes())).unwrap()
    }));
}

#[cfg(feature = "bundled-cleaner")]
fn parse_bundled_included_bytes_to_str(c: &mut Criterion) {
    c.bench_function("Cleaner parse bundled included bytes to str", |b| b.iter(|| {
        serde_json::from_str::<Cleaner>(bb(str::from_utf8(bb(BUNDLED_CLEANER_STR.as_bytes())).unwrap())).unwrap()
    }));
}



fn parse_bundled_file_original_str(c: &mut Criterion) {
    c.bench_function("Cleaner parse bundled file original str", |b| b.iter(|| {
        serde_json::from_str::<Cleaner>(&bb(read_to_string("bundled-cleaner.json").unwrap()))
    }));
}

fn parse_bundled_file_original_bytes(c: &mut Criterion) {
    c.bench_function("Cleaner parse bundled file original bytes", |b| b.iter(|| {
        let mut buf = Vec::new();
        File::open("bundled-cleaner.json").unwrap().read_to_end(&mut buf).unwrap();
        serde_json::from_slice::<Cleaner>(&bb(buf)).unwrap();
    }));
}

fn parse_bundled_file_original_bytes_to_str(c: &mut Criterion) {
    c.bench_function("Cleaner parse bundled file original bytes to str", |b| b.iter(|| {
        let mut buf = Vec::new();
        File::open("bundled-cleaner.json").unwrap().read_to_end(&mut buf).unwrap();
        serde_json::from_str::<Cleaner>(&bb(String::try_from(bb(buf)).unwrap())).unwrap();
    }));
}

fn parse_bundled_file_original_reader(c: &mut Criterion) {
    c.bench_function("Cleaner parse bundled file original reader", |b| b.iter(|| {
        serde_json::from_reader::<_, Cleaner>(bb(File::open("bundled-cleaner.json").unwrap())).unwrap();
    }));
}



fn parse_bundled_file_minified_str(c: &mut Criterion) {
    c.bench_function("Cleaner parse bundled file minified str", |b| b.iter(|| {
        serde_json::from_str::<Cleaner>(&bb(read_to_string(DCMF).unwrap()))
    }));
}

fn parse_bundled_file_minified_bytes(c: &mut Criterion) {
    c.bench_function("Cleaner parse bundled file minified bytes", |b| b.iter(|| {
        let mut buf = Vec::new();
        File::open(DCMF).unwrap().read_to_end(&mut buf).unwrap();
        serde_json::from_slice::<Cleaner>(&bb(buf)).unwrap();
    }));
}

fn parse_bundled_file_minified_bytes_to_str(c: &mut Criterion) {
    c.bench_function("Cleaner parse bundled file minified bytes to str", |b| b.iter(|| {
        let mut buf = Vec::new();
        File::open(DCMF).unwrap().read_to_end(&mut buf).unwrap();
        serde_json::from_str::<Cleaner>(&bb(String::try_from(bb(buf)).unwrap())).unwrap();
    }));
}

fn parse_bundled_file_minified_reader(c: &mut Criterion) {
    c.bench_function("Cleaner parse bundled file minified reader", |b| b.iter(|| {
        serde_json::from_reader::<_, Cleaner>(File::open(DCMF).unwrap()).unwrap()
    }));
}
