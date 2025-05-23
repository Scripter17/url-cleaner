//! Build script for build script stuff.

use std::io::Write;

fn main() {
    // "Watch" the default config.
    println!("cargo::rerun-if-changed=default-cleaner.json");

    // Minify the default cleaner.
    let default_cleaner = serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string("default-cleaner.json").expect("The default cleaner couldn't be read")).expect("The default cleaner coulnd't be parsed");
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("The OUT_DIR environment variable wasn't set")).join("default-cleaner.json.minified"))
        .expect("The minified default cleaner's output file couldn't be opened")
        .write_all(serde_json::to_string(&default_cleaner).expect("The minified default cleaner couldn't be serialized").as_bytes())
        .expect("The minified default cleaner coulnd't be written");
}
