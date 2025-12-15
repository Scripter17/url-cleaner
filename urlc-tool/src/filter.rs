//! Filter.

use super::prelude::*;

use std::io::pipe;

/// Filter failing benchmarks.
///
/// Reads lines from STDIN.
///
/// Prints passing lines to STDOUT.
///
/// Prints failing lines to STDERR.
#[derive(Debug, Parser)]
pub struct Args {
    /// The format of the lines being filtered.
    #[arg(long, value_enum, default_value_t = Default::default())]
    format: Format,
    /// Extra arguments to give URL Cleaner CLI.
    #[arg(last = true)]
    last: Vec<String>
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let (in_read, mut in_write) = pipe().unwrap();
        let (out_read, out_write) = pipe().unwrap();

        let _child = TerminateOnDrop(Command::new("target/release/url-cleaner")
            .args(["--output-buffer", "0"])
            .args(self.last)
            .stdin(in_read)
            .stdout(out_write)
            .spawn().unwrap());

        let mut results = BufReader::new(out_read).lines();

        for line in BufReader::new(std::io::stdin()).lines().map(Result::unwrap) {
            let mut parts = line.splitn(3, '\t');

            match self.format {
                Format::Quick => {},
                Format::Suite => {parts.next().unwrap(); parts.next().unwrap();}
            }

            let task = parts.next().unwrap();

            writeln!(in_write, "{task}").unwrap();

            if results.next().unwrap().unwrap().starts_with('-') {
                eprintln!("{line}");
            } else {
                println!("{line}");
            }
        }
    }
}
