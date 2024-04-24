//! URL Cleaner originally started as a project to remove tracking garbage from URLs but has since grown into a very powerful URL manipulation tool.

use std::path::PathBuf;

#[cfg(feature = "stdin")]
use std::io;

use clap::Parser;
use url::Url;

mod glue;
mod types;
mod util;

#[derive(Debug, Clone, Parser)]
/// URL Cleaner - Explicit non-consent to URL-based tracking.
struct Args {
    /// The URLs to clean before the URLs in the STDIN.
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
    /// Read stuff from caches. Default value is controlled by the config. Omitting a value means true.
    #[cfg(feature = "cache")]
    #[arg(             long, num_args(0..=1), default_missing_value("true"))]
    read_cache : Option<bool>,
    /// Write stuff to caches. Default value is controlled by the config. Omitting a value means true.
    #[cfg(feature = "cache")]
    #[arg(             long, num_args(0..=1), default_missing_value("true"))]
    write_cache: Option<bool>,
    /// The proxy to send HTTP requests over. Example: socks5://localhost:9150
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[arg(             long)] http_proxy: Option<glue::ProxyConfig>,
    /// Disables all HTTP proxying.
    #[cfg(all(feature = "http", not(target_family = "wasm")))]
    #[arg(             long)] no_http_proxy: Option<bool>,
    /// Print the config's contents then exit.
    #[arg(             long)] print_config: bool,
    /// Run the config's tests then exit.
    #[arg(             long)] test_config : bool
}

impl From<Args> for (Vec<Url>, types::ParamsDiff) {
    fn from(args: Args) -> Self {
        (
            args.urls,
            types::ParamsDiff {
                vars   : args.var   .into_iter().filter_map(|mut kev| kev.find('=').map(|e| {let mut v=kev.split_off(e); v.drain(..1); kev.shrink_to_fit(); (kev, v)})).collect(),
                unvars : args.unvar .into_iter().collect(), // `impl<X: IntoIterator, Y: FromIterator<<X as IntoIterator>::Item>> From<X> for Y`?
                flags  : args.flag  .into_iter().collect(), // It's probably not a good thing to do a global impl for,
                unflags: args.unflag.into_iter().collect(), // but surely once specialization lands in Rust 2150 it'll be fine?
                #[cfg(feature = "cache")] read_cache : args.read_cache,
                #[cfg(feature = "cache")] write_cache: args.write_cache,
                #[cfg(all(feature = "http", not(target_family = "wasm")))] http_client_config_diff: Some(types::HttpClientConfigDiff {
                    set_proxies: args.http_proxy.map(|x| vec![x]),
                    no_proxy: args.no_http_proxy,
                    ..types::HttpClientConfigDiff::default()
                })
            }
        )
    }
}

impl TryFrom<Args> for (Vec<Url>, types::Config) {
    type Error = types::GetConfigError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let mut config = types::Config::get_default_or_load(args.config.as_deref())?.into_owned();
        let (urls, params_diff) = args.into();
        params_diff.apply(&mut config.params);
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
