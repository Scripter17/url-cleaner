use std::fs;
use std::process::Command;
use std::io::{Read, Write, BufReader, BufRead};
use num_format::{Locale, ToFormattedString};

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(long)]
    cli: bool,
    #[arg(long)]
    site_http: bool,
    #[arg(long)]
    site_ws: bool,

    #[arg(long)]
    hyperfine: bool,
    #[arg(long)]
    massif: bool
}

struct Task {
    name: &'static str,
    desc: &'static str,
    url: &'static str
}

const TASKS: &[Task] = &[
    Task {
        name: "Baseline",
        desc: "An already clean URL",
        url: "https://example.com"
    },
    Task {
        name: "UTPs",
        desc: "Baseline with some universal tracking parameters",
        url: "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id"
    },
    Task {
        name: "Amazon",
        desc: "An amazon product listing",
        url: "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8"
    },
    Task {
        name: "Google",
        desc: "A google search result",
        url: "https://www.google.com/search?q=url+cleaner&sca_esv=de6549fe37924183&ei=eRAYabb6O7Gb4-EP79Xe6A8&ved=0ahUKEwj2mqWLt_OQAxWxzTgGHe-qF_0Q4dUDCBE&oq=url+cleaner&gs_lp=Egxnd3Mtd2l6LXNlcnAiC3VybCBjbGVhbmVySABQAFgAcAB4AZABAJgBAKABAKoBALgBDMgBAJgCAKACAJgDAJIHAKAHALIHALgHAMIHAMgHAA&sclient=gws-wiz-serp"
    }
];

const HYPERFINE_NUMS: &[usize] = &[1_000, 10_000, 100_000];
const MASSIF_NUMS: &[usize] = &[0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];

macro_rules! printflush {
    ($($x:tt)*) => {
        print!($($x)*);
        std::io::stdout().flush().unwrap();
    }
}

impl Args {
    pub fn r#do(self) {
        match fs::remove_dir_all("benchmark-results") {
            Ok(_) => {},
            Err(e) if matches!(e.kind(), std::io::ErrorKind::NotFound) => {},
            x => x.unwrap()
        };

        println!("# Benchmarks\n");

        println!("## Tasks\n");

        println!("|Name|Description|URL|");
        println!("|:--:|:--:|:--|");
        for Task {name, desc, url} in TASKS {
            println!("|{name}|{desc}|`{url}`|");
        }
        println!();

        if self.cli {
            Command::new("cargo")
                .args(["build", "-r", "--bin", "url-cleaner"])
                .args(crate::CARGO_CONFIG)
                .stdout(std::io::stderr())
                .stderr(std::io::stderr())
                .spawn().unwrap().wait().unwrap();

            if self.hyperfine {
                println!("## CLI Hyperfine\n");

                print_hyperfine_header();
                for task in TASKS {
                    for num in HYPERFINE_NUMS {
                        printflush!("|{}|{num}|", task.name);
                        print_hyperfine_entry(crate::cli::hyperfine::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do());
                    }
                }
            }

            if self.massif {
                println!("\n## CLI Massif\n");

                print_massif_header();
                for task in TASKS {
                    printflush!("|{}|", task.name);
                    for num in MASSIF_NUMS {
                        print_massif_entry   (crate::cli::massif::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do());
                    }
                    println!();
                }
            }
        }

        if self.site_http {
            Command::new("cargo")
                .args(["build", "-r", "--bin", "url-cleaner-site"])
                .args(crate::CARGO_CONFIG)
                .stdout(std::io::stderr())
                .stderr(std::io::stderr())
                .spawn().unwrap().wait().unwrap();

            if self.hyperfine {
                println!("\n## Site HTTP Hyperfine\n");
                println!("The max payload was increased from 25MiB to 1GiB.\n");
                println!("While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.\n");

                print_hyperfine_header();
                for task in TASKS {
                    for num in HYPERFINE_NUMS {
                        printflush!("|{}|{num}|", task.name);
                        print_hyperfine_entry(crate::site::http::hyperfine::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do());
                    }
                }
            }

            if self.massif {
                println!("\n## Site HTTP Massif\n");
                println!("The max payload was increased from 25MiB to 1GiB.\n");
                println!("While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.\n");

                print_massif_header();
                for task in TASKS {
                    printflush!("|{}|", task.name);
                    for num in MASSIF_NUMS {
                        print_massif_entry   (crate::site::http::massif::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do());
                    }
                    println!();
                }
            }
        }

        if self.site_ws {
            Command::new("cargo")
                .args(["build", "-r", "--bin", "url-cleaner-site"])
                .args(crate::CARGO_CONFIG)
                .stdout(std::io::stderr())
                .stderr(std::io::stderr())
                .spawn().unwrap().wait().unwrap();

            if self.hyperfine {
                println!("\n## Site Websocket Hyperfine\n");

                print_hyperfine_header();
                for task in TASKS {
                    for num in HYPERFINE_NUMS {
                        printflush!("|{}|{num}|", task.name);
                        print_hyperfine_entry(crate::site::websocket::hyperfine::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do());
                    }
                }
            }

            if self.massif {
                println!("\n## Site Websocket Massif\n");

                print_massif_header();
                for task in TASKS {
                    printflush!("|{}|", task.name);
                    for num in MASSIF_NUMS {
                        print_massif_entry   (crate::site::websocket::massif::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do());
                    }
                    println!();
                }
            }
        }
    }
}

fn print_hyperfine_header() {
    println!("Time it takes to do various amounts of the tasks, measured in milliseconds.");
    println!();
    println!("|Name|Count|Min|Mean|Max|Std. Dev.|");
    println!("|:--:|:--:|--:|--:|--:|--:|");
}

fn print_hyperfine_entry(mut file: fs::File) {
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let data = serde_json::from_str::<serde_json::Value>(&data).unwrap()["results"][0].take();
    println!("`{:.1}`|`{:.1}`|`{:.1}`|`{:.1}`|",
        data["min"   ].as_f64().unwrap() * 1000.0,
        data["mean"  ].as_f64().unwrap() * 1000.0,
        data["max"   ].as_f64().unwrap() * 1000.0,
        data["stddev"].as_f64().unwrap() * 1000.0
    )
}

fn print_massif_header() {
    println!("Peak memory usage to do various amounts of the tasks, measured in bytes.");
    println!();
    printflush!("|Name|");
    for num in MASSIF_NUMS {printflush!("{num}|");}
    println!();
    printflush!("|:--:|");
    for _ in MASSIF_NUMS {printflush!("--:|");}
    println!();
}

fn print_massif_entry(mut file: fs::File) {
    let mut max = 0;

    for line in BufReader::new(file).lines() {
        if let Some(num) = line.unwrap().strip_prefix("mem_heap_B=") {
            max = max.max(num.parse().unwrap());
        }
    }

    printflush!("`{}`|", max.to_formatted_string(&Locale::en));
}
