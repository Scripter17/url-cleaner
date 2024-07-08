//! URL Cleaner originally started as a project to remove tracking garbage from URLs but has since grown into a very powerful URL manipulation tool.

use std::path::PathBuf;
#[cfg(feature = "stdin")]
use std::io;
#[cfg(feature = "debug")]
use std::sync::Mutex;

use clap::Parser;
use url::Url;
use thiserror::Error;

mod glue;
mod types;
mod util;

#[cfg(feature = "debug")]
pub(crate) static DEBUG_INDENT: Mutex<usize> = Mutex::new(0);

#[derive(Debug, Clone, Parser)]
/// URL Cleaner - Explicit non-consent to URL-based tracking.
/// 
/// Enabled features:
#[cfg_attr(feature = "default-config"         , doc = "default-config"         )]
#[cfg_attr(feature = "minify-included-strings", doc = "minify-included-strings")]
#[cfg_attr(feature = "stdin"                  , doc = "stdin"                  )]
#[cfg_attr(feature = "regex"                  , doc = "regex"                  )]
#[cfg_attr(feature = "glob"                   , doc = "glob"                   )]
#[cfg_attr(feature = "commands"               , doc = "commands"               )]
#[cfg_attr(feature = "http"                   , doc = "http"                   )]
#[cfg_attr(feature = "advanced-requests"      , doc = "advanced-requests"      )]
#[cfg_attr(feature = "cache"                  , doc = "cache"                  )]
#[cfg_attr(feature = "cache-redirects"        , doc = "cache-redirects"        )]
#[cfg_attr(feature = "debug"                  , doc = "debug"                  )]
/// 
/// Disabled features:
#[cfg_attr(not(feature = "default-config"         ), doc = "default-config"         )]
#[cfg_attr(not(feature = "minify-included-strings"), doc = "minify-included-strings")]
#[cfg_attr(not(feature = "stdin"                  ), doc = "stdin"                  )]
#[cfg_attr(not(feature = "regex"                  ), doc = "regex"                  )]
#[cfg_attr(not(feature = "glob"                   ), doc = "glob"                   )]
#[cfg_attr(not(feature = "commands"               ), doc = "commands"               )]
#[cfg_attr(not(feature = "http"                   ), doc = "http"                   )]
#[cfg_attr(not(feature = "advanced-requests"      ), doc = "advanced-requests"      )]
#[cfg_attr(not(feature = "cache"                  ), doc = "cache"                  )]
#[cfg_attr(not(feature = "cache-redirects"        ), doc = "cache-redirects"        )]
#[cfg_attr(not(feature = "debug"                  ), doc = "debug"                  )]
struct Args {
    /// The URLs to clean before the URLs in the STDIN.
    urls: Vec<Url>,
    /// The config.json to use. If unspecified, use the config compiled into URL Cleaner.
    #[arg(short      , long)] config: Option<PathBuf>,
    /// Output JSON.
    #[arg(short      , long)] json: bool,
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
    /// For each occurrence of this option, its first argument is the set name and subsequent arguments are the values to insert.
    #[arg(             long, num_args(2..))] insert_into_set: Vec<Vec<String>>,
    /// For each occurrence of this option, its first argument is the set name and subsequent arguments are the values to remove.
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
    #[arg(             long, num_args(0..=1), default_missing_value("true"))]
    no_http_proxy: Option<bool>,
    /// Print the parsed arguments for debugging.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)] print_args: bool,
    /// Print the ParamsDiffs loaded from `--params--diff` files and derived from the parsed arguments for debugging.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)] print_params_diffs: bool,
    /// Print the config's params before applying the ParamsDiff.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)] print_params: bool,
    /// Print the specified config as JSON before applying the ParamsDiff.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)] print_config: bool,
    /// Print the config's params after applying the ParamsDiff.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)] print_diffed_params: bool,
    /// Print the specified config's JSON after applying the ParamsDiff.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)] print_diffed_config: bool,
    /// Run the config's tests.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)] test_config : bool
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

fn str_to_json_str(s: &str) -> String {
    serde_json::to_string(s).expect("Serializing a string to never fail.")
}

fn main() -> Result<(), CliError> {
    let args = Args::parse();

    let json = args.json;

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

    if json {
        print!("{{\"urls\":[");
        let mut first_url = true;

        for mut url in urls {
            if !first_url {print!(",");}
            match config.apply(&mut url) {
                Ok(()) => {print!("{{\"Ok\":{:?}}}", url.as_str());},
                Err(e) => {print!("{{\"Err\":{{\"type\":\"RuleError\",\"source_url\":{},\"error\":{}}}}}", str_to_json_str(url.as_str()), str_to_json_str(&e.to_string()));}
            }
            first_url = false;
        }

        #[cfg(feature = "stdin")]
        if atty::isnt(atty::Stream::Stdin) {
            for maybe_line in io::stdin().lines() {
                if !first_url {print!(",");}
                match maybe_line {
                    Ok(line) => match Url::parse(&line) {
                        Ok(mut url) => match config.apply(&mut url) {
                            Ok(()) => {print!("{{\"Ok\":{}}}", str_to_json_str(url.as_str()));},
                            Err(e) => {print!("{{\"Err\":{{\"type\":\"RuleError\",\"source\":{},\"error\":{}}}}}", str_to_json_str(url.as_str()), str_to_json_str(&e.to_string()));}
                        },
                        Err(e) => {print!("{{\"Err\":{{\"type\":\"UrlParseError\",\"source\":{},\"error\":{}}}}}", str_to_json_str(&line), str_to_json_str(&e.to_string()));}
                    },
                    Err(e) => {print!("{{\"Err\":{{\"type\":\"LineReadError\",\"Error\":{}}}}}", str_to_json_str(&e.to_string()));}
                }
                first_url = false;
            }
        }

        print!("]}}");
    } else {
        for mut url in urls {
            match config.apply(&mut url) {
                Ok(()) => {println!("{url}");},
                Err(e) => {println!(); eprintln!("Rule error\t{:?}\t{e:?}", url.as_str());}
            }
        }

        #[cfg(feature = "stdin")]
        if atty::isnt(atty::Stream::Stdin) {
            for maybe_line in io::stdin().lines() {
                match maybe_line {
                    Ok(line) => match Url::parse(&line) {
                        Ok(mut url) => match config.apply(&mut url) {
                            Ok(()) => {println!("{url}");},
                            Err(e) => {println!(); eprintln!("Rule error\t{:?}\t{e:?}", url.as_str());}
                        },
                        Err(e) => {println!(); eprintln!("Line parse error\t{line:?}\t{e:?}");}
                    },
                    Err(e) => {println!(); eprintln!("Line read error\t\t{e:?}");}
                }
            }
        }
    }

    Ok(())
}
