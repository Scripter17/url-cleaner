//! Build script to minify the default config to save a tiny amount of time when deserializing it.

use std::io::Write;

fn main() {
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("default-config.minified.json")
        .expect("Opening default-config.minified.json to work.")
        .write_all(serde_json::to_string(&serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string("default-config.json").expect("Reading the default config to work.")).expect("Deserializing the default config to work.")).expect("Serializing the default config to work.").as_bytes())
        .expect("Writing the minified default config to work.");
}
