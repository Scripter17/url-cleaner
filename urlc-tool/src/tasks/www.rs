//! Www.

use super::prelude::*;

/// Figure out which websites need to be example.com and which need to be www.example.com.
///
/// Takes in lines of domain origins from STDIN.
#[derive(Debug, Parser)]
pub struct Args {
    /// The number of threads to use.
    #[arg(long)]
    pub threads: usize,
}

/// Thing.
#[derive(Debug, Clone, Copy)]
enum Thing {
    /// 1xx.
    Info,
    /// 2xx.
    Stay,
    /// 3xx and a Location header of the other.
    Swap,
    /// 3xx.
    Redirect,
    /// 4xx.
    ClientError,
    /// 5xx.
    ServerError,
    /// Err.
    NetworkError,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let client = reqwest::blocking::Client::builder().default_headers([
		    ("user-agent".try_into().unwrap(), "Firefox".try_into().unwrap()),
		    ("sec-gpc"   .try_into().unwrap(), "1"      .try_into().unwrap()),
		    ("dnt"       .try_into().unwrap(), "1"      .try_into().unwrap()),
        ].into_iter().collect())
            .redirect(reqwest::redirect::Policy::none())
            .referer(false)
            .timeout(Some(std::time::Duration::from_secs(5)))
            .build().unwrap();

        let (iss, irs) = (0..self.threads).map(|_| std::sync::mpsc::channel::<String>()).collect::<(Vec<_>, Vec<_>)>();
        let (os, or) = std::sync::mpsc::channel();
        let oo = &std::fs::OpenOptions::new().create(true).read(true).write(true).clone();

        std::thread::scope(|s| {
            s.spawn(move || {
                for (i, domain) in std::io::stdin().lines().map(Result::unwrap).enumerate() {
                    iss[i % iss.len()].send(domain).expect("???");
                }
            });

            for (ir, os) in irs.into_iter().zip(std::iter::repeat(os)) {
                let client = client.clone();

                s.spawn(move || {
                    while let Ok(domain) = ir.recv() {
                        let without = format!("https://{domain}/");
                        let with    = format!("https://www.{domain}/");

                        let dir          = format!("urlc-tool/tmp/tasks/www/{domain}");
                        let without_file = format!("{dir}/without.txt"               );
                        let with_file    = format!("{dir}/with.txt"                  );

                        std::fs::create_dir_all(dir).expect("???");



                        let mut without_file = oo.open(without_file).expect("???");
                        let     without_res  = client.get(&without).send();

                        writeln!(without_file, "{without_res:?}").expect("???");

                        let without_result = match without_res {
                            Ok(mut res) => {
                                let _ = std::io::copy(&mut res, &mut without_file);

                                match res.status().as_u16() {
                                      0..200                                                                                => Thing::Info,
                                    200..300                                                                                => Thing::Stay,
                                    300..400 if let Some(location) = res.headers().get("location") && *location == *with    => Thing::Swap,
                                    300..400                                                                                => Thing::Redirect,
                                    400..500                                                                                => Thing::ClientError,
                                    500..                                                                                   => Thing::ServerError,
                                }
                            },
                            Err(_) => Thing::NetworkError,
                        };



                        let mut with_file = oo.open(with_file).expect("???");
                        let     with_res  = client.get(&with).send();

                        writeln!(with_file, "{with_res:?}").expect("???");

                        let with_result = match with_res {
                            Ok(mut res) => {
                                let _ = std::io::copy(&mut res, &mut with_file);

                                match res.status().as_u16() {
                                      0..200                                                                                => Thing::Info,
                                    200..300                                                                                => Thing::Stay,
                                    300..400 if let Some(location) = res.headers().get("location") && *location == *without => Thing::Swap,
                                    300..400                                                                                => Thing::Redirect,
                                    400..500                                                                                => Thing::ClientError,
                                    500..                                                                                   => Thing::ServerError,
                                }
                            },
                            Err(_) => Thing::NetworkError,
                        };

                        os.send((domain, without_result, with_result)).expect("???");
                    }
                });
            }

            s.spawn(move || {
                while let Ok((domain, without_result, with_result)) = or.recv() {
                    println!("{domain}\t{without_result:?}\t{with_result:?}");
                }
            });
        });
    }
}
