use std::path::PathBuf;
use std::fs::read_to_string;

#[cfg(feature = "stdin")]
use std::io;
#[cfg(feature = "stdin")]
use atty;

use clap::Parser;
use serde_json;

mod rules;
mod cleaner;

#[derive(Parser)]
struct Args {
    url: Option<String>,
    #[arg(short, long)]
    rules: Option<PathBuf>
}

fn main() {
    let args=Args::parse();
    let rules=args.rules.map(|path| serde_json::from_str::<Vec<rules::Rule>>(&read_to_string(path).unwrap()).unwrap());
    if let Some(url) = args.url {
        match cleaner::clean_url_str(&url, rules.as_deref()) {
            Ok(url) => {println!("{url}");},
            Err(_) => {println!();}
        }
    }

    #[cfg(feature = "stdin")]
    {
        if atty::isnt(atty::Stream::Stdin) {
            for line in io::stdin().lines() {
                match cleaner::clean_url_str(&line.unwrap(), rules.as_deref()) {
                    Ok(url) => {println!("{url}");},
                    Err(_) => {println!();}
                }
            }
        }
    }
}
