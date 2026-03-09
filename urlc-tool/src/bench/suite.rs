//! Suite.

use super::prelude::*;
use super::{cli, site, site_client};

/// The bundled [`Suite`].
pub const BUNDLED_SUITE: &str = include_str!("bundled-suite.json");

/// Generate a markdown document of benchmark details.
#[derive(Debug, Parser)]
pub struct Args {
    /// The suite.
    #[arg(long)]
    pub suite: Option<PathBuf>,
    /// The regex to filter tables.
    #[arg(long)]
    pub table_filter: Option<String>,
    /// The regex to filter tasks.
    #[arg(long)]
    pub task_filter: Option<String>,
    /// The nums.
    #[arg(long, num_args = 1..)]
    pub nums: Option<Vec<u64>>,
    /// Don't build anything.
    #[arg(long)]
    pub no_build: bool,
    /// Just print dummy entries.
    #[arg(long)]
    pub dry: bool,
}

/// A suite.
#[derive(Debug, Clone, Deserialize)]
pub struct Suite {
    /// The [`Matrix`]s.
    pub matrices: Vec<Matrix>,
    /// The regex to filter table.s
    #[serde(default)]
    pub table_filter: String,
    /// The tasks.
    pub tasks: IndexMap<String, String>,
    /// The regex to filter tasks.
    #[serde(default)]
    pub task_filter: String,
    /// The nums.
    pub nums: Vec<u64>,
}

/// A matrix of [`Table`]s.
#[derive(Debug, Clone, Deserialize)]
pub enum Matrix {
    /// [`Table::Cli`].
    Cli {
        /// The [`Table::Cli::tool`]s.
        tools: Vec<cli::Tool>,
    },
    /// [`Table::Site`].
    Site {
        /// The [`Table::Site::tool`]s.
        tools: Vec<site::Tool>,
        /// The [`Table::Site::client`]s.
        clients: Vec<site::Client>,
        /// The [`Table::Site::tls`]s.
        tlss: Vec<bool>,
        /// The [`Table::Site::parallel`]s.
        parallels: Vec<u64>,
    },
    /// [`Table::SiteClient`].
    SiteClient {
        /// The [`Table::SiteClient::tool`]s.
        tools: Vec<site_client::Tool>,
        /// The [`Table::Site::client`]s.
        clients: Vec<site_client::Client>,
        /// The [`Table::Site::tls`]s.
        tlss: Vec<bool>,
    },
}

impl Matrix {
    /// Get the [`Table`]s.
    pub fn get_tables(self) -> Vec<Table> {
        let mut ret = Vec::new();

        match self {
            Self::Cli {tools} => {
                for tool in tools {
                    ret.push(Table::Cli {tool});
                }
            },
            Self::Site {tools, clients, tlss, parallels} => {
                for tool in tools {
                    for client in clients.iter().copied() {
                        for tls in tlss.iter().copied() {
                            for parallel in parallels.iter().copied() {
                                ret.push(Table::Site {client, tls, tool, parallel});
                            }
                        }
                    }
                }
            },
            Self::SiteClient {tools, clients, tlss} => {
                for tool in tools {
                    for client in clients.iter().copied() {
                        for tls in tlss.iter().copied() {
                            ret.push(Table::SiteClient {client, tls, tool});
                        }
                    }
                }
            },
        }

        ret
    }
}

/// A table.
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Table {
    /// [`cli`].
    Cli {
        /// [`cli::Args::tool`].
        tool: cli::Tool,
    },
    /// [`site`].
    Site {
        /// [`site::Args::tool`].
        tool: site::Tool,
        /// [`site::Args::client`].
        client: site::Client,
        /// [`site::Args::tls`].
        tls: bool,
        /// [`site::Args::parallel`].
        parallel: u64,
    },
    /// [`site_client`].
    SiteClient {
        /// [`site_client::Args::tool`].
        tool: site_client::Tool,
        /// [`site_client::Args::client`].
        client: site_client::Client,
        /// [`site_client::Args::tls`].
        tls: bool,
    },
}

impl Table {
    /// The name of the table.
    fn name(self) -> String {
        match self {
            Self::Cli        {tool                       } => format!("[CLI] - [{tool:?}]"                                   ),
            Self::Site       {tool, client, tls, parallel} => format!("[Site] - [{tool:?}] - {client:?} - {tls} - {parallel}"),
            Self::SiteClient {tool, client, tls          } => format!("[Site CLIent] - [{tool:?}] - {client:?} - {tls}"      ),
        }
    }
}

/// Get an entry.
pub fn get_entry(path: &str) -> String {
    match path.rsplit_once('/').unwrap().1.split_once('.').unwrap().0 {
        "hyperfine" => format!("{:.1}", serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(path).unwrap()).unwrap()["results"][0]["mean"].as_f64().unwrap() * 1000.0),
        "massif" => {
            let mut ret = 0u64;
            for line in BufReader::new(File::open(path).unwrap()).lines() {
                if let Some(x) = line.unwrap().strip_prefix("mem_heap_B=") {
                    ret = ret.max(x.parse().unwrap());
                }
            }
            ret.to_formatted_string(&Locale::en)
        },
        "callgrind" => "...".into(),
        _ => panic!("???")
    }
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let suite: Suite = match self.suite {
            Some(suite) => serde_json::from_str(&std::fs::read_to_string(suite).unwrap()).unwrap(),
            None        => serde_json::from_str(BUNDLED_SUITE).unwrap(),
        };

        let table_filter = Regex::new(&self.table_filter.unwrap_or(suite.table_filter)).unwrap();
        let task_filter  = Regex::new(&self.task_filter .unwrap_or(suite.task_filter )).unwrap();

        let tables = suite.matrices.into_iter().flat_map(Matrix::get_tables).filter(|table| table_filter.is_match(&table.name())).collect::<Vec<_>>();
        let tasks  = suite.tasks.into_iter().filter(|(name, _)| task_filter.is_match(name)).collect::<Vec<_>>();

        let nums = self.nums.unwrap_or(suite.nums);

        if !self.no_build {
            let mut bins = Vec::new();

            for table in &tables {
                bins.extend_from_slice(match table {
                    Table::Cli        {..} => &[Bin::Cli],
                    Table::Site       {..} => &[Bin::Site, Bin::SiteClient],
                    Table::SiteClient {..} => &[Bin::Site, Bin::SiteClient],
                });
            }

            crate::build::Args {bins}.r#do();
        }

        let _ = std::fs::remove_dir_all("urlc-tool/out/bench");

        let tar_out = format!("urlc-tool/out/bench-{}.tar.gz", std::time::SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs_f64());

        let mut tar = Command::new("tar");

        tar.args(["-C", "urlc-tool/out", "-cvzf", &tar_out, "bench"]);

        tar.stdout(std::process::Stdio::null());

        println!("# Benchmarks");
        println!();

        println!("[CLI]: cli");
        println!("[Site]: site");
        println!("[Site CLIent]: site-client");
        println!("[Websocat]: https://github.com/vi/websocat");
        println!();

        println!("[Hyperfine]: https://github.com/sharkdp/hyperfine");
        println!("[Massif]: https://valgrind.org/info/tools.html#massif");
        println!("[Callgrind]: https://valgrind.org/info/tools.html#callgrind");
        println!();

        println!("## Names");
        println!();
        println!("Each table has a name comprised of the target and some parameters separated by -.");
        println!();
        println!("- [Cli]");
        println!("  - The tool");
        println!("- [Site]");
        println!("  - The tool");
        println!("  - The client");
        println!("  - Whether TLS is enabled");
        println!("  - The number of parallel clients");
        println!("- [Site CLIent]");
        println!("  - The tool");
        println!("  - The client");
        println!("  - Whether TLS is enabled");
        println!();

        println!("## Tools");
        println!();
        println!("- [Hyperfine]: The average runtime in milliseconds.");
        println!("- [Massif]: Peak memory usage in bytes.");
        println!();

        println!("## Site Clients");
        println!();
        println!("- SiteClientHttp: [Site CLIent] using HTTP/HTTPS.");
        println!("- SiteClientWs: [Site CLIent] using WebSocket/Secure WebSocket.");
        println!("- Curl: Curl.");
        println!("- Websocat: [Websocat].");
        println!();

        println!("## Tasks");
        println!();

        println!("|Nmae|Task|");
        println!("|:--|:--|");
        for (name, task) in &tasks {
            println!("|{name}|`{task}`|");
        }
        println!();

        println!("## Tables");
        println!();

        let suite_start = std::time::Instant::now();
        let table_count = tables.len();

        for (table, i) in tables.into_iter().zip(1..) {
            println!("### {}", table.name());
            println!();
            println!("<!-- {i}/{table_count} -->");
            println!();

            print!("|Task|");
            for num in &nums {
                print!("`{}`|", num.to_formatted_string(&Locale::en));
            }
            println!();
            print!("|:--|");
            for _ in &nums {
                print!("--:|");
            }
            println!();

            let table_start = std::time::Instant::now();

            for (name, task) in &tasks {
                print!("|{name}|");
                std::io::stdout().flush().unwrap();

                let row_start = std::time::Instant::now();

                for num in nums.iter().copied() {
                    if self.dry {
                        print!(".|");
                    } else {
                        print!("`{}`|", get_entry(&match table {
                            Table::Cli        {tool                       } => cli        ::Args {name: name.into(), task: task.into(), num, tool                       }.r#do(),
                            Table::Site       {tool, client, tls, parallel} => site       ::Args {name: name.into(), task: task.into(), num, tool, client, tls, parallel}.r#do(),
                            Table::SiteClient {tool, client, tls          } => site_client::Args {name: name.into(), task: task.into(), num, tool, client, tls          }.r#do(),
                        }));
                    }
                    std::io::stdout().flush().unwrap();
                }

                println!(" <!-- {:.1}s -->", row_start.elapsed().as_secs_f64());
            }

            println!();
            println!("<!-- {:.1}s -->", table_start.elapsed().as_secs_f64());
            println!("<!-- {:.1}s -->", suite_start.elapsed().as_secs_f64());
            println!();
        }

        eprintln!("Writing {tar_out}");

        assert_eq!(tar.spawn().unwrap().wait().unwrap().code(), Some(0));
    }
}
