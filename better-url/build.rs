//! Generate `idna-data.bin`.

use std::io::Write;

/// IdnaMappingTable.txt
const TABLE: &str = include_str!("src/util/parts/host/domain/IdnaMappingTable.txt");

fn main() {
    println!("cargo::rerun-if-changed=src/util/parts/host/domain/IdnaMappingTable.txt");

    let out_path = format!("{}/idna-data.bin", std::env::var("OUT_DIR").expect("OUT_DIR to be set"));

    let mut out           = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(out_path).expect("To open the out file.");
    let mut current_valid = false;

    for mut line in TABLE.lines() {
        if let Some(idna_unicode_version) = line.strip_prefix("# Version: ") {
            let (a, b, c) = char::UNICODE_VERSION;
            let rust_unicode_version = format!("{a}.{b}.{c}");

            if rust_unicode_version != idna_unicode_version {
                println!("cargo::warning=Rust has Unicode version {rust_unicode_version} but IdnaMappingTable.txt is from Unicode version {idna_unicode_version}.");
            }
        }

        line = line.split('#').next().expect("To always have at least one segment.");

        if line.is_empty() {
            continue;
        }

        let start = u32::from_str_radix(&line[..4], 16).expect("To parse the start of the range.");
        let status = line.split(';').nth(1).expect("To have a second column").trim();

        let next_valid = status == "valid" || status == "deviation";

        if current_valid != next_valid {
            out.write_all(&start.to_be_bytes()).expect("To write to the out file.");
            current_valid = next_valid;
        }
    }

    out.flush().expect("To flush the out file.");
}
