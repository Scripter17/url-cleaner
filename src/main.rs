//! URL Cleaner originally started as a project to remove tracking garbage from URLs but has since grown into a very powerful URL manipulation tool.
use std::path::PathBuf;

#[cfg(feature = "stdin")]
use std::io;

use clap::Parser;
use url::Url;
use thiserror::Error;

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
    /// Additional ParamsDiffs to apply before the rest of the options.
    #[arg(             long)] params_diff: Vec<PathBuf>,
    /// Set flags.
    #[arg(short      , long)] flag  : Vec<String>,
    /// Unset flags set by the config.
    #[arg(short = 'F', long)] unflag: Vec<String>,
    /// Set variables using name=value syntax.
    #[arg(short      , long)] var   : Vec<String>,
    /// Unset variables set by the config.
    #[arg(short = 'V', long)] unvar : Vec<String>,
    /// For each occurence of this option, its first argument is the set name and subsequent arguments are the values to insert.
    #[arg(             long, num_args(2..))] insert_into_set: Vec<Vec<String>>,
    /// For each occurence of this option, its first argument is the set name and subsequent arguments are the values to remove.
    #[arg(             long, num_args(2..))] remove_from_set: Vec<Vec<String>>,
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
    /// Print the parsed arguments for debugging.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long)] print_args: bool,
    /// Print the ParamsDiffs loaded from `--params--diff` files and derived from the parsed arguments for debugging.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long)] print_params_diffs: bool,
    /// Print the config's params before applying the ParamsDiff.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long)] print_params: bool,
    /// Print the specified config as JSON before applying the ParamsDiff.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long)] print_config: bool,
    /// Print the config's params after applying the ParamsDiff.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long)] print_diffed_params: bool,
    /// Print the specified config'as JSON after applying the ParamsDiff.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long)] print_diffed_config: bool,
    /// Run the config's tests.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long)] test_config : bool
}

/// The enum of all errors that can occur when converting an [`Args`] to types usable by URL Cleaner.
#[derive(Debug, Error)]
pub enum InterpretArgsError {
    /// Returned when a [`GetConfigError`] is encountered.
    #[error(transparent)] GetConfigError(#[from] types::GetConfigError),
    /// Returned when URL Cleaner fails to load a [`ParamsDiff`] file.
    #[error(transparent)] CantLoadParamsDiffFile(std::io::Error),
    /// Returned when URL Cleaner fails to parse a [`ParamsDiff`] file's contents.
    #[error(transparent)] CantParseParamsDiffFile(serde_json::Error)
}

impl TryFrom<Args> for (Vec<Url>, types::Config, Vec<types::ParamsDiff>) {
    type Error = InterpretArgsError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        Ok((
            args.urls,
            types::Config::get_default_or_load(args.config.as_deref())?.into_owned(),
            {
                let mut ret = args.params_diff.into_iter().map(|path| serde_json::from_str(&std::fs::read_to_string(path).map_err(InterpretArgsError::CantLoadParamsDiffFile)?).map_err(InterpretArgsError::CantParseParamsDiffFile)).collect::<Result<Vec<_>, _>>()?;
                ret.push(types::ParamsDiff {
                    flags  : args.flag  .into_iter().collect(), // It's probably not a good thing to do a global impl for,
                    unflags: args.unflag.into_iter().collect(), // but surely once specialization lands in Rust 2150 it'll be fine?
                    vars   : args.var   .into_iter().filter_map(|mut kev| kev.find('=').map(|e| {let mut v=kev.split_off(e); v.drain(..1); kev.shrink_to_fit(); (kev, v)})).collect(),
                    unvars : args.unvar .into_iter().collect(), // `impl<X: IntoIterator, Y: FromIterator<<X as IntoIterator>::Item>> From<X> for Y`?
                    init_sets: Default::default(),
                    insert_into_sets: args.insert_into_set.clone().into_iter().map(|mut x| (x.swap_remove(0), x)).collect(),
                    remove_from_sets: args.remove_from_set.clone().into_iter().map(|mut x| (x.swap_remove(0), x)).collect(),
                    delete_sets: Default::default(),
                    #[cfg(feature = "cache")] read_cache : args.read_cache,
                    #[cfg(feature = "cache")] write_cache: args.write_cache,
                    #[cfg(all(feature = "http", not(target_family = "wasm")))] http_client_config_diff: Some(types::HttpClientConfigDiff {
                        set_proxies: args.http_proxy.map(|x| vec![x]),
                        no_proxy: args.no_http_proxy,
                        ..types::HttpClientConfigDiff::default()
                    })
                });
                ret
            }
        ))
    }
}

/// The enum of all errors that can occur when using the URL Cleaner CLI tool.
#[derive(Debug, Error)]
pub enum CliError {
    /// Returned when an [`InterpretArgsError`] is encountered.
    #[error(transparent)] InterpretArgsError(#[from] InterpretArgsError),
    /// Returned when a [`CleaningError`] is encountered.
    #[error(transparent)] CleaningError(#[from] types::CleaningError),
    /// Returned when a [`SerdeJsonError`] is encountered.
    #[error(transparent)] SerdeJsonError(#[from] serde_json::Error)
}

fn main() -> Result<(), CliError> {
    let args = Args::parse();

    let print_args          = args.print_args;
    let print_params_diffs  = args.print_params_diffs;
    let print_params        = args.print_params;
    let print_diffed_params = args.print_diffed_params;
    let print_config        = args.print_config;
    let print_diffed_config = args.print_diffed_config;
    let test_config         = args.test_config;

    let no_cleaning = print_args || print_params_diffs || print_params || print_diffed_params || print_config || print_diffed_config || test_config;

    if print_args {println!("{args:?}");}

    let (urls, mut config, params_diffs) = args.try_into()?;

    if print_params_diffs {println!("{}", serde_json::to_string(&params_diffs)?);}
    if print_params {println!("{}", serde_json::to_string(&config.params)?)};
    if print_config {println!("{}", serde_json::to_string(&config)?);}

    for params_diff in params_diffs {
        params_diff.apply(&mut config.params);
    }

    if print_diffed_config {println!("{}", serde_json::to_string(&config)?);}
    if print_diffed_params {println!("{}", serde_json::to_string(&config.params)?)};
    if test_config {config.run_tests();}

    if no_cleaning {std::process::exit(0);}

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
