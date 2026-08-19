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
    pub pages: usize,
    /// The cookie to bypass anti-bot stuff.
    #[arg(long)]
    pub cookie: String,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let client = reqwest::blocking::Client::builder().default_headers([
		    ("user-agent".try_into().unwrap(), "Firefox"  .try_into().unwrap()),
		    ("sec-gpc"   .try_into().unwrap(), "1"        .try_into().unwrap()),
		    ("dnt"       .try_into().unwrap(), "1"        .try_into().unwrap()),
		    ("cookie"    .try_into().unwrap(), self.cookie.try_into().unwrap()),
        ].into_iter().collect())
            .redirect(reqwest::redirect::Policy::none())
            .referer(false).build().unwrap();

        let mut url = BetterUrl::new("https://old.reddit.com/domain/example.com/.json?limit=100").unwrap();

        for host in std::io::stdin().lock().lines().map(Result::unwrap) {
            eprint!("{host}:");
            std::io::stderr().flush().unwrap();

            url.set_path_segment(1, Some(&*host)).unwrap();
            url.set_query_param("after", 0, None::<&str>).unwrap();

            for page in 1..=self.pages {
                eprint!(" {page}");
                std::io::stderr().flush().unwrap();

                let mut sleep = std::time::Duration::from_secs(10);

                let data = loop {
                    match client.get(url.as_str()).send() {
                        Ok(res) if res.status() == 200 => break res.bytes().unwrap(),
                        e => {
                            eprint!(" ... {e:?}");
                            std::io::stderr().flush().unwrap();
                            std::thread::sleep(sleep);
                            sleep *= 2
                        }
                    }
                };

                new_file(format!("urlc-tool/tmp/get/reddit/{host}/{page}.json")).write_all(&data).unwrap();

                let response = serde_json::from_slice::<Response>(&data).unwrap();

                let mut out = new_file(format!("urlc-tool/out/get/reddit/{host}/{page}.txt"));

                for child in response.data.children {
                    let url = unescape_html(child.data.url).unwrap();

                    writeln!(out, "{url}").unwrap();
                    println!("{url}");
                }

                out.flush().unwrap();

                match response.data.after {
                    Some(after) => {url.set_query_param("after", 0, Some(Some(&after))).unwrap();},
                    None => break
                }
            }

            eprintln!();
        }
    }
}

/// A response.
#[derive(Debug, Deserialize)]
pub struct Response {
    /// The [`Data`].
    pub data: Data
}

/// A [`Response`]'s data.
#[derive(Debug, Deserialize)]
pub struct Data {
    /// The [`Child`]ren.
    pub children: Vec<Child>,
    /// The value to put in the `after` query param for the next page.
    pub after: Option<String>,
}

/// A child.
#[derive(Debug, Deserialize)]
pub struct Child {
    /// The [`ChildData`].
    pub data: ChildData,
}

/// A [`Child`]'s data.
#[derive(Debug, Deserialize)]
pub struct ChildData {
    /// The URL.
    pub url: String,
}
