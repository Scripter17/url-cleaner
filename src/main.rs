#![warn(missing_docs)]
#![warn(clippy::expect_used)] // On a separate line so I can comment it out when using Bacon.
#![deny(clippy::unwrap_used, clippy::missing_panics_doc)]

//! URL Cleaner - A tool to remove tracking garbage from URLs.

use std::path::PathBuf;

#[cfg(feature = "stdin")]
use std::io;

use clap::Parser;
use url::Url;

mod rules;
mod glue;
mod suffix;
mod types;

#[derive(Parser)]
struct Args {
    urls: Vec<Url>,
    #[arg(short, long)]
    rules: Option<PathBuf>,
    #[arg(short, long, default_value_t)]
    domain_condition_rule: types::DomainConditionRule
}

fn main() -> Result<(), types::CleaningError> {
    let args=Args::parse();
    let rules=rules::get_rules(args.rules.as_deref())?;
    for mut url in args.urls.into_iter() {
        match rules.apply_with_dcr(&mut url, &args.domain_condition_rule) {
            Ok(_) => {println!("{url}");},
            Err(e) => {println!(); eprintln!("{e:?}");}
        }
    }

    #[cfg(feature = "stdin")]
    {
        if atty::isnt(atty::Stream::Stdin) {
            for line in io::stdin().lines() {
                let mut url=Url::parse(&line?)?;
                match rules.apply_with_dcr(&mut url, &args.domain_condition_rule) {
                    Ok(_) => {println!("{url}");},
                    Err(e) => {println!(); eprintln!("{e:?}")}
                }
            }
        }
    }
    Ok(())
}
