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
    callgrind: bool,
    #[arg(long)]
    massif: bool,

    #[arg(long, num_args = 1.., default_values_t = [0, 1, 10, 100, 1_000, 10_000, 100_000])]
    hyperfine_nums: Vec<usize>,
    #[arg(long, num_args = 1.., default_values_t = [0, 10_000])]
    callgrind_nums: Vec<usize>,
    #[arg(long, num_args = 1.., default_values_t = [0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000])]
    massif_nums: Vec<usize>
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

macro_rules! printflush {
    ($($x:tt)*) => {
        print!($($x)*);
        std::io::stdout().flush().unwrap();
    }
}

impl Args {
    pub fn r#do(self) {
        let mut time = std::time::Instant::now();

        match fs::remove_dir_all("benchmark-results") {
            Ok(_) => {},
            Err(e) if matches!(e.kind(), std::io::ErrorKind::NotFound) => {},
            x => x.unwrap()
        };

        println!("# Benchmarks");
        println!();
        println!("As measured on a thinkpad T460S (from 2016) running Kubuntu.");
        println!();
        println!("## Tasks");
        println!();
        println!("The tasks that are benchmarks");
        println!();
        println!("|Name|Description|URL|");
        println!("|:--:|:--:|:--|");
        for Task {name, desc, url} in TASKS {
            println!("|{name}|{desc}|`{url}`|");
        }
        println!();

        if self.cli {
            assert!(Command::new("cargo")
                .args(["+stable", "build", "-r", "--bin", "url-cleaner"])
                .args(crate::CARGO_CONFIG)
                .stdout(std::io::stderr())
                .stderr(std::io::stderr())
                .spawn().unwrap().wait().unwrap().success());

            eprintln!("{:?}", time.elapsed());
            time = std::time::Instant::now();

            if self.hyperfine {
                println!("## CLI Hyperfine");
                println!();
                println!("Time it takes to do various amounts of the tasks, measured in milliseconds.");
                println!();

                print_hyperfine_header(&self.hyperfine_nums);
                for task in TASKS {
                    printflush!("|{}|", task.name);
                    for num in &self.hyperfine_nums {
                        print_hyperfine_entry(crate::cli::hyperfine::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do());
                    }
                    println!();

                    std::thread::sleep(std::time::Duration::from_millis(1));
                    eprintln!("{:?}", time.elapsed());
                    time = std::time::Instant::now();
                }
                println!();
            }

            if self.callgrind {
                eprintln!("CLI Callgrind");

                for task in TASKS {
                    eprintln!("  {}", task.name);
                    for num in &self.callgrind_nums {
                        eprintln!("    {num}");
                        crate::cli::callgrind::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do();

                        std::thread::sleep(std::time::Duration::from_millis(1));
                        eprintln!("{:?}", time.elapsed());
                        time = std::time::Instant::now();
                    }
                }
            }

            if self.massif {
                println!("## CLI Massif");
                println!();
                println!("Peak memory usage to do various amounts of the tasks, measured in bytes.");
                println!();

                print_massif_header(&self.massif_nums);
                for task in TASKS {
                    printflush!("|{}|", task.name);
                    for num in &self.massif_nums {
                        print_massif_entry(crate::cli::massif::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do());
                    }
                    println!();

                    std::thread::sleep(std::time::Duration::from_millis(1));
                    eprintln!("{:?}", time.elapsed());
                    time = std::time::Instant::now();
                }
                println!();
            }
        }

        if self.site_http {
            assert!(Command::new("cargo")
                .args(["+stable", "build", "-r", "--bin", "url-cleaner-site"])
                .args(crate::CARGO_CONFIG)
                .stdout(std::io::stderr())
                .stderr(std::io::stderr())
                .spawn().unwrap().wait().unwrap().success());

            eprintln!("{:?}", time.elapsed());
            time = std::time::Instant::now();

            if self.hyperfine {
                println!("## Site HTTP Hyperfine");
                println!();
                println!("Time it takes to do various amounts of the tasks, measured in milliseconds.");
                println!();
                println!("The max payload was increased from 25MiB to 1GiB.");
                println!();
                println!("While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.");
                println!();
                print_hyperfine_header(&self.hyperfine_nums);
                for task in TASKS {
                    printflush!("|{}|", task.name);
                    for num in &self.hyperfine_nums {
                        print_hyperfine_entry(crate::site::http::hyperfine::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do());
                    }
                    println!();

                    std::thread::sleep(std::time::Duration::from_millis(1));
                    eprintln!("{:?}", time.elapsed());
                    time = std::time::Instant::now();
                }
                println!();
            }

            if self.callgrind {
                eprintln!("Site HTTP Callgrind)");
                println!();

                for task in TASKS {
                    eprintln!("  {}", task.name);
                    for num in &self.callgrind_nums {
                        eprintln!("    {num}");
                        crate::site::http::callgrind::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do();

                        std::thread::sleep(std::time::Duration::from_millis(1));
                        eprintln!("{:?}", time.elapsed());
                        time = std::time::Instant::now();
                    }
                }
                println!();
            }

            if self.massif {
                println!("## Site HTTP Massif");
                println!();
                println!("Peak memory usage to do various amounts of the tasks, measured in bytes.");
                println!();
                println!("The max payload was increased from 25MiB to 1GiB.");
                println!();
                println!("While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.");
                println!();
                print_massif_header(&self.massif_nums);
                for task in TASKS {
                    printflush!("|{}|", task.name);
                    for num in &self.massif_nums {
                        print_massif_entry(crate::site::http::massif::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do());
                    }
                    println!();

                    std::thread::sleep(std::time::Duration::from_millis(1));
                    eprintln!("{:?}", time.elapsed());
                    time = std::time::Instant::now();
                }
                println!();
            }
        }

        if self.site_ws {
            assert!(Command::new("cargo")
                .args(["+stable", "build", "-r", "--bin", "url-cleaner-site"])
                .args(crate::CARGO_CONFIG)
                .stdout(std::io::stderr())
                .stderr(std::io::stderr())
                .spawn().unwrap().wait().unwrap().success());

            eprintln!("{:?}", time.elapsed());
            time = std::time::Instant::now();

            if self.hyperfine {
                println!("## Site WebSocket Hyperfine");
                println!();
                println!("Time it takes to do various amounts of the tasks, measured in milliseconds.");
                println!();
                print_hyperfine_header(&self.hyperfine_nums);
                for task in TASKS {
                    printflush!("|{}|", task.name);
                    for num in &self.hyperfine_nums {
                        print_hyperfine_entry(crate::site::websocket::hyperfine::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do());
                    }
                    println!();

                    std::thread::sleep(std::time::Duration::from_millis(1));
                    eprintln!("{:?}", time.elapsed());
                    time = std::time::Instant::now();
                }
                println!();
            }

            if self.callgrind {
                eprintln!("Site WebSocket Callgrind");

                for task in TASKS {
                    eprintln!("  {}", task.name);
                    for num in &self.callgrind_nums {
                        eprintln!("    {num}");
                        crate::site::websocket::callgrind::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do();

                        std::thread::sleep(std::time::Duration::from_millis(1));
                        eprintln!("{:?}", time.elapsed());
                        time = std::time::Instant::now();
                    }
                }
            }

            if self.massif {
                println!("## Site WebSocket Massif");
                println!();
                println!("Peak memory usage to do various amounts of the tasks, measured in bytes.");
                println!();
                print_massif_header(&self.massif_nums);
                for task in TASKS {
                    printflush!("|{}|", task.name);
                    for num in &self.massif_nums {
                        print_massif_entry(crate::site::websocket::massif::Args { name: task.name.into(), url: task.url.into(), num: *num}.r#do());
                    }
                    println!();

                    std::thread::sleep(std::time::Duration::from_millis(1));
                    eprintln!("{:?}", time.elapsed());
                    time = std::time::Instant::now();
                }
                println!();
            }
        }

        eprintln!("{:?}", time.elapsed());
    }
}

fn print_hyperfine_header(nums: &[usize]) {
    printflush!("|Name|");
    for num in nums {printflush!("{}|", num.to_formatted_string(&Locale::en));}
    println!();
    printflush!("|:--:|");
    for _ in nums {printflush!("--:|");}
    println!();
}

fn print_hyperfine_entry(mut file: fs::File) {
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let data = serde_json::from_str::<serde_json::Value>(&data).unwrap()["results"][0].take();
    printflush!("`{:.1}`|", data["mean"].as_f64().unwrap() * 1000.0);
}

fn print_massif_header(nums: &[usize]) {
    printflush!("|Name|");
    for num in nums {printflush!("{}|", num.to_formatted_string(&Locale::en));}
    println!();
    printflush!("|:--:|");
    for _ in nums {printflush!("--:|");}
    println!();
}

fn print_massif_entry(file: fs::File) {
    let mut max = 0;

    for line in BufReader::new(file).lines() {
        if let Some(num) = line.unwrap().strip_prefix("mem_heap_B=") {
            max = max.max(num.parse().unwrap());
        }
    }

    printflush!("`{}`|", max.to_formatted_string(&Locale::en));
}
