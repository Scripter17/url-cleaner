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

const DIR  : &str = "benchmark-results/site-http/hyperfine/";
const STDIN: &str = "benchmark-results/site-http/hyperfine/stdin.txt";

impl Args {
    pub fn r#do(self) -> fs::File {
        fs::create_dir_all(DIR).unwrap();
    
        let mut stdin = fs::OpenOptions::new().create(true).write(true).truncate(true).open(STDIN).unwrap();

        write!(stdin, r#"{{"tasks":["#).unwrap();

        let task_json = serde_json::to_string(&self.url).unwrap();

        for i in 0..self.num {
            if i != 0 {
                write!(stdin, ",").unwrap();
            }
            write!(stdin, "{}", task_json).unwrap();
        }

        write!(stdin, r#"]}}"#).unwrap();

        drop(stdin);

        let server = crate::KillOnDrop(Command::new("target/release/url-cleaner-site")
            .args(["--port", "9148", "--max-payload", "1GiB"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap());

        let out = format!("{DIR}/hyperfine.out-{}-{}.json", self.name, self.num);

        for _ in 0..10 {
            match std::net::TcpStream::connect("127.0.0.1:9148") {
                Ok(_) => {
                    Command::new("hyperfine")
                        .args([
                            "--command-name", &self.name,
                            "curl http://127.0.0.1:9148/clean --json @-",
                            "--input", STDIN,
                            "--export-json", &out
                        ])
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .spawn().unwrap().wait().unwrap();

                    drop(server);

                    return fs::File::open(out).unwrap();
                },
                Err(_) => std::thread::sleep(std::time::Duration::from_secs(1))
            }
        }
        panic!("Server not found???")
    }
}

