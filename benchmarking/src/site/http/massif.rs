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

const DIR  : &str = "benchmark-results/site-http/massif/";
const STDIN: &str = "benchmark-results/site-http/massif/stdin.txt";

impl Args {
    pub fn r#do(self) -> fs::File {
        fs::create_dir_all(DIR).unwrap();
    
        let mut stdin = fs::OpenOptions::new().create(true).write(true).truncate(true).open(STDIN).unwrap();

        write!(stdin, r#"{{"tasks":["#);

        let task_json = serde_json::to_string(&self.url).unwrap();

        for i in 0..self.num {
            if i != 0 {
                write!(stdin, ",");
            }
            write!(stdin, "{}", task_json);
        }

        write!(stdin, r#"]}}"#);

        drop(stdin);

        let out = format!("{DIR}/massif.out-{}-{}", self.name, self.num);

        let server = crate::KillOnDrop(Command::new("valgrind")
            .args([
                "-q",
                "--tool=massif",
                &format!("--massif-out-file={out}"),
                "target/release/url-cleaner-site",
                "--port", "9148",
                "--max-payload", "1GiB"
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap());

        for i in 0..10 {
            match std::net::TcpStream::connect("127.0.0.1:9148") {
                Ok(_) => {
                    Command::new("curl")
                        .args([
                            "http://127.0.0.1:9148/clean",
                            "--json", &format!("@{STDIN}")
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
