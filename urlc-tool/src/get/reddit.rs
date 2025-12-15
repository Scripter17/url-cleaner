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
    #[arg(long, default_value_t = 1)]
    pub pages: usize,
    /// The mode.
    #[arg(long, value_enum, default_value_t = Default::default())]
    pub mode: Mode,
    /// The format.
    #[arg(long, value_enum, default_value_t = Default::default())]
    pub format: Format
}

/// The temp directory.
const TMP: &str = "urlc-tool/tmp/get/reddit/";
/// The output directory.
const OUT: &str = "urlc-tool/out/get/reddit/";

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        std::fs::create_dir_all(TMP).unwrap();
        std::fs::create_dir_all(OUT).unwrap();

        for host in std::io::stdin().lock().lines().map(Result::unwrap) {
            let mut after = String::new();
            let mut i = 0;

            for page in 1..=self.pages {
                let out = format!("{TMP}/{host}-{page}.json");

                let has_page = std::fs::exists(&out).unwrap() && std::fs::metadata(&out).unwrap().len() != 0;

                match (self.mode, has_page) {
                    (Mode::Normal, _) | (Mode::Continue, false) => {
                        eprintln!("Getting {host} {page}");

                        let mut sleep = std::time::Duration::from_secs(10);

                        loop {
                            match Command::new("curl")
                                .arg("-sf")
                                .args(["-H", "User-Agent: Firefox"])
                                .args(["-o", &out])
                                .arg(format!("https://old.reddit.com/domain/{host}/.json?{after}limit=100"))
                                .spawn().unwrap().wait().unwrap().code() {
                                Some(0) => break,
                                Some(22) => {
                                    eprintln!("Error: Sleeping for {sleep:?}");
                                    std::thread::sleep(sleep);
                                    sleep += sleep;
                                },
                                x => panic!("Unknown curl exit code: {x:?}")
                            }
                        }
                    },
                    (Mode::Continue | Mode::Regenerate, true) => eprintln!("Already have {host} {page}"),
                    (Mode::Regenerate, false) => {
                        eprintln!("Missing {host} {page}; Skipping");
                        continue;
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
                    i+=1;
                    match self.format {
                        Format::Quick => println!("{unescaped}"),
                        Format::Suite => println!("{host}-{i}\t\t{unescaped}")
                    }
                }

                match data["data"]["after"].as_str() {
                    Some(x) => after = format!("after={x}&"),
                    None => break
                }
            }
        }
    }
}
