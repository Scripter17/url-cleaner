//! Convert [`std::time::Duration`]s to seconds.

use clap::Parser;

use crate::util::*;

/// Convert Durations to seconds.
#[derive(Debug, Parser)]
pub struct Args {
    
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        for (i, line) in (1..).zip(std::io::stdin().lines().map(Result::unwrap)) {
            print!("{i}");

            for column in line.split('\t') {
                print!("\t{}", parse_duration(column).unwrap().as_secs_f64());
            }

            println!();
        }
    }
}
