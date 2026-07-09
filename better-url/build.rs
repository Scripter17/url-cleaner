//! Generate `idna-data.bin`.

use std::io::Write;

/// IdnaMappingTable.txt
const TABLE: &str = include_str!("src/util/parts/host/domain/IdnaMappingTable.txt");

fn main() {
    println!("cargo::rerun-if-changed=src/util/parts/host/domain/IdnaMappingTable.txt");

    // Generates a file containing a list of 24 bit integers in increasing order.
    //
    // The first number is the first entry in IdnaMappingTable.txt whose status if "valid" or "deviation".
    // The second number is the next entry whose status is neither "valid" or "devaition".
    // The third number is the next entry whose status is "valid" or "deviation".
    // And so on.
    //
    // For example, the first 5 entries are 0x00, 0x41, 0x5B. 0x80, and 0xA1.
    //
    // 0x00..0x41 are all "valid".
    // 0x41..0x5B are all "mapped".
    // 0x5B..0x7F are all "valid".
    // 0x80..0xA1 are all either "disallowed" or "mapped".
    //
    // This is intended to be `include!`d as a `&[[u8; 3]]` on which you use `[u8]::binary_search` with `&(char as u32).to_be_bytes()[1..]`.

    let out_path = format!("{}/idna-data.bin", std::env::var("OUT_DIR").expect("OUT_DIR to be set"));

    let mut out           = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(out_path).expect("To open the out file.");
    let mut current_valid = false;

    for mut line in TABLE.lines() {
        #[cfg(debug_assertions)]
        if let Some(idna_unicode_version) = line.strip_prefix("# Version: ") {
            let (a, b, c) = char::UNICODE_VERSION;
            let rust_unicode_version = format!("{a}.{b}.{c}");

            if rust_unicode_version != idna_unicode_version {
                println!("cargo::warning=Rust has Unicode version {rust_unicode_version} but IdnaMappingTable.txt is from Unicode version {idna_unicode_version}.");
            }
        }

        line = line.split('#').next().expect("???");

        if line.is_empty() {
            continue;
        }

        let start = line.split(';').next().expect("???").split("..").next().expect("???").trim();

        let start = u32::from_str_radix(start, 16).expect("To parse the start of the range.");
        let status = line.split(';').nth(1).expect("To have a second column").trim();

        let next_valid = status == "valid" || status == "deviation";

        if current_valid != next_valid {
            match start.to_be_bytes() {
                [0, x, y, z] => out.write_all(&[x, y, z]).expect("To write to the out file."),
                _            => panic!("Somehow 25+ bit char??? {start:x}"),
            }
            current_valid = next_valid;
        }
    }

    out.flush().expect("To flush the out file.");
}
