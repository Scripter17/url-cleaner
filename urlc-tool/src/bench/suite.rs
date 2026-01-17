//! Suite.

pub use super::prelude::*;

/// Generate a markdown document of benchmark details.
#[derive(Debug, Parser)]
pub struct Args {
    /// The targets to benchmark.
    #[arg(long, num_args = 1.., default_values = ["cli", "http", "https", "ws", "wss"])]
    pub targets: Vec<Target>,
    /// The tools to benchmark with.
    #[arg(long, num_args = 1.., default_values = ["hyperfine", "massif"])]
    pub tools: Vec<Tool>,
    /// The nums to use.
    #[arg(long, num_args = 1.., default_values_t = [0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000])]
    pub nums: Vec<u64>,

    /// The regex to filter task names with.
    #[arg(long)]
    pub filter: Option<String>
}

/// The tasks to benchmark with.
pub const TASKS: &[(&str, &str)] = &[
    ("Baseline", "https://example.com"),
    ("UTPs"    , "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id"),
    ("Amazon"  , "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8"),
    ("Google"  , "https://www.google.com/search?q=url+cleaner&sca_esv=de6549fe37924183&ei=eRAYabb6O7Gb4-EP79Xe6A8&ved=0ahUKEwj2mqWLt_OQAxWxzTgGHe-qF_0Q4dUDCBE&oq=url+cleaner&gs_lp=Egxnd3Mtd2l6LXNlcnAiC3VybCBjbGVhbmVySABQAFgAcAB4AZABAJgBAKABAKoBALgBDMgBAJgCAKACAJgDAJIHAKAHALIHALgHAMIHAMgHAA&sclient=gws-wiz-serp"),
];

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut bins = Vec::new();

        if self.targets.iter().any(|target| matches!(target, Target::Cli    )) {bins.push(Bin::Cli );                            }
        if self.targets.iter().any(|target| matches!(target, Target::Site(_))) {bins.push(Bin::Site); bins.push(Bin::SiteClient);}

        crate::build::Args {
            bins,
            no_compile: false,
            debug: false
        }.r#do();

        let filter = Regex::new(&self.filter.unwrap_or_default()).unwrap();

        let tasks = TASKS.iter().filter(|(name, _)| filter.find(name).is_some()).collect::<Vec<_>>();

        println!("# Benchmarks");
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

        println!("## Tasks");
        println!();

        println!("|Name|Task|");
        println!("|:--|:--|");
        for (name, task) in &tasks {
            println!("|{name}|`{task}`|");
        }
        println!();

        for target in self.targets {
            match target {
                Target::Cli => println!("## CLI"),
                Target::Site(api) => {
                    println!("## Site {}", api.name());
                    println!();
                    println!("Measured using [URL Cleaner Site Client](./site-client/)");
                }
            }
            println!();

            for tool in self.tools.iter().copied() {
                match tool {
                    Tool::Hyperfine => {println!("### Speed"    ); println!(); println!("Average milliseconds. Measured by [Hyperfine]."       )},
                    Tool::Massif    => {println!("### Memory"   ); println!(); println!("Peak memory usage. Measured by [Valgrind]'s [Massif].")},
                    Tool::Callgrind => {println!("### Callgrind"); println!(); println!("Callgrind info. Measured by [Valgrind]'s [Callgrind].")}
                }
                println!();

                print!("|Name|");
                for num in self.nums.iter() {
                    print!("{}|", num.to_formatted_string(&Locale::en));
                }
                println!();

                print!("|:--|");
                for _ in 0..self.nums.len() {
                    print!("--:|");
                }
                println!();

                for (name, task) in &tasks {
                    print!("|{name}|");
                    std::io::stdout().flush().unwrap();

                    for num in self.nums.iter().copied() {
                        print!("{}|", tool.get_entry(&match target {
                            Target::Cli => super::cli::Args {
                                name: name.to_string(),
                                job_config: JobConfig { task: task.to_string(), num },
                                tool
                            }.r#do(),
                            Target::Site(api) => super::site::Args {
                                name: name.to_string(),
                                job_config: JobConfig { task: task.to_string(), num },
                                tool,
                                api
                            }.r#do()
                        }));

                        std::io::stdout().flush().unwrap();
                    }

                    println!();
                }

                println!();
            }
        }
    }
}
