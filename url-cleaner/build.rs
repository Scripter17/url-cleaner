//! Build script for build script stuff.

use std::process::Command;

fn main() {
    // "Watch" the default config.
    println!("cargo::rerun-if-changed=default-cleaner.json");

    // Get version info.
    let tags = String::from_utf8( Command::new("git").args(["tag"      , "--contains", "HEAD"]).output().expect("Spawning commands to work").stdout).expect("Git to give valid UTF-8 output").trim().replace('\n', ",");
    let hash = String::from_utf8( Command::new("git").args(["rev-parse",               "HEAD"]).output().expect("Spawning commands to work").stdout).expect("Git to give valid UTF-8 output").trim().to_string();
    let more =                   !Command::new("git").args(["diff"                           ]).output().expect("Spawning commands to work").stdout.is_empty();
    let version_info = match (&*tags, hash, more) {
        ("", hash, false) => hash,
        ("", hash, true ) => format!("{hash} and more"),
        (_ , hash, false) => format!("{tags} ({hash})"),
        (_ , hash, true ) => format!("{hash} and more")
   };
    println!("cargo::rustc-env=VERSION_INFO={version_info}");
}
