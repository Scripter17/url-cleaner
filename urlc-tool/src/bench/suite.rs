//! Suite.

use super::prelude::*;

/// Generate a markdown document of benchmark details.
#[derive(Debug, Parser)]
pub struct Args {
    /// Don't compile stuff first.
    #[arg(long)]
    pub no_build: bool,
    /// The benchmarks file.
    #[arg(long, default_value = "urlc-tool/benchmarks.txt")]
    pub benchmarks: PathBuf,

    /// Benchmark CLI.
    #[arg(long)]
    pub cli: bool,
    /// The tools to benchmark CLI with.
    #[arg(long, num_args = 1.., default_values = ["hyperfine", "massif"])]
    pub cli_tools: Vec<cli::Tool>,

    /// Benchmark Site.
    #[arg(long)]
    pub site: bool,
    /// The tools to benchmark Site with.
    #[arg(long, num_args = 1.., default_values = ["massif"])]
    pub site_tools: Vec<site::Tool>,

    /// Benchmark Site CLIent.
    #[arg(long)]
    pub site_client: bool,
    /// The tools to benchmark Site CLIent with.
    #[arg(long, num_args = 1.., default_values = ["hyperfine", "massif"])]
    pub site_client_tools: Vec<site_client::Tool>,

    /// The protocols to benchmark Site and Site CLIent with.
    #[arg(long, num_args = 1.., default_values = ["http", "https", "ws"])]
    pub site_protocols: Vec<SiteProtocol>,

    /// The nums to use.
    #[arg(long, num_args = 1.., default_values_t = [0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000])]
    pub nums: Vec<u64>,

    /// The regex to filter benchmarks by name with.
    #[arg(long)]
    pub filter: Option<String>
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        if !self.cli && !self.site && !self.site_client {
            panic!("You gotta benchmark something.");
        }

        if !self.no_build {
            let mut bins = Vec::new();

            if self.cli {
                bins.push(Bin::Cli);
            }

            if self.site || self.site_client {
                bins.push(Bin::Site);
                bins.push(Bin::SiteClient);
            }

            crate::build::Args {bins}.r#do();
        }

        let benchmarks_string = std::fs::read_to_string(self.benchmarks).unwrap();
        let mut benchmarks = Vec::new();
        let filter = Regex::new(&self.filter.unwrap_or_default()).unwrap();

        for (name, task) in benchmarks_string.lines().map(|line| line.split_once('\t').unwrap()) {
            if filter.find(name).is_some() {
                benchmarks.push((name, task));
            }
        }

        println!("# Benchmarks");
        println!();

        println!("[CLI]: cli");
        println!("[Site]: site");
        println!("[Site CLIent]: site-client");
        println!();

        println!("[Hyperfine]: https://github.com/sharkdp/hyperfine");
        println!("[Valgrind]: https://valgrind.org");
        println!("[Callgrind]: https://valgrind.org/info/tools.html#callgrind");
        println!("[Massif]: https://valgrind.org/info/tools.html#massif");
        println!();

        println!("## System info");
        println!();

        println!("```");
        assert_eq!(Command::new("neofetch")
            .args(["distro", "kernel", "model", "cpu", "memory"])
            .spawn().unwrap().wait().unwrap().code(), Some(0));
        println!("```");
        println!();

        println!("## Benchmarks");
        println!();

        println!("|Name|Task|");
        println!("|:--|:--|");
        for (name, task) in &benchmarks {
            println!("|{name}|`{task}`|");
        }
        println!();

        if self.cli {
            println!("## [CLI]");
            println!();

            for tool in self.cli_tools {
                println!("### [{tool:?}]");
                println!();

                print!("|Name|"); for num in self.nums.iter() {print!("`{}`|", num.to_formatted_string(&Locale::en));} println!();
                print!("|:--|" ); for _   in self.nums.iter() {print!("--:|");                                       } println!();

                for (name, task) in benchmarks.iter().copied() {
                    print!("|{name}|");
                    std::io::stdout().flush().unwrap();

                    for num in self.nums.iter().copied() {
                        print!("`{}`|", get_table_entry(cli::Args {
                            name: name.into(),
                            task: task.into(),
                            num,
                            tool
                        }.r#do()));
                        std::io::stdout().flush().unwrap();
                    }

                    println!();
                }

                println!();
            }
        }

        if self.site {
            println!("## [Site]");
            println!();

            for protocol in self.site_protocols.iter().copied() {
                println!("### {}", protocol.name());
                println!();

                for tool in self.site_tools.iter().copied() {
                    println!("#### [{tool:?}]");
                    println!();

                    print!("|Name|"); for num in self.nums.iter() {print!("`{}`|", num.to_formatted_string(&Locale::en));} println!();
                    print!("|:--|" ); for _   in self.nums.iter() {print!("--:|");                                       } println!();

                    for (name, task) in benchmarks.iter().copied() {
                        print!("|{name}|");
                        std::io::stdout().flush().unwrap();

                        for num in self.nums.iter().copied() {
                            print!("`{}`|", get_table_entry(site::Args {
                                name: name.into(),
                                task: task.into(),
                                num,
                                tool,
                                protocol,
                            }.r#do()));
                            std::io::stdout().flush().unwrap();
                        }

                        println!();
                    }

                    println!();
                }
            }
        }

        if self.site_client {
            println!("## [Site CLIent]");
            println!();

            for protocol in self.site_protocols.iter().copied() {
                println!("### {}", protocol.name());
                println!();

                for tool in self.site_client_tools.iter().copied() {
                    println!("#### [{tool:?}]");
                    println!();

                    print!("|Name|"); for num in self.nums.iter() {print!("`{}`|", num.to_formatted_string(&Locale::en));} println!();
                    print!("|:--|" ); for _   in self.nums.iter() {print!("--:|");                                       } println!();

                    for (name, task) in benchmarks.iter().copied() {
                        print!("|{name}|");
                        std::io::stdout().flush().unwrap();

                        for num in self.nums.iter().copied() {
                            print!("`{}`|", get_table_entry(site_client::Args {
                                name: name.into(),
                                task: task.into(),
                                num,
                                tool,
                                protocol,
                            }.r#do()));
                            std::io::stdout().flush().unwrap();
                        }

                        println!();
                    }

                    println!();
                }
            }
        }
    }
}
