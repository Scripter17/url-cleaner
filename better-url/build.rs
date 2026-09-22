//! Generate `idna-data.bin`.

use std::io::Write;

/// IdnaMappingTable.txt
const TABLE: &str = include_str!("src/util/parts/host/domain/IdnaMappingTable.txt");

fn main() {
    println!("cargo::rerun-if-changed=src/util/parts/host/domain/IdnaMappingTable.txt");

    // Generates a file containing a list of u32s in increasing order.
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

    let out_be_path = format!("{}/idna-data-be.bin", std::env::var("OUT_DIR").expect("OUT_DIR to be set"));
    let out_le_path = format!("{}/idna-data-le.bin", std::env::var("OUT_DIR").expect("OUT_DIR to be set"));

    let mut out_be = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(out_be_path).expect("To open the out file.");
    let mut out_le = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(out_le_path).expect("To open the out file.");

    let mut current_valid = false;

    for mut line in TABLE.lines() {
        line = line.split('#').next().expect("???");

        if line.is_empty() {
            continue;
        }

        let start = line.split(';').next().expect("???").split("..").next().expect("???").trim();

        let start = u32::from_str_radix(start, 16).expect("To parse the start of the range.");
        let status = line.split(';').nth(1).expect("To have a second column").trim();

        let next_valid = status == "valid" || status == "deviation";

        if current_valid != next_valid {
            out_be.write_all(&start.to_be_bytes()).expect("To write to the BE out file.");
            out_le.write_all(&start.to_le_bytes()).expect("To write to the LE out file.");

            current_valid = next_valid;
        }
    }

    out_be.flush().expect("To flush the out BE file.");
    out_le.flush().expect("To flush the out LE file.");
}
