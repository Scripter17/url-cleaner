//! Suite.

use super::prelude::*;
use super::{cli, site, site_client};

/// The bundled tasks.
pub const BUNDLED_TASKS: &str = include_str!("bundled-tasks.tsv");

/// Generate a markdown document of benchmark details.
#[derive(Debug, Parser)]
pub struct Args {
    /// The table filter.
    #[arg(long, default_value = "hyperfine|massif")]
    pub table_filter: Regex,
    /// The task filter.
    #[arg(long, default_value = "")]
    pub task_filter: Regex,
    /// The nums.
    #[arg(long, num_args = 1.., default_values_t = [0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000])]
    pub nums: Vec<u64>,
    /// Don't build anything.
    #[arg(long)]
    pub no_build: bool,
}

/// A table.
#[derive(Debug, Clone, Copy)]
pub enum Table {
    /// [`cli::Args`].
    Cli {
        /// [`cli::Args::tool`].
        tool: ClientTool,
    },
    /// [`site::Args`].
    Site {
        /// [`site::Args::protocol`].
        protocol: Protocol,
        /// [`site::Args::tool`].
        tool: ServerTool,
    },
    /// [`site_client::Args`].
    SiteClient {
        /// [`site_client::Args::protocol`].
        protocol: Protocol,
        /// [`site_client::Args::tool`].
        tool: ClientTool,
    },
}

impl std::fmt::Display for Table {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cli        {tool          } => write!(formatter, "CLI - {tool}"                     ),
            Self::Site       {protocol, tool} => write!(formatter, "Site - {protocol} - {tool}"       ),
            Self::SiteClient {protocol, tool} => write!(formatter, "Site CLIent - {protocol} - {tool}"),
        }
    }
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut tables = Vec::new();

        for tool in ClientTool::value_variants() {
            tables.push(Table::Cli {tool: *tool});
        }

        for protocol in Protocol::value_variants() {
            for tool in ServerTool::value_variants() {
                tables.push(Table::Site {protocol: *protocol, tool: *tool});
            }
        }

        for protocol in Protocol::value_variants() {
            for tool in ClientTool::value_variants() {
                tables.push(Table::SiteClient {protocol: *protocol, tool: *tool});
            }
        }

        tables.retain(|table| self.table_filter.is_match(&table.to_string()));

        let tasks = BUNDLED_TASKS.lines().map(|x| x.split_once('\t').unwrap()).filter(|(name, _)| self.task_filter.is_match(name)).collect::<Vec<_>>();

        if !self.no_build && !tables.is_empty() {
            let mut bins = Vec::new();

            if tables.iter().any(|table| matches!(table, Table::Cli {..})) {
                bins.push(Bin::Cli);
            }

            if tables.iter().any(|table| matches!(table, Table::Site {..} | Table::SiteClient {..})) {
                bins.extend([Bin::Site, Bin::SiteClient]);
            }

            crate::build::Args {bins}.r#do();
        }

        println!("# Benchmarks");
        println!();

        println!("## Runner");
        println!();

        println!("```");
        assert_eq!(Command::new("neofetch")
            .args(["distro", "kernel", "model", "cpu", "memory"])
            .spawn().unwrap().wait().unwrap().code(), Some(0));
        println!("```");
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

        let mut lines_time = std::time::Duration::default();

        for table in tables {
            println!("### {table}");
            println!();

            print!("|Task|");
            for num in &self.nums {
                print!("`{}`|", num.to_formatted_string(&Locale::en));
            }
            println!();
            print!("|:--|");
            for _ in &self.nums {
                print!("--:|");
            }
            println!();

            for (name, task) in tasks.iter().copied() {
                let line_start = std::time::Instant::now();
                
                print!("|{name}|");
                std::io::stdout().flush().unwrap();

                for num in self.nums.iter().copied() {
                    print!("`{}`|", match table {
                        Table::Cli        {tool          } => tool.get_entry(cli        ::Args {name: name.into(), task: task.into(), num, tool          }.r#do()),
                        Table::Site       {tool, protocol} => tool.get_entry(site       ::Args {name: name.into(), task: task.into(), num, tool, protocol}.r#do()),
                        Table::SiteClient {tool, protocol} => tool.get_entry(site_client::Args {name: name.into(), task: task.into(), num, tool, protocol}.r#do()),
                    });
                    std::io::stdout().flush().unwrap();
                }

                let line_time = line_start.elapsed();
                lines_time += line_time;

                println!(" <!-- + {line_time:?} -> {lines_time:?} -->");
            }

            println!();
        }
    }
}
