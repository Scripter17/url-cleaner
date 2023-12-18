use std::path::PathBuf;

#[cfg(feature = "stdin")]
use std::io;
#[cfg(feature = "stdin")]
use atty;

use clap::Parser;
use url::Url;

mod rules;
mod glue;
mod types;

#[derive(Parser)]
struct Args {
    url: Option<String>,
    #[arg(short, long)]
    rules: Option<PathBuf>
}

fn main() {
    let args=Args::parse();
    let rules=rules::get_rules(args.rules.as_deref()).unwrap();
    if let Some(url) = args.url {
        let mut url=Url::parse(&url).unwrap();
        match rules.apply(&mut url) {
            Ok(_) => {println!("{url}");},
            Err(e) => {println!("{e:?}");}
        }
    }

    #[cfg(feature = "stdin")]
    {
        if atty::isnt(atty::Stream::Stdin) {
            for line in io::stdin().lines() {
                let mut url=Url::parse(&line.unwrap()).unwrap();
                match rules.apply(&mut url) {
                    Ok(_) => {println!("{url}");},
                    Err(_) => {println!();}
                }
            }
        }
    }
}
