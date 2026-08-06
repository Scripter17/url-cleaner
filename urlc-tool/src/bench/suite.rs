//! Suite.

use super::prelude::*;
use super::{cli, site, site_client};

/// The bundled tasks.
pub const BUNDLED_TASKS: &str = include_str!("bundled-tasks.tsv");

/// Generate a markdown document of benchmark details.
#[derive(Debug, Parser)]
pub struct Args {
    /// The table filter.
    #[arg(long, default_value = "Hyperfine|Massif")]
    pub table_filter: Regex,
    /// The task filter.
    #[arg(long, default_value = "")]
    pub task_filter: Regex,
    /// The nums.
    #[arg(long, num_args = 1.., default_values_t = [0, 1, 10, 100, 1_000, 10_000, 100_000])]
    pub nums: Vec<u64>,
    /// Don't build anything.
    #[arg(long)]
    pub no_build: bool,
    /// Don't run anything.
    #[arg(long)]
    pub no_run: bool,
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
            Self::Cli        {          tool} => write!(formatter, "CLI - {}"             ,                   tool.title()),
            Self::Site       {protocol, tool} => write!(formatter, "Site - {} - {}"       , protocol.upper(), tool.title()),
            Self::SiteClient {protocol, tool} => write!(formatter, "Site CLIent - {} - {}", protocol.upper(), tool.title()),
        }
    }
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut tables = Vec::new();

        for &tool in ClientTool::value_variants() {
            tables.push(Table::Cli {tool});
        }

        for &protocol in Protocol::value_variants() {
            for &tool in ServerTool::value_variants() {
                tables.push(Table::Site {protocol, tool});
            }
        }

        for &protocol in Protocol::value_variants() {
            for &tool in ClientTool::value_variants() {
                tables.push(Table::SiteClient {protocol, tool});
            }
        }

        tables.retain(|table| self.table_filter.is_match(&table.to_string()));

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

        let mut tasks = Vec::new();

        let mut name_width = 0;

        for line in BUNDLED_TASKS.lines() {
            let mut columns = line.split('\t');

            let name        = columns.next().unwrap();
            let task        = columns.next().unwrap();
            let params_diff = columns.next();

            if self.task_filter.is_match(name) {
                tasks.push((name, task, params_diff));

                name_width = name_width.max(name.len());
            }
        }

        println!("# Benchmarks");
        println!();

        println!("## As seen on");
        println!();

        println!("```");
        assert_eq!(
            Command::new("neofetch")
                .args(["distro", "kernel", "model", "cpu", "memory"])
                .spawn().unwrap().wait().unwrap().code(),
            Some(0)
        );
        println!("```");
        println!();

        println!("## Tasks");
        println!();

        println!("|Name|Task|ParamsDiff|");
        println!("|:--|:--|:--|");
        for (name, task, params_diff) in &tasks {
            print!("|{name:<name_width$}|`{task}`|");
            match params_diff {
                Some(params_diff) => println!("`{params_diff}`|"),
                None              => println!("None|"),
            }
        }
        println!();

        println!("## Tables");
        println!();

        let mut lines_time = std::time::Duration::default();

        for table in tables {
            println!("### {table}");
            println!();

            print!("|Task|");
            for &num in &self.nums {
                print!("`{}`|", format_int(num));
            }
            println!();
            print!("|:--|");
            for _ in &self.nums {
                print!("--:|");
            }
            println!();

            for (name, task, params_diff) in tasks.iter().copied() {
                let line_start = std::time::Instant::now();

                print!("|{name:<name_width$}|");
                std::io::stdout().flush().unwrap();

                for num in self.nums.iter().copied() {
                    if self.no_run {
                        print!("`...`|");
                    } else {
                        print!("`{}`|", match table {
                            Table::Cli        {          tool} => tool.get_entry(cli        ::Args {name: name.into(), task: task.into(), num, params_diff: params_diff.map(Into::into),           tool}.r#do()),
                            Table::Site       {protocol, tool} => tool.get_entry(site       ::Args {name: name.into(), task: task.into(), num, params_diff: params_diff.map(Into::into), protocol, tool}.r#do()),
                            Table::SiteClient {protocol, tool} => tool.get_entry(site_client::Args {name: name.into(), task: task.into(), num, params_diff: params_diff.map(Into::into), protocol, tool}.r#do()),
                        });
                    }
                    std::io::stdout().flush().unwrap();
                }

                let line_time = line_start.elapsed();
                lines_time += line_time;

                println!(" <!-- + {line_time:.2?} -> {lines_time:.2?} -->");
            }

            println!();
        }
    }
}
