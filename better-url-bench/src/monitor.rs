//! Get stats about benchmarks.

use std::time::Duration;

use clap::Parser;

use crate::util::*;

/// Get stats about benchmarks.
#[derive(Debug, Parser, Default)]
pub struct Args {
    /// The title of the benchmark.
    #[arg(long)]
    pub title: String,
    /// The things being benchmarked.
    #[arg(long, num_args = 0..)]
    pub columns: Vec<String>,
    /// How many benchmarks to wait before showing the current stats.
    #[arg(long, default_value = "100")]
    pub prog_every: u32,
    /// Print the table as markdown.
    #[arg(long)]
    pub markdown: bool,
    /// Don't print the factor row.
    #[arg(long)]
    pub no_factor: bool,
}

/// The state.
#[derive(Debug)]
struct State {
    /** The title.                                              */ title     : String,
    /** The names of the columns.                               */ columns   : Vec<String>,
    /** The number of lines after which to print progress info. */ prog_every: u32,
    /** The number of benchmarks.                               */ count     : u32,
    /** The current sum of each thing's times.                  */ sums      : Vec<Duration>,
    /** The current min of each thing's times.                  */ mins      : Vec<Duration>,
    /** The current max of each thing's times.                  */ maxs      : Vec<Duration>,
    /** The number of times temporary output has been printed.  */ temps     : u64,
    /** The number of lines to replace on next print.           */ out_lines : u64,
    /** The time at which this process started.                 */ start     : std::time::Instant,
    /** If the table should be printed as markdown.             */ markdown  : bool,
    /** If the factor row should be printed.                    */ factor    : bool,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut state = State {
            title     : self.title,
            columns   : self.columns,
            prog_every: self.prog_every,
            count     : 0,
            sums      : Vec::new(),
            mins      : Vec::new(),
            maxs      : Vec::new(),
            temps     : 0,
            out_lines : 0,
            start     : std::time::Instant::now(),
            markdown  : self.markdown,
            factor    : !self.no_factor,
        };

        for line in std::io::stdin().lines().map(Result::unwrap) {
            state.process_line(&line);
        }

        state.print_table(false);
    }
}

impl State {
    /// Process a line.
    fn process_line(&mut self, line: &str) {
        self.count += 1;

        for (i, time) in line.split('\t').map(|s| parse_duration(s).expect(s)).enumerate() {
            if i >= self.sums.len() {
                self.sums.push(Default::default());
                self.mins.push(time);
                self.maxs.push(time);
            }

            self.sums[i] += time;
            self.mins[i] = self.mins[i].min(time);
            self.maxs[i] = self.maxs[i].max(time);
        }

        if self.count.is_multiple_of(self.prog_every) {
            self.print_table(true);
        }
    }

    /// Print the data as a table.
    fn print_table(&mut self, temp: bool) {
        if self.markdown {
            self.print_markdown_table(temp)
        } else {
            self.print_raw_table(temp)
        }
    }

    /// Print the table as a TSV.
    fn print_raw_table(&mut self, temp: bool) {
        for _ in 0..self.out_lines {
            print!("\x1b[1A\x1b[K");
        }

        self.out_lines = 0;

        print!("{}", self.title);

        for column in &self.columns {
            print!("\t{column}");
        }

        if temp {
            self.temps += 1;
            print!("\t{}", self.temps);
            print!("\t{:.2?}", self.start.elapsed());
        }

        println!();
        self.out_lines += 1;

        print!("Sum" ); for &sum in &self.sums { print!("\t{:.2?}", sum             ); } println!(); self.out_lines += 1;
        print!("Min" ); for &min in &self.mins { print!("\t{:.2?}", min             ); } println!(); self.out_lines += 1;
        print!("Max" ); for &max in &self.maxs { print!("\t{:.2?}", max             ); } println!(); self.out_lines += 1;
        print!("Mean"); for &sum in &self.sums { print!("\t{:.2?}", sum / self.count); } println!(); self.out_lines += 1;

        if self.factor && let Some(base) = self.sums.first() {
            print!("Factor");

            for &sum in &self.sums {
                print!("\t{:.3?}", sum.as_secs_f64() / base.as_secs_f64());
            }

            println!();

            self.out_lines += 1;
        }
    }

    /// Print the table as markdown.
    fn print_markdown_table(&mut self, temp: bool) {
        for _ in 0..self.out_lines {
            print!("\x1b[1A\x1b[K");
        }

        self.out_lines = 0;

        let c1_pad = self.title.len().max(8);
        let pads = self.columns.iter().map(|x| x.len().max(8)).collect::<Vec<_>>();

        print!("|{:<c1_pad$}|", self.title);

        for (column, pad) in self.columns.iter().zip(&pads) {
            print!("{:<pad$}|", column);
        }

        if temp {
            self.temps += 1;
            print!("\t{}", self.temps);
            print!("\t{:.2?}", self.start.elapsed());
        }

        println!();
        self.out_lines += 1;



        let align_pad_1 = c1_pad - 1;
        print!("|:{:-<align_pad_1$}|", "");

        for &pad in &pads {
            let align_pad = pad - 1;
            print!("{:-<align_pad$}:|", "");
        }

        println!();
        self.out_lines += 1;


        print!("|{:<c1_pad$}|", "Sum" ); for (&sum, &pad) in self.sums.iter().zip(&pads) { print!("{:>pad$.2?}|", sum             ); } println!(); self.out_lines += 1;
        print!("|{:<c1_pad$}|", "Min" ); for (&min, &pad) in self.mins.iter().zip(&pads) { print!("{:>pad$.2?}|", min             ); } println!(); self.out_lines += 1;
        print!("|{:<c1_pad$}|", "Max" ); for (&max, &pad) in self.maxs.iter().zip(&pads) { print!("{:>pad$.2?}|", max             ); } println!(); self.out_lines += 1;
        print!("|{:<c1_pad$}|", "Mean"); for (&sum, &pad) in self.sums.iter().zip(&pads) { print!("{:>pad$.2?}|", sum / self.count); } println!(); self.out_lines += 1;

        if self.factor && let Some(base) = self.sums.first() {
            print!("|{:<c1_pad$}|", "Factor");

            for (&sum, &pad) in self.sums.iter().zip(&pads) {
                print!("{:>pad$.3?}|", sum.as_secs_f64() / base.as_secs_f64());
            }

            println!();

            self.out_lines += 1;
        }
    }
}
