use criterion::Criterion;

use crate::*;

group!(segments, set_segment, insert_segment);

const STRINGS: [&str; 3] = [
    "a-b-c",
    "aaaaaaaaaa-bbbbbbbbbb-cccccccccc",
    ""
];

const INDICES: [isize; 8] = [-4, -3, -2, -1, 0, 1, 2, 3];

const SETS: [Option<&str>; 3] = [
    Some("a"),
    Some("aaaaaaaaaa"),
    None
];

const INSERTS: [&str; 2] = [
    "a",
    "aaaaaaaaaa"
];

fn set_segment(c: &mut Criterion) {
    for string in STRINGS {
        for index in INDICES {
            for value in SETS {
                c.bench_function(
                    &format!("better_url::util::set_segment({string:?}, \"-\", {index:?}, {value:?}, ())"),
                    |b| b.iter(|| better_url::util::set_segment(string, "-", index, value, ()))
                );
            }
        }
    }
}

fn insert_segment(c: &mut Criterion) {
    for string in STRINGS {
        for index in INDICES {
            for value in INSERTS {
                c.bench_function(
                    &format!("better_url::util::insert_segment({string:?}, \"-\", {index:?}, {value:?}, ())"),
                    |b| b.iter(|| better_url::util::insert_segment(string, "-", index, value, ()))
                );
            }
        }
    }
}
