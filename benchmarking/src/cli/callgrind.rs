use std::process::Command;
use std::fs;
use std::io::Write;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub url: String,
    #[arg(long)]
    pub num: usize
}

const DIR  : &str = "benchmark-results/cli/callgrind/";
const STDIN: &str = "benchmark-results/cli/callgrind/stdin.txt";

impl Args {
    pub fn r#do(self) -> fs::File {
        fs::create_dir_all(DIR).unwrap();
    
        let mut stdin = fs::OpenOptions::new().create(true).write(true).truncate(true).open(STDIN).unwrap();

        for _ in 0..self.num {
            writeln!(stdin, "{}", self.url).unwrap();
        }

        drop(stdin);

        let out = format!("{DIR}/callgrind.out-{}-{}", self.name, self.num);

        Command::new("valgrind")
            .args([
                "-q",
                "--tool=callgrind",
                "--separate-threads=yes",
                &format!("--callgrind-out-file={out}"),
                "target/release/url-cleaner"
            ])
            .stdin(fs::File::open(STDIN).unwrap())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap().wait().unwrap();

        fs::remove_file(STDIN).unwrap();

        fs::File::open(out).unwrap()
    }
}

