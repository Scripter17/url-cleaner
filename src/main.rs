//! URL Cleaner - A tool to remove tracking garbage from URLs.

use std::path::PathBuf;

#[cfg(feature = "stdin")]
use std::io;

use clap::Parser;
use url::Url;

mod rules;
mod glue;
mod types;

#[derive(Parser)]
struct Args {
    urls: Vec<Url>,
    #[arg(short, long)]
    rules: Option<PathBuf>,
    #[arg(short, long, default_value_t)]
    domain_condition_rule: types::DomainConditionRule,
    #[arg(short, long)]
    variables: Vec<String>,
    #[arg(short, long)]
    flags: Vec<String>
}

fn main() -> Result<(), types::CleaningError> {
    let args=Args::parse();
    let rules=rules::get_rules(args.rules.as_deref())?;
    let config=types::RuleConfig {
        dcr: args.domain_condition_rule,
        variables: args.variables.iter().filter_map(|kev| kev.split_once('=')).map(|(k, v)| (k.to_owned(), v.to_owned())).collect(),
        flags: args.flags.into_iter().collect()
    };
    for mut url in args.urls {
        match rules.apply_with_config(&mut url, &config) {
            Ok(()) => {println!("{url}");},
            Err(e) => {println!(); eprintln!("ERROR: {e:?}");}
        }
    }

    #[cfg(feature = "stdin")]
    {
        if atty::isnt(atty::Stream::Stdin) {
            for maybe_line in io::stdin().lines() {
                match maybe_line {
                    Ok(line) => match Url::parse(&line) {
                        Ok(mut url) => match rules.apply_with_config(&mut url, &config) {
                            Ok(()) => {println!("{url}");},
                            Err(e) => {println!(); eprintln!("ERROR: {e:?}");}
                        },
                        Err(e) => {println!(); eprintln!("ERROR: {e:?}");}
                    },
                    Err(e) => {println!(); eprintln!("ERROR: {e:?}");}
                }
            }
        }
    }

    Ok(())
}
