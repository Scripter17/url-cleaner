//! URL Cleaner originally started as a project to remove tracking garbage from URLs but has since grown into a very powerful URL manipulation tool.

use std::path::PathBuf;

#[cfg(feature = "stdin")]
use std::io;

use clap::Parser;
use url::Url;

mod rules;
mod glue;
mod types;
mod util;

#[derive(Parser)]
struct Args {
    urls: Vec<Url>,
    #[arg(short, long)]
    config: Option<PathBuf>,
    #[arg(short, long)]
    var: Vec<String>,
    #[arg(short, long)]
    flag: Vec<String>,
    #[arg(long)]
    no_read_cache: bool,
    #[arg(long)]
    no_write_cache: bool,
    #[arg(long)]
    print_config: bool
}

impl TryFrom<Args> for (Vec<Url>, types::Config) {
    type Error=types::GetConfigError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let mut config=types::Config::get_default_or_load(args.config.as_deref())?.into_owned();
        config.params.merge(
            #[allow(clippy::needless_update)]
            types::Params {
                vars: args.var
                    .into_iter()
                    .filter_map(|mut kev| kev.find('=').map(|e| {let mut v=kev.split_off(e); v.drain(..1); kev.shrink_to_fit(); (kev, v)}))
                    .collect(),
                flags: args.flag.into_iter().collect(),
                no_read_cache: args.no_read_cache,
                no_write_cache: args.no_write_cache,
                ..types::Params::default()
            }
        );
        Ok((args.urls, config))
    }
}

fn main() -> Result<(), types::CleaningError> {
    let args = Args::parse();

    if args.print_config {
        let (_, config): (Vec<Url>, types::Config)=args.try_into()?;
        println!("{}", serde_json::to_string(&config)?);
        std::process::exit(0);
    }
    
    let (urls, config): (Vec<Url>, types::Config)=args.try_into()?;

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
