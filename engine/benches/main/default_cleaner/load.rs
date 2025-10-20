use crate::*;

group!(load, fns, #[cfg(feature = "default-cleaner")] parse_default_included, parse_default_file_original, parse_default_file_minified);
group!(fns, #[cfg(feature = "default-cleaner")] get_default_no_cache, load_from_file_default_original, load_from_file_default_minified);
#[cfg(feature = "default-cleaner")]
group!(parse_default_included     , parse_default_included_str     , parse_default_included_bytes     , parse_default_included_bytes_to_str     );
group!(parse_default_file_original, parse_default_file_original_str, parse_default_file_original_bytes, parse_default_file_original_bytes_to_str, parse_default_file_original_reader);
group!(parse_default_file_minified, parse_default_file_minified_str, parse_default_file_minified_bytes, parse_default_file_minified_bytes_to_str, parse_default_file_minified_reader);

const DCMF: &str = concat!(env!("OUT_DIR"), "/default-cleaner.json.minified");

#[cfg(feature = "default-cleaner")]
fn get_default_no_cache(c: &mut Criterion) {
    c.bench_function("Cleaner::get_default_no_cache", |b| b.iter(|| {
        Cleaner::get_default_no_cache().unwrap()
    }));
}

fn load_from_file_default_original(c: &mut Criterion) {
    c.bench_function("Cleaner::load_from_file default original", |b| b.iter(|| {
        Cleaner::load_from_file("default-cleaner.json").unwrap()
    }));
}

fn load_from_file_default_minified(c: &mut Criterion) {
    c.bench_function("Cleaner::load_from_file default minified", |b| b.iter(|| {
        Cleaner::load_from_file(DCMF).unwrap()
    }));
}

#[cfg(feature = "default-cleaner")]
fn parse_default_included_str(c: &mut Criterion) {
    c.bench_function("Cleaner parse default included str", |b| b.iter(|| {
        serde_json::from_str::<Cleaner>(bb(DEFAULT_CLEANER_STR)).unwrap()
    }));
}

#[cfg(feature = "default-cleaner")]
fn parse_default_included_bytes(c: &mut Criterion) {
    c.bench_function("Cleaner parse default included bytes", |b| b.iter(|| {
        serde_json::from_slice::<Cleaner>(bb(DEFAULT_CLEANER_STR.as_bytes())).unwrap()
    }));
}

#[cfg(feature = "default-cleaner")]
fn parse_default_included_bytes_to_str(c: &mut Criterion) {
    c.bench_function("Cleaner parse default included bytes to str", |b| b.iter(|| {
        serde_json::from_str::<Cleaner>(bb(str::from_utf8(bb(DEFAULT_CLEANER_STR.as_bytes())).unwrap())).unwrap()
    }));
}



fn parse_default_file_original_str(c: &mut Criterion) {
    c.bench_function("Cleaner parse default file original str", |b| b.iter(|| {
        serde_json::from_str::<Cleaner>(&bb(read_to_string("default-cleaner.json").unwrap()))
    }));
}

fn parse_default_file_original_bytes(c: &mut Criterion) {
    c.bench_function("Cleaner parse default file original bytes", |b| b.iter(|| {
        let mut buf = Vec::new();
        File::open("default-cleaner.json").unwrap().read_to_end(&mut buf).unwrap();
        serde_json::from_slice::<Cleaner>(&bb(buf)).unwrap();
    }));
}

fn parse_default_file_original_bytes_to_str(c: &mut Criterion) {
    c.bench_function("Cleaner parse default file original bytes to str", |b| b.iter(|| {
        let mut buf = Vec::new();
        File::open("default-cleaner.json").unwrap().read_to_end(&mut buf).unwrap();
        serde_json::from_str::<Cleaner>(&bb(String::try_from(bb(buf)).unwrap())).unwrap();
    }));
}

fn parse_default_file_original_reader(c: &mut Criterion) {
    c.bench_function("Cleaner parse default file original reader", |b| b.iter(|| {
        serde_json::from_reader::<_, Cleaner>(bb(File::open("default-cleaner.json").unwrap())).unwrap();
    }));
}



fn parse_default_file_minified_str(c: &mut Criterion) {
    c.bench_function("Cleaner parse default file minified str", |b| b.iter(|| {
        serde_json::from_str::<Cleaner>(&bb(read_to_string(DCMF).unwrap()))
    }));
}

fn parse_default_file_minified_bytes(c: &mut Criterion) {
    c.bench_function("Cleaner parse default file minified bytes", |b| b.iter(|| {
        let mut buf = Vec::new();
        File::open(DCMF).unwrap().read_to_end(&mut buf).unwrap();
        serde_json::from_slice::<Cleaner>(&bb(buf)).unwrap();
    }));
}

fn parse_default_file_minified_bytes_to_str(c: &mut Criterion) {
    c.bench_function("Cleaner parse default file minified bytes to str", |b| b.iter(|| {
        let mut buf = Vec::new();
        File::open(DCMF).unwrap().read_to_end(&mut buf).unwrap();
        serde_json::from_str::<Cleaner>(&bb(String::try_from(bb(buf)).unwrap())).unwrap();
    }));
}

fn parse_default_file_minified_reader(c: &mut Criterion) {
    c.bench_function("Cleaner parse default file minified reader", |b| b.iter(|| {
        serde_json::from_reader::<_, Cleaner>(File::open(DCMF).unwrap()).unwrap()
    }));
}
