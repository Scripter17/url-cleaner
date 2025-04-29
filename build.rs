//! Build script for build script stuff.

use std::io::Write;
use std::process::Command;

fn main() {
    // "Watch" the default config.
    println!("cargo::rerun-if-changed=default-cleaner.json");

    // Get version info.
    let tag  = String::from_utf8( Command::new("git").args(["tag"      , "--contains", "HEAD"]).output().expect("Spawning commands to work").stdout).expect("Git to give valid UTF-8 output").trim().to_string();
    let hash = String::from_utf8( Command::new("git").args(["rev-parse",               "HEAD"]).output().expect("Spawning commands to work").stdout).expect("Git to give valid UTF-8 output").trim().to_string();
    let more =                   !Command::new("git").args(["diff"                           ]).output().expect("Spawning commands to work").stdout.is_empty();
    let version_info = match (&*tag, hash, more) {
        ("", hash, false) => hash,
        (_ , hash, true ) => format!("{hash} and more"),
        (_ , _   , false) => tag
    };
    println!("cargo::rustc-env=VERSION_INFO={version_info}");

    // Minify the default cleaner.
    let default_cleaner = serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string("default-cleaner.json").expect("Reading the default cleaner to work.")).expect("Deserializing the default cleaner to work.");
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("Env var OUT_DIR to be set.")).join("default-cleaner.json.minified"))
        .expect("Opening default-cleaner.minified.json to work.")
        .write_all(serde_json::to_string(&default_cleaner).expect("Serializing the default cleaner to work.").as_bytes())
        .expect("Writing the minified default cleaner to work.");
}
