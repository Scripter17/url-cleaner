//! Massif.

use super::prelude::*;

/// Massif.
#[derive(Debug, Parser)]
pub struct Args {
    /// The name of the benchmark.
    #[arg(long)]
    pub name: String,
    /// The task line.
    #[arg(long)]
    pub task: String,
    /// The number to clean per run.
    #[arg(long)]
    pub num: usize
}

/// The output directory.
const OUT: &str = "urlc-tool/out/bench/cli/massif";

impl Args {
    /// Do the command.
    pub fn r#do(self) -> String {
        std::fs::create_dir_all(OUT).unwrap();

        let stdin = crate::bench::make_stdin(&self.task, self.num);

        let out = format!("{OUT}/massif.out-{}-{}", self.name, self.num);

        assert_eq!(Command::new("valgrind")
            .args([
                "-q",
                "--tool=massif",
                &format!("--massif-out-file={out}")
            ])
            .arg(BINDIR.join("url-cleaner"))
            .stdin(File::open(stdin.path()).unwrap())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap().wait().unwrap().code(), Some(0));

        out
    }
}
