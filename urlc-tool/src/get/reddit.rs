//! Reddit.

use super::prelude::*;

/// Get tasks from reddit.
///
/// Reads STDIN as lines of domains.
///
/// Prints task/benchmark lines to STDOUT.
///
/// Prints progress info to STDERR.
#[derive(Debug, Parser)]
pub struct Args {
    /// The number of pages to get.
    #[arg(long)]
    pub pages: usize
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let client = reqwest::blocking::Client::builder().user_agent("Firefox").build().unwrap();

        for host in std::io::stdin().lock().lines().map(Result::unwrap) {
            eprint!("{host}:");
            std::io::stderr().flush().unwrap();

            let mut after = String::new();

            for page in 1..=self.pages {
                eprint!(" {page}");
                std::io::stderr().flush().unwrap();

                let mut sleep = std::time::Duration::from_secs(10);

                let data = loop {
                    match client.get(format!("https://old.reddit.com/domain/{host}/.json?{after}limit=100")).send() {
                        Ok(res) => break res.bytes().unwrap(),
                        Err(_) => {
                            std::thread::sleep(sleep);
                            sleep *= 2
                        }
                    }
                };

                new_file(format!("urlc-tool/tmp/get/reddit/{host}/{page}.json")).write_all(&data).unwrap();

                let data = serde_json::from_slice::<serde_json::Value>(&data).unwrap();

                let mut out = new_file(format!("urlc-tool/out/get/reddit/{host}/{page}.txt"));

                for child in data["data"]["children"].as_array().unwrap() {
                    let mut url = String::new();
                    let mut segments = child["data"]["url"].as_str().unwrap().split('&');

                    url.push_str(segments.next().unwrap());

                    for segment in segments {
                        let (code, rest) = segment.split_once(';').unwrap();
                        url.push(match code {
                            "amp"  => '&' ,
                            "quot" => '"' ,
                            "gt"   => '>' ,
                            "lt"   => '<' ,
                            "apos" => '\'',
                            x => panic!("Unknown escape: {x}")
                        });
                        url.push_str(rest);
                    }

                    writeln!(out, "{url}").unwrap();
                    println!("{url}");
                }

                out.flush().unwrap();

                match data["data"]["after"].as_str() {
                    Some(x) => after = format!("after={x}&"),
                    None => break
                }
            }

            eprintln!();
        }
    }
}
