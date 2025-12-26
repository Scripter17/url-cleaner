//! Suite.

use super::prelude::*;

/// Generate benchmarks.md, writing to STDOUT.
#[derive(Debug, Parser)]
pub struct Args {
    /// The benchmarks file.
    #[arg(long)]
    pub benchmarks: Option<PathBuf>,
    /// The regex to filter names with.
    #[arg(long, default_value = "")]
    pub filter: String,
    /// Disable compiling the frontends.
    #[arg(long)]
    pub no_compile: bool,
    /// The number locale.
    #[arg(long, default_value = "en")]
    pub number_locale: Locale,

    /// CLI.
    #[arg(long)]
    pub cli: bool,
    /// Site HTTP.
    #[arg(long)]
    pub site_http: bool,
    /// Site WebSocket.
    #[arg(long)]
    pub site_ws: bool,

    /// Hyperfine.
    #[arg(long)]
    pub hyperfine: bool,
    /// Callgrind.
    #[arg(long)]
    pub callgrind: bool,
    /// massif.
    #[arg(long)]
    pub massif: bool,

    /// Hyperfine nums.
    #[arg(long, requires = "hyperfine", num_args = 1.., default_values_t = [0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000])]
    pub hyperfine_nums: Vec<usize>,
    /// callgrind nums.
    #[arg(long, requires = "callgrind", num_args = 1.., default_values_t = [0, 10_000])]
    pub callgrind_nums: Vec<usize>,
    /// Massif nums.
    #[arg(long, requires = "massif"   , num_args = 1.., default_values_t = [0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000])]
    pub massif_nums: Vec<usize>,

    /// Hyperfine runs.
    #[arg(long, requires = "hyperfine")]
    pub hyperfine_runs: Option<usize>,
    /// Hyperfine warmup.
    #[arg(long, requires = "hyperfine")]
    pub hyperfine_warmup: Option<usize>,
}

/// A benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Benchmark<'a> {
    /// The name.
    name: &'a str,
    /// The description.
    desc: &'a str,
    /// The task.
    task: &'a str
}

/// The bundled benchmarks.
const BUNDLED_BENCHMARKS: &str = include_str!("bundled-benchmarks.txt");

/// Print to and flsuh STDOUT.
macro_rules! printflush {
    ($($x:tt)*) => {
        print!($($x)*);
        std::io::stdout().flush().unwrap();
    }
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let benchmarks_string = match self.benchmarks {
            Some(path) => Cow::Owned(std::fs::read_to_string(path).unwrap()),
            None => Cow::Borrowed(BUNDLED_BENCHMARKS)
        };

        let filter = Regex::new(&self.filter).unwrap();

        let benchmarks = benchmarks_string.lines()
            .map(|line| {
                let mut parts = line.splitn(3, '\t');
                Benchmark {
                    name: parts.next().unwrap(),
                    desc: parts.next().unwrap(),
                    task: parts.next().unwrap()
                }
            })
            .filter(|Benchmark {name, ..}| filter.find(name).is_some())
            .collect::<Vec<Benchmark<'_>>>();

        let hyperfine_table_header = table_header(&self.hyperfine_nums, &self.number_locale);
        let callgrind_table_header = table_header(&self.callgrind_nums, &self.number_locale);
        let massif_table_header    = table_header(&self.massif_nums   , &self.number_locale);

        if !self.no_compile && (self.cli || self.site_http || self.site_ws) {
            crate::compile::Args {
                frontends: crate::compile::Frontends {
                    cli: self.cli,
                    site: self.site_http || self.site_ws,
                    site_ws_client: self.site_ws,
                    discord: false
                }
            }.r#do();
        }

        println!("# Benchmarks");
        println!();
        println!("Measurements of how fast URL Cleaner's frontends are, as seen on the following hardware:");
        println!();
        println!("```");
        assert_eq!(Command::new("neofetch")
            .args(["distro", "kernel", "model", "cpu", "memory"])
            .spawn().unwrap().wait().unwrap().code(), Some(0));
        println!("```");
        println!();
        println!("## Tasks");
        println!();
        println!("The tasks that are benchmarked.");
        println!();
        println!("|Name|Description|Task|");
        println!("|:--|:--|:--|");
        for Benchmark {name, desc, task} in &benchmarks {
            println!("|{name}|{desc}|`{task}`|");
        }
        println!();

        if self.cli {
            println!("## Cli");
            println!();

            if self.hyperfine {
                println!("### Speed");
                println!();
                println!("Measured in milliseconds.");
                println!();

                println!("{hyperfine_table_header}");
                for benchmark in &benchmarks {
                    printflush!("|{}|", benchmark.name);
                    for num in &self.hyperfine_nums {
                        print_hyperfine_entry(crate::bench::cli::hyperfine::Args {
                            name: benchmark.name.to_owned(),
                            task: benchmark.task.to_owned(),
                            num: *num,
                            runs: self.hyperfine_runs,
                            warmup: self.hyperfine_warmup,
                        }.r#do());
                    }
                    println!();
                }
                println!();
            }

            if self.callgrind {
                println!("### Callgrind");
                println!();
                println!("No info to show.");
                println!();

                println!("{callgrind_table_header}");
                for benchmark in &benchmarks {
                    printflush!("|{}|", benchmark.name);
                    for num in &self.callgrind_nums {
                        print_callgrind_entry(crate::bench::cli::callgrind::Args {
                            name: benchmark.name.to_owned(),
                            task: benchmark.task.to_owned(),
                            num: *num,
                        }.r#do());
                    }
                    println!();
                }
                println!();
            }

            if self.massif {
                println!("### Peak memory usage");
                println!();
                println!("Measured in bytes.");
                println!();

                println!("{massif_table_header}");
                for benchmark in &benchmarks {
                    printflush!("|{}|", benchmark.name);
                    for num in &self.massif_nums {
                        print_massif_entry(crate::bench::cli::massif::Args {
                            name: benchmark.name.to_owned(),
                            task: benchmark.task.to_owned(),
                            num: *num,
                        }.r#do(), &self.number_locale);
                    }
                    println!();
                }
                println!();
            }
        }

        if self.site_http {
            println!("## Site HTTP");
            println!();
            println!("The max payload was increased from 25MiB to 1GiB.");
            println!();
            println!("Below is a table of how many of each task can actually fit in the default and increased limits.");
            println!();
            println!("|Name|Bytes|Lines in 25MiB|Lines in 1GiB|");
            println!("|:--|--:|--:|--:|");
            for benchmark in &benchmarks {
                // Assuming line separator of `\n` and no trailing empty line.
                // size = num * ( len + 1 ) - 1
                // num  = ( size + 1 ) / ( len + 1 )

                println!(
                    "|{}|`{}`|`{}`|`{}`|",
                    benchmark.name,
                    benchmark.task.len().to_formatted_string(&self.number_locale),
                    ((  25 * 1024 * 1024 + 1) / (benchmark.task.len() + 1)).to_formatted_string(&self.number_locale),
                    ((1024 * 1024 * 1024 + 1) / (benchmark.task.len() + 1)).to_formatted_string(&self.number_locale)
                );
            }
            println!();

            if self.hyperfine {
                println!("### Speed");
                println!();
                println!("Measured in milliseconds.");
                println!();

                println!("{hyperfine_table_header}");
                for benchmark in &benchmarks {
                    printflush!("|{}|", benchmark.name);
                    for num in &self.hyperfine_nums {
                        print_hyperfine_entry(crate::bench::site::http::hyperfine::Args {
                            name: benchmark.name.to_owned(),
                            task: benchmark.task.to_owned(),
                            num: *num,
                            runs: self.hyperfine_runs,
                            warmup: self.hyperfine_warmup,
                        }.r#do());
                    }
                    println!();
                }
                println!();
            }

            if self.callgrind {
                println!("### Callgrind");
                println!();
                println!("No info to show.");
                println!();

                println!("{callgrind_table_header}");
                for benchmark in &benchmarks {
                    printflush!("|{}|", benchmark.name);
                    for num in &self.callgrind_nums {
                        print_callgrind_entry(crate::bench::site::http::callgrind::Args {
                            name: benchmark.name.to_owned(),
                            task: benchmark.task.to_owned(),
                            num: *num,
                        }.r#do());
                    }
                    println!();
                }
                println!();
            }

            if self.massif {
                println!("### Peak memory usage");
                println!();
                println!("Measured in bytes.");
                println!();

                println!("{massif_table_header}");
                for benchmark in &benchmarks {
                    printflush!("|{}|", benchmark.name);
                    for num in &self.massif_nums {
                        print_massif_entry(crate::bench::site::http::massif::Args {
                            name: benchmark.name.to_owned(),
                            task: benchmark.task.to_owned(),
                            num: *num,
                        }.r#do(), &self.number_locale);
                    }
                    println!();
                }
                println!();
            }
        }

        if self.site_ws {
            println!("## Site WebSocket");
            println!();
            println!("Please note that a custom client is used to send multiple task configs per message, unlike WebSocat which can only do 1 per message.");
            println!();
            println!("This reduces the overhead of using WebSockets DRAMATICALLY. If your client isn't bundling tasks it'll likely be several times slower.");
            println!();

            if self.hyperfine {
                println!("### Speed");
                println!();
                println!("Measured in milliseconds.");
                println!();

                println!("{hyperfine_table_header}");
                for benchmark in &benchmarks {
                    printflush!("|{}|", benchmark.name);
                    for num in &self.hyperfine_nums {
                        print_hyperfine_entry(crate::bench::site::websocket::hyperfine::Args {
                            name: benchmark.name.to_owned(),
                            task: benchmark.task.to_owned(),
                            num: *num,
                            runs: self.hyperfine_runs,
                            warmup: self.hyperfine_warmup,
                        }.r#do());
                    }
                    println!();
                }
                println!();
            }

            if self.callgrind {
                println!("### Callgrind");
                println!();
                println!("No info to show.");
                println!();

                println!("{callgrind_table_header}");
                for benchmark in &benchmarks {
                    printflush!("|{}|", benchmark.name);
                    for num in &self.callgrind_nums {
                        print_callgrind_entry(crate::bench::site::websocket::callgrind::Args {
                            name: benchmark.name.to_owned(),
                            task: benchmark.task.to_owned(),
                            num: *num,
                        }.r#do());
                    }
                    println!();
                }
                println!();
            }

            if self.massif {
                println!("### Peak memory usage");
                println!();
                println!("Measured in bytes.");
                println!();

                println!("{massif_table_header}");
                for benchmark in &benchmarks {
                    printflush!("|{}|", benchmark.name);
                    for num in &self.massif_nums {
                        print_massif_entry(crate::bench::site::websocket::massif::Args {
                            name: benchmark.name.to_owned(),
                            task: benchmark.task.to_owned(),
                            num: *num,
                        }.r#do(), &self.number_locale);
                    }
                    println!();
                }
                println!();
            }
        }
    }
}

/// Generate a table header.
pub fn table_header(nums: &[usize], number_locale: &Locale) -> String {
    let mut ret = "|Name|".to_string();
    for num in nums {
        ret.push_str(&num.to_formatted_string(number_locale));
        ret.push('|');
    }
    ret.push_str("\n|:--|");
    for _ in nums {
        ret.push_str("--:|");
    }
    ret
}

/// Print a Hyperfine entry.
fn print_hyperfine_entry<P: AsRef<Path>>(path: P) {
    let data = std::fs::read_to_string(path).unwrap();
    let data = serde_json::from_str::<serde_json::Value>(&data).unwrap()["results"][0].take();
    printflush!("`{:.1}`|", data["mean"].as_f64().unwrap() * 1000.0);
}

/// Print a Callgrind entry.
fn print_callgrind_entry<P: AsRef<Path>>(_: P) {
    printflush!("`...`|");
}

/// Print a Massif entry.
fn print_massif_entry<P: AsRef<Path>>(path: P, number_locale: &Locale) {
    let mut max = 0;

    for line in BufReader::new(File::open(path).unwrap()).lines() {
        if let Some(num) = line.unwrap().strip_prefix("mem_heap_B=") {
            max = max.max(num.parse().unwrap());
        }
    }

    printflush!("`{}`|", max.to_formatted_string(number_locale));
}
