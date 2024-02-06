//! URL Cleaner - A tool to remove tracking garbage from URLs.

use std::path::PathBuf;

#[cfg(feature = "stdin")]
use std::io;

use clap::Parser;
use url::Url;

mod rules;
mod glue;
mod types;
mod config;

#[derive(Parser)]
struct Args {
    urls: Vec<Url>,
    #[arg(short, long)]
    config: Option<PathBuf>,
    #[arg(short, long)]
    dcr: Option<types::DomainConditionRule>,
    #[arg(short, long)]
    var: Vec<String>,
    #[arg(short, long)]
    flag: Vec<String>
}

impl TryFrom<Args> for (Vec<Url>, config::Config) {
    type Error=config::GetConfigError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let mut config=config::Config::get_default_or_load(args.config.as_deref())?.into_owned();

        if let Some(dcr) = args.dcr {config.params.dcr=dcr;}
        // Tuple maps when.
        if !args.var.is_empty() {
            config.params.variables=args.var
                .into_iter()
                .filter_map(|mut kev| kev.find('=').map(|e| {let v=kev.split_off(e); kev.shrink_to_fit(); (kev, v)}))
                .collect();
        }
        if !args.flag.is_empty() {config.params.flags=args.flag.into_iter().collect();}

        Ok((args.urls, config))
    }
}

fn main() -> Result<(), types::CleaningError> {
    let (urls, config): (Vec<Url>, config::Config)=Args::parse().try_into()?;

    for mut url in urls {
        match config.apply(&mut url) {
            Ok(()) => {println!("{url}");},
            Err(e) => {println!(); eprintln!("Rule error: {e:?}");}
        }
    }

    #[cfg(feature = "stdin")]
    if atty::isnt(atty::Stream::Stdin) {
        for maybe_line in io::stdin().lines() {
            match maybe_line {
                Ok(line) => match Url::parse(&line) {
                    Ok(mut url) => match config.apply(&mut url) {
                        Ok(()) => {println!("{url}");},
                        Err(e) => {println!(); eprintln!("Rule error: {e:?}");}
                    },
                    Err(e) => {println!(); eprintln!("Line parse: {e:?}");}
                },
                Err(e) => {println!(); eprintln!("Line read error: {e:?}");}
            }
        }
    }

    Ok(())
}
