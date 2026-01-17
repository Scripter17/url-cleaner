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
    pub pages: usize
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let client = reqwest::blocking::Client::builder()
            .user_agent("Firefox")
            .build().unwrap();

        for host in std::io::stdin().lock().lines().map(Result::unwrap) {
            let out_dir = format!("urlc-tool/out/get/reddit/{host}");

            std::fs::create_dir_all(&out_dir).unwrap();

            let mut after = String::new();

            for page in 1..=self.pages {
                eprintln!("{host} - {page}");

                let out = format!("{out_dir}/{page}.json");

                if !std::fs::exists(&out).unwrap() || std::fs::metadata(&out).unwrap().len() == 0 {
                    let mut file = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(&out).unwrap();
                    let mut sleep = std::time::Duration::from_secs(1);

                    loop {
                        match client.get(format!("https://old.reddit.com/domain/{host}/.json?{after}limit=100")).send() {
                            Ok(mut res) => {
                                res.copy_to(&mut file).unwrap();
                                break;
                            },
                            Err(_) => sleep *= 2
                        }
                        eprintln!("Sleeping for {sleep:?}");
                        std::thread::sleep(sleep);
                    }
                }

                let data = serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(out).unwrap()).unwrap();

                for child in data["data"]["children"].as_array().unwrap() {
                    let mut escaped = child["data"]["url"].as_str().unwrap();
                    let mut unescaped = String::new();
                    while !escaped.is_empty() {
                        if let Some(rest) = escaped.strip_prefix("&amp;" ) {unescaped.push('&' ); escaped = rest; continue;}
                        if let Some(rest) = escaped.strip_prefix("&quot;") {unescaped.push('"' ); escaped = rest; continue;}
                        if let Some(rest) = escaped.strip_prefix("&gt;"  ) {unescaped.push('>' ); escaped = rest; continue;}
                        if let Some(rest) = escaped.strip_prefix("&lt;"  ) {unescaped.push('<' ); escaped = rest; continue;}
                        if let Some(rest) = escaped.strip_prefix("&#x27;") {unescaped.push('\''); escaped = rest; continue;}
                        let mut chars = escaped.chars();
                        unescaped.push(chars.next().unwrap());
                        escaped = chars.as_str();
                    }
                    println!("{unescaped}");
                }

                match data["data"]["after"].as_str() {
                    Some(x) => after = format!("after={x}&"),
                    None => break
                }
            }
        }
    }
}
