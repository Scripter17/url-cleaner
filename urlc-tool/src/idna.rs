use crate::prelude::*;

const TABLE: &str = include_str!("IdnaMappingTable.txt");

#[derive(Debug, Parser)]
pub struct Args {
    
}

impl Args {
    pub fn r#do(self) {
        let mut out = std::fs::OpenOptions::new().read(true).write(true).create(true).open("idna-data.bin").unwrap();
        let mut x = false;

        for mut line in TABLE.lines() {
            match line.find('#') {
                Some(i) => line = &line[..i],
                None    => {}
            }

            if line.is_empty() {
                continue;
            }

            let start = u32::from_str_radix(&line[..4], 16).unwrap();
            let status = line.split(';').nth(1).unwrap().trim();

            let y = status == "valid" || status == "deviation";

            if x != y {
                out.write_all(&start.to_be_bytes()).unwrap();
                x = !x;
            }
        }

        out.flush().unwrap();
    }
}
