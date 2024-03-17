//! URL Cleaner originally started as a project to remove tracking garbage from URLs but has since grown into a very powerful URL manipulation tool.

use std::path::PathBuf;

#[cfg(feature = "stdin")]
use std::io;

use clap::Parser;
use url::Url;

mod glue;
mod types;
mod util;

#[derive(Debug, Clone, Parser, Default)]
#[command(name = "URL Cleaner")]
/// URL Cleaner is a tool meant primarily to remove tracking garbage from URLs and more generally is a very powerful URL manipulation library/CLI tool.
struct Args {
    /// The URLs to cleam before the URLs in the STDIN.
    urls: Vec<Url>,
    /// The config.json to use. If unspecified, use the config compiled into URL Cleaner
    #[arg(short      , long)] config: Option<PathBuf>,
    /// Set variables using name=value syntax.
    #[arg(short      , long)] var   : Vec<String>,
    /// Unset variables set by the config.
    #[arg(short = 'V', long)] unvar : Vec<String>,
    /// Set flags.
    #[arg(short      , long)] flag  : Vec<String>,
    /// Unset flags set by the config.
    #[arg(short = 'F', long)] unflag: Vec<String>,
    /// Read stuff from caches. Currently only applies to Mapper::ExpandShortLink. Default value is set by the config (which in turn defaults to true).
    #[cfg(feature = "cache")] #[arg(long)] read_cache : Option<bool>,
    /// Write stuff to caches. Currently only applies to Mapper::ExpandShortLink. Default value is set by the config (which in turn defaults to true).
    #[cfg(feature = "cache")] #[arg(long)] write_cache: Option<bool>,
    /// Print the config's contents then exit.
    #[arg(long)] print_config: bool,
    /// Run the config's tests then exit.
    #[arg(long)] test_config : bool
}

impl From<Args> for (Vec<Url>, types::ParamsDiff) {
    fn from(args: Args) -> Self {
        (
            args.urls,
            types::ParamsDiff {
                vars   : args.var   .into_iter().filter_map(|mut kev| kev.find('=').map(|e| {let mut v=kev.split_off(e); v.drain(..1); kev.shrink_to_fit(); (kev, v)})).collect(),
                unvars : args.unvar .into_iter().collect(),
                flags  : args.flag  .into_iter().collect(),
                unflags: args.unflag.into_iter().collect(),
                #[cfg(feature = "cache")] read_cache : args.read_cache,
                #[cfg(feature = "cache")] write_cache: args.write_cache
            }
        )
    }
}

impl TryFrom<Args> for (Vec<Url>, types::Config) {
    type Error = types::GetConfigError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let mut config = types::Config::get_default_or_load(args.config.as_deref())?.into_owned();
        let (urls, params_diff) = args.into();
        config.params.apply_diff(params_diff);
        Ok((urls, config))
    }
}

fn main() -> Result<(), types::CleaningError> {
    let args = Args::parse();

    let print_config = args.print_config;
    let test_config = args.test_config;
    let (urls, config): (Vec<Url>, types::Config) = args.try_into()?;

    if print_config {println!("{}", serde_json::to_string(&config)?);}
    if test_config {config.clone().run_tests();}
    if print_config || test_config {std::process::exit(0)}

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
                    Err(e) => {println!(); eprintln!("Line parse error: {e:?}");}
                },
                Err(e) => {println!(); eprintln!("Line read error: {e:?}");}
            }
        }
    }

    Ok(())
}
