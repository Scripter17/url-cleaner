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

fn main() {
    suffix::init_tlds();
    
    let args=Args::parse();
    let rules=rules::get_rules(args.rules.as_deref()).unwrap();
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
                let mut url=Url::parse(&line.unwrap()).unwrap();
                match rules.apply_with_dcr(&mut url, &args.domain_condition_rule) {
                    Ok(_) => {println!("{url}");},
                    Err(e) => {println!(); eprintln!("{e:?}")}
                }
            }
        }
    }
}
