//! Build script for build script stuff.

use std::io::Write;

fn main() {
    // "Watch" the bundled cleaner.
    println!("cargo::rerun-if-changed=bundled-cleaner.json");

    // Minify the bundled cleaner.
    let original = std::fs::read_to_string("bundled-cleaner.json").expect("The bundled cleaner couldn't be read").into_bytes();
    let mut minified = Vec::with_capacity(original.len());

    // A basic state machine to remove unnecessary whitespace.
    let mut ins = false;
    let mut esc = false;
    for b in original {
        match (ins, esc, b) {
            (false, _    , b' ' | b'\n' | b'\t') => continue,
            (false, _    , b'"' ) => ins = true,
            (false, _    , _    ) => {},
            (true , false, b'"' ) => ins = false,
            (true , false, b'\\') => esc = true,
            (true , false, _    ) => {},
            (true , true , _    ) => esc = false,
        }
        minified.push(b);
    }
    
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("{}/bundled-cleaner.json.minified", std::env::var("OUT_DIR").expect("OUT_DIR to be set")))
        .expect("The minified bundled cleaner's output file couldn't be opened")
        .write_all(&minified)
        .expect("The minified bundled cleaner coulnd't be written");
}
