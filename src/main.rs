//! URL Cleaner originally started as a project to remove tracking garbage from URLs but has since grown into a very powerful URL manipulation tool.

use std::path::PathBuf;
use std::io::{self, IsTerminal};
use std::borrow::Cow;
use std::process::ExitCode;
use std::collections::HashMap;
use std::str::FromStr;

use clap::{Parser, CommandFactory};
use thiserror::Error;

mod glue;
use glue::*;
mod types;
use types::*;
mod util;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
/// URL Cleaner - Explicit non-consent to URL-based tracking.
/// 
/// Enabled features:
#[cfg_attr(feature = "default-config"         , doc = "default-config"         )]
#[cfg_attr(feature = "minify-included-strings", doc = "minify-included-strings")]
#[cfg_attr(feature = "regex"                  , doc = "regex"                  )]
#[cfg_attr(feature = "glob"                   , doc = "glob"                   )]
#[cfg_attr(feature = "commands"               , doc = "commands"               )]
#[cfg_attr(feature = "http"                   , doc = "http"                   )]
#[cfg_attr(feature = "advanced-http"          , doc = "advanced-http"          )]
#[cfg_attr(feature = "cache"                  , doc = "cache"                  )]
#[cfg_attr(feature = "base64"                 , doc = "base64"                 )]
#[cfg_attr(feature = "cache-redirects"        , doc = "cache-redirects"        )]
#[cfg_attr(feature = "debug"                  , doc = "debug"                  )]
/// 
/// Disabled features:
#[cfg_attr(not(feature = "default-config"         ), doc = "default-config"         )]
#[cfg_attr(not(feature = "minify-included-strings"), doc = "minify-included-strings")]
#[cfg_attr(not(feature = "regex"                  ), doc = "regex"                  )]
#[cfg_attr(not(feature = "glob"                   ), doc = "glob"                   )]
#[cfg_attr(not(feature = "commands"               ), doc = "commands"               )]
#[cfg_attr(not(feature = "http"                   ), doc = "http"                   )]
#[cfg_attr(not(feature = "advanced-http"          ), doc = "advanced-http"          )]
#[cfg_attr(not(feature = "cache"                  ), doc = "cache"                  )]
#[cfg_attr(not(feature = "base64"                 ), doc = "base64"                 )]
#[cfg_attr(not(feature = "cache-redirects"        ), doc = "cache-redirects"        )]
#[cfg_attr(not(feature = "debug"                  ), doc = "debug"                  )]
pub struct Args {
    /// The URLs to clean before the URLs in the STDIN.
    pub urls: Vec<String>,
    /// The JSON config to use. If unspecified and URL Cleaner was compiled with the default-config feature, use the default config compiled into URL Cleaner.
    #[cfg(feature = "default-config")]
    #[arg(short      , long)]
    pub config: Option<PathBuf>,
    /// The JSON config to use. Has to be set because this instance of URL Cleaner was compiled without a default config.
    #[cfg(not(feature = "default-config"))]
    #[arg(short      , long)]
    pub config: PathBuf,
    /// Output JSON. It is intended to be identical to URL Cleaner Site's output, so while some of the output is "redundant", it's important.
    #[arg(short      , long)]
    pub json: bool,
    /// Additional ParamsDiffs to apply before the rest of the options.
    #[arg(             long)]
    pub params_diff: Vec<PathBuf>,
    /// Set flags.
    #[arg(short      , long, value_names = ["NAME"])]
    pub flag  : Vec<String>,
    /// Unset flags set by the config.
    #[arg(short = 'F', long, value_names = ["NAME"])]
    pub unflag: Vec<String>,
    /// For each occurrence of this option, its first argument is the variable name and the second argument is its value.
    #[arg(short      , long, num_args(2), value_names = ["NAME", "VALUE"])]
    pub var: Vec<Vec<String>>,
    /// Unset variables set by the config.
    #[arg(short = 'V', long, value_names = ["NAME"])]
    pub unvar : Vec<String>,
    /// For each occurrence of this option, its first argument is the set name and subsequent arguments are the values to insert.
    #[arg(             long, num_args(2..), value_names = ["NAME", "VALUE"])]
    pub insert_into_set: Vec<Vec<String>>,
    /// For each occurrence of this option, its first argument is the set name and subsequent arguments are the values to remove.
    #[arg(             long, num_args(2..), value_names = ["NAME", "VALUE"])]
    pub remove_from_set: Vec<Vec<String>>,
    /// For each occurrence of this option, its first argument is the map name, the second is the map key, and subsequent arguments are the values to insert.
    #[arg(             long, num_args(3..), value_names = ["NAME", "KEY1", "VALUE1"])]
    pub insert_into_map: Vec<Vec<String>>,
    /// For each occurrence of this option, its first argument is the map name, the second is the map key, and subsequent arguments are the values to remove.
    #[arg(             long, num_args(2..), value_names = ["NAME", "KEY1"])]
    pub remove_from_map: Vec<Vec<String>>,
    /// Read stuff from caches. Default value is controlled by the config. Omitting a value means true.
    #[cfg(feature = "cache")]
    #[arg(             long, num_args(0..=1), default_missing_value("true"))]
    pub read_cache : Option<bool>,
    /// Write stuff to caches. Default value is controlled by the config. Omitting a value means true.
    #[cfg(feature = "cache")]
    #[arg(             long, num_args(0..=1), default_missing_value("true"))]
    pub write_cache: Option<bool>,
    /// The proxy to send HTTP requests over. Example: socks5://localhost:9150
    #[cfg(feature = "http")]
    #[arg(             long)]
    pub http_proxy: Option<ProxyConfig>,
    /// Disables all HTTP proxying.
    #[cfg(feature = "http")]
    #[arg(             long, num_args(0..=1), default_missing_value("true"))]
    pub no_http_proxy: Option<bool>,
    /// Overrides the config's [`Config::cache_path`].
    #[arg(             long)]
    pub cache_path: Option<String>,
    /// Print the parsed arguments for debugging.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)]
    pub print_args: bool,
    /// Print the ParamsDiffs loaded from `--params--diff` files and derived from the parsed arguments for debugging.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)]
    pub print_params_diffs: bool,
    /// Print the config's params after applying the ParamsDiff.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)]
    pub print_params: bool,
    /// Print the specified config as JSON after applying the ParamsDiff.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)]
    pub print_config: bool,
    /// Print the config's documentation.
    /// When this, any other `--print-...` flag, or `--test-config` is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)]
    pub print_docs: bool,
    /// Run the config's tests.
    /// When this or any other `--print-...` flag is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)]
    pub test_config : bool
}

/// The enum of all errors that can occur when using the URL Cleaner CLI tool.
#[derive(Debug, Error)]
pub enum CliError {
    /// Returned when a [`GetConfigError`] is encountered.
    #[error(transparent)] GetConfigError(#[from] GetConfigError),
    /// Returned when URL Cleaner fails to load a [`ParamsDiff`] file.
    #[error(transparent)] CantLoadParamsDiffFile(std::io::Error),
    /// Returned when URL Cleaner fails to parse a [`ParamsDiff`] file's contents.
    #[error(transparent)] CantParseParamsDiffFile(serde_json::Error),
    /// Returned when a [`SerdeJsonError`] is encountered.
    #[error(transparent)] SerdeJsonError(#[from] serde_json::Error)
}

fn str_to_json_str(s: &str) -> String {
    serde_json::to_string(s).expect("Serializing a string to never fail.")
}

fn main() -> Result<ExitCode, CliError> {
    #[cfg(feature = "debug-time")] let start_time = std::time::Instant::now();

    let mut some_ok = false;
    let mut some_error = false;

    #[cfg(feature = "debug-time")] let x = std::time::Instant::now();

    let args = Args::parse();

    for invocation in args.insert_into_map.iter() {
        if invocation.is_empty() {
            Args::command()
                .error(clap::error::ErrorKind::WrongNumberOfValues, "--insert-into-map needs a map to insert key-value pairs into.")
                .exit();
        }
        if invocation.len() % 2 != 1 {
            Args::command()
                .error(clap::error::ErrorKind::WrongNumberOfValues, "--insert-into-map found a key without a value at the end.")
                .exit();
        }
    }

    for invocation in args.remove_from_map.iter() {
        if invocation.is_empty() {
            Args::command()
                .error(clap::error::ErrorKind::WrongNumberOfValues, "--remove-from-map needs a map to remove keys from.")
                .exit();
        }
    }

    #[cfg(feature = "debug-time")] eprintln!("Parse args: {:?}", x.elapsed());

    let print_args = args.print_args;
    if print_args {println!("{args:?}");}

    #[cfg(feature = "debug-time")] let x = std::time::Instant::now();

    #[cfg(feature = "default-config")]
    let mut config = Config::get_default_no_cache_or_load(args.config.as_deref())?;
    #[cfg(not(feature = "default-config"))]
    let mut config = Config::load_from_file(&args.config)?;

    #[cfg(feature = "debug-time")] eprintln!("Load Config: {:?}", x.elapsed());
    #[cfg(feature = "debug-time")] let x = std::time::Instant::now();

    let mut params_diffs = args.params_diff
        .into_iter()
        .map(|path| serde_json::from_str(&std::fs::read_to_string(path).map_err(CliError::CantLoadParamsDiffFile)?).map_err(CliError::CantParseParamsDiffFile))
        .collect::<Result<Vec<_>, _>>()?;
    #[allow(unused_mut, reason = "Attributes on expressions WHEN. PLEASE.")]
    let mut feature_flag_make_params_diff = false;
    #[cfg(feature = "cache")] #[allow(clippy::unnecessary_operation, reason = "False positive.")] {feature_flag_make_params_diff = feature_flag_make_params_diff || args.read_cache.is_some()};
    #[cfg(feature = "cache")] #[allow(clippy::unnecessary_operation, reason = "False positive.")] {feature_flag_make_params_diff = feature_flag_make_params_diff || args.write_cache.is_some()};
    #[cfg(feature = "http" )] #[allow(clippy::unnecessary_operation, reason = "False positive.")] {feature_flag_make_params_diff = feature_flag_make_params_diff || args.http_proxy.is_some()};
    if !args.flag.is_empty() || !args.unflag.is_empty() || !args.var.is_empty() || !args.unvar.is_empty() || !args.insert_into_set.is_empty() || !args.remove_from_set.is_empty() || !args.insert_into_map.is_empty() || !args.remove_from_map.is_empty() || feature_flag_make_params_diff {
        params_diffs.push(ParamsDiff {
            flags  : args.flag  .into_iter().collect(), // `impl<X: IntoIterator, Y: FromIterator<<X as IntoIterator>::Item>> From<X> for Y`?
            unflags: args.unflag.into_iter().collect(), // It's probably not a good thing to do a global impl for,
            vars   : args.var   .into_iter().map(|x| x.try_into().expect("Clap guarantees the length is always 2")).map(|[name, value]: [String; 2]| (name, value)).collect(), // Either let me TryFrom a Vec into a tuple or let me collect a [T; 2] into a HashMap. Preferably both.
            unvars : args.unvar .into_iter().collect(), // but surely once specialization lands in Rust 2150 it'll be fine?
            init_sets: Default::default(),
            insert_into_sets: args.insert_into_set.into_iter().map(|mut x| (x.swap_remove(0), x)).collect(),
            remove_from_sets: args.remove_from_set.into_iter().map(|mut x| (x.swap_remove(0), x)).collect(),
            delete_sets     : Default::default(),
            init_maps       : Default::default(),
            insert_into_maps: args.insert_into_map.into_iter().map(|x| {
                let mut values = HashMap::new();
                let mut args_iter = x.into_iter();
                let map = args_iter.next().expect("The validation to have worked.");
                while let Some(k) = args_iter.next() {
                    values.insert(k, args_iter.next().expect("The validation to have worked."));
                }
                (map, values)
            }).collect::<HashMap<_, _>>(),
            remove_from_maps: args.remove_from_map.into_iter().map(|mut x| (x.swap_remove(0), x)).collect::<HashMap<_, _>>(),
            delete_maps     : Default::default(),
            #[cfg(feature = "cache")] read_cache : args.read_cache,
            #[cfg(feature = "cache")] write_cache: args.write_cache,
            #[cfg(feature = "http")] http_client_config_diff: Some(HttpClientConfigDiff {
                set_proxies: args.http_proxy.map(|x| vec![x]),
                no_proxy: args.no_http_proxy,
                ..HttpClientConfigDiff::default()
            })
        });
    }

    #[cfg(feature = "debug-time")] eprintln!("Args to ParamsDiffs: {:?}", x.elapsed());

    let print_params_diffs = args.print_params_diffs;
    if print_params_diffs {println!("{}", serde_json::to_string(&params_diffs)?);}

    #[cfg(feature = "debug-time")] let x = std::time::Instant::now();

    for params_diff in params_diffs {
        params_diff.apply(&mut config.params);
    }

    #[cfg(feature = "debug-time")] eprintln!("Apply ParamsDiffs: {:?}", x.elapsed());

    let json = args.json;

    let print_params = args.print_params;
    let print_config = args.print_config;
    let print_docs   = args.print_docs;
    let test_config  = args.test_config;

    let no_cleaning = print_args || print_params_diffs || print_params || print_config || print_docs || test_config;

    if print_params {println!("{}", serde_json::to_string(&config.params)?);}
    if print_config {println!("{}", serde_json::to_string(&config)?);}
    if print_docs {println!("{}", config.docs.to_markdown());}
    if test_config {config.run_tests();}

    if no_cleaning {std::process::exit(0);}

    #[cfg(feature = "debug-time")] let x = std::time::Instant::now();

    let mut jobs = Jobs {
        #[cfg(feature = "cache")]
        cache: args.cache_path.as_deref().unwrap_or(&*config.cache_path).into(),
        job_config_source: {
            let ret = args.urls.into_iter().map(|url| JobConfig::from_str(&url).map_err(Into::into));
            if !io::stdin().is_terminal() {
                Box::new(ret.chain(io::stdin().lines().map(|line| JobConfig::from_str(&line?).map_err(Into::into))))
            } else {
                Box::new(ret)
            }
        },
        config: Cow::Owned(config)
    };

    #[cfg(feature = "debug-time")] eprintln!("Make Jobs: {:?}", x.elapsed());
    #[cfg(feature = "debug-time")] let x = std::time::Instant::now();

    if json {
        print!("{{\"Ok\":{{\"urls\":[");
        let mut first_job = true;

        for job in jobs.iter() {
            if !first_job {print!(",");}
            match job {
                Ok(job) => match job.r#do() {
                    Ok(url) => {
                        print!("{{\"Ok\":{{\"Ok\":{}}}}}", str_to_json_str(url.as_str()));
                        some_ok = true;
                    },
                    Err(e) => {
                        print!("{{\"Ok\":{{\"Err\":{{\"message\":{},\"variant\":{}}}}}}}", str_to_json_str(&e.to_string()), str_to_json_str(&format!("{e:?}")));
                        some_error = true;
                    }
                },
                Err(e) => {
                    print!("{{\"Err\":{{\"message\":{},\"variant\":{}}}}}", str_to_json_str(&e.to_string()), str_to_json_str(&format!("{e:?}")));
                    some_error = true;
                }
            }
            first_job = false;
        }

        print!("]}}}}");
    } else {
        for job in jobs.iter() {
            match job {
                Ok(job) => match job.r#do() {
                    Ok(url) => {
                        println!("{url}");
                        some_ok = true;
                    },
                    Err(e) => {
                        println!();
                        eprintln!("DoJobError\t{e:?}");
                        some_error = true;
                    }
                },
                Err(e) => {
                    println!();
                    eprintln!("GetJobError\t{e:?}");
                    some_error = true;
                }
            }
        }
    }

    #[cfg(feature = "debug-time")] eprintln!("Run Jobs: {:?}", x.elapsed());
    #[cfg(feature = "debug-time")] let x = std::time::Instant::now();

    #[cfg(feature = "debug-time")] drop(jobs);

    #[cfg(feature = "debug-time")] eprintln!("Drop Jobs: {:?}", x.elapsed());
    #[cfg(feature = "debug-time")] eprintln!("Total: {:?}", start_time.elapsed());

    Ok(match (some_ok, some_error) {
        (false, false) => 0,
        (false, true ) => 1,
        (true , false) => 0,
        (true , true ) => 2
    }.into())
}
