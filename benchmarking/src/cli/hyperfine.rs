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

const DIR  : &str = "benchmark-results/cli/hyperfine/";
const STDIN: &str = "benchmark-results/cli/hyperfine/stdin.txt";

impl Args {
    pub fn r#do(self) -> fs::File {
        fs::create_dir_all(DIR).unwrap();
    
        let mut stdin = fs::OpenOptions::new().create(true).write(true).truncate(true).open(STDIN).unwrap();

        for _ in 0..self.num {
            writeln!(stdin, "{}", self.url).unwrap();
        }

        drop(stdin);

        let out = format!("{DIR}/hyperfine.out-{}-{}.json", self.name, self.num);

        Command::new("hyperfine")
            .args([
                "--command-name", &self.name,
                "target/release/url-cleaner",
                "--input", STDIN,
                "--export-json", &out
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap().wait().unwrap();

        fs::File::open(out).unwrap()
    }
}
