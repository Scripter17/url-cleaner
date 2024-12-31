//! URL Cleaner originally started as a project to remove tracking garbage from URLs but has since grown into a very powerful URL manipulation tool.

use std::path::PathBuf;
use std::io::{self, IsTerminal};
use std::borrow::Cow;
use std::process::ExitCode;
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
/// Released under the GNU Affero General Public License 3.0 or later (AGPL-3.0-or-later).
/// 
/// Source code: https://github.com/Scripter17/url-cleaner
/// 
/// Enabled features:
#[cfg_attr(feature = "default-config"     , doc = "default-config"     )]
#[cfg_attr(feature = "regex"              , doc = "regex"              )]
#[cfg_attr(feature = "glob"               , doc = "glob"               )]
#[cfg_attr(feature = "http"               , doc = "http"               )]
#[cfg_attr(feature = "cache"              , doc = "cache"              )]
#[cfg_attr(feature = "base64"             , doc = "base64"             )]
#[cfg_attr(feature = "commands"           , doc = "commands"           )]
#[cfg_attr(feature = "custom"             , doc = "custom"             )]
#[cfg_attr(feature = "experiment-parallel", doc = "experiment-parallel")]
#[cfg_attr(feature = "debug"              , doc = "debug"              )]
/// 
/// Disabled features:
#[cfg_attr(not(feature = "default-config"     ), doc = "default-config"     )]
#[cfg_attr(not(feature = "regex"              ), doc = "regex"              )]
#[cfg_attr(not(feature = "glob"               ), doc = "glob"               )]
#[cfg_attr(not(feature = "http"               ), doc = "http"               )]
#[cfg_attr(not(feature = "cache"              ), doc = "cache"              )]
#[cfg_attr(not(feature = "base64"             ), doc = "base64"             )]
#[cfg_attr(not(feature = "commands"           ), doc = "commands"           )]
#[cfg_attr(not(feature = "custom"             ), doc = "custom"             )]
#[cfg_attr(not(feature = "experiment-parallel"), doc = "experiment-parallel")]
#[cfg_attr(not(feature = "debug"              ), doc = "debug"              )]
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
    /// Overrides the config's [`Config::cache_path`].
    #[cfg(feature = "cache")]
    #[arg(             long)]
    pub cache_path: Option<CachePath>,
    /// Output JSON. It is intended to be identical to URL Cleaner Site's output, so while some of the output is "redundant", it's important.
    #[arg(short      , long)]
    pub json: bool,
    /// Additional ParamsDiffs to apply before the rest of the options.
    #[arg(             long)]
    pub params_diff: Vec<PathBuf>,
    /// Stuff to make a [`ParamsDiff`] from the CLI.
    #[command(flatten)]
    pub params_diff_args: ParamsDiffArgParser,
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
    /// Run the config's tests.
    /// When this or any `--print-...` flag is set, no URLs are cleaned.
    #[arg(             long, verbatim_doc_comment)]
    pub test_config : bool,
    /// Amount of threads to process jobs in.
    /// 
    /// Zero gets the current CPU threads.
    #[cfg(feature = "experiment-parallel")]
    #[arg(long, default_value_t = 0)]
    pub threads: usize
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

/// Shorthand for serializing a string to JSON.
fn str_to_json_str(s: &str) -> String {
    serde_json::to_string(s).expect("Serializing a string to never fail.")
}

fn main() -> Result<ExitCode, CliError> {
    let some_ok = std::sync::Mutex::new(false);
    let some_error = std::sync::Mutex::new(false);

    let args = Args::parse();


    let print_args = args.print_args;
    if print_args {println!("{args:?}");}

    #[cfg(feature = "default-config")]
    let mut config = Config::get_default_no_cache_or_load(args.config.as_deref())?;
    #[cfg(not(feature = "default-config"))]
    let mut config = Config::load_from_file(&args.config)?;

    let mut params_diffs: Vec<ParamsDiff> = args.params_diff
        .into_iter()
        .map(|path| serde_json::from_str(&std::fs::read_to_string(path).map_err(CliError::CantLoadParamsDiffFile)?).map_err(CliError::CantParseParamsDiffFile))
        .collect::<Result<Vec<_>, _>>()?;
    if args.params_diff_args.does_anything() {
        match args.params_diff_args.try_into() {
            Ok(params_diff) => params_diffs.push(params_diff),
            Err(e) => Args::command()
                .error(
                    clap::error::ErrorKind::WrongNumberOfValues,
                    match e {
                        ParamsDiffArgParserValueWrong::InsertIntoMapNeedsAMap   => "--insert-into-map needs a map to insert key-value pairs into.",
                        ParamsDiffArgParserValueWrong::InsertIntoMapNeedsAValue => "--insert-into-map found a key without a value at the end.",
                        ParamsDiffArgParserValueWrong::RemoveFromMapNeedsAMap   => "--remove-from-map needs a map to remove keys from."
                    }
                )
                .exit()
        }
    }

    let print_params_diffs = args.print_params_diffs;
    if print_params_diffs {println!("{}", serde_json::to_string(&params_diffs)?);}

    for params_diff in params_diffs {
        params_diff.apply(&mut config.params);
    }

    let json = args.json;

    let print_params = args.print_params;
    let print_config = args.print_config;
    let test_config  = args.test_config;

    let no_cleaning = print_args || print_params_diffs || print_params || print_config || test_config;

    if print_params {println!("{}", serde_json::to_string(&config.params)?);}
    if print_config {println!("{}", serde_json::to_string(&config)?);}
    if test_config {config.run_tests();}

    if no_cleaning {std::process::exit(0);}

    #[cfg(feature = "experiment-parallel")]
    {
        let mut threads = args.threads;
        if threads == 0 {threads = std::thread::available_parallelism().expect("To be able to get the available parallelism.").into();}
        let (in_senders , in_recievers ) = (0..threads).map(|_| std::sync::mpsc::channel::<Result<String, io::Error>>()).collect::<(Vec<_>, Vec<_>)>();
        let (out_senders, out_recievers) = (0..threads).map(|_| std::sync::mpsc::channel::<Result<Result<url::Url, DoJobError>, MakeJobError>>()).collect::<(Vec<_>, Vec<_>)>();

        let config_ref = &config;
        #[cfg(feature = "cache")]
        let cache: Cache = args.cache_path.as_ref().unwrap_or(&config.cache_path).clone().into();
        #[cfg(feature = "cache")]
        let cache_ref = &cache;

        std::thread::scope(|s| {
            s.spawn(move || {
                let job_config_strings_source: Box<dyn Iterator<Item = Result<String, io::Error>>> = {
                    let ret = args.urls.into_iter().map(Ok);
                    if !io::stdin().is_terminal() {
                        Box::new(ret.chain(io::stdin().lines()))
                    } else {
                        Box::new(ret)
                    }
                };

                for (i, job_config_string) in job_config_strings_source.enumerate() {
                    #[allow(clippy::arithmetic_side_effects, reason = "Whatever exactly the issue with `i % threads` is it will, at worst, give slightly worse load balancing around each multiple of usize::MAX jobs. I think that's fine.")]
                    in_senders.get(i % threads).expect("The amount of senders to not exceed the count of senders to make.").send(job_config_string).expect("To successfuly send the Job.");
                }
            });

            in_recievers.into_iter().zip(out_senders).map(|(ir, os)| {
                s.spawn(move || {
                    while let Ok(maybe_job_config_string) = ir.recv() {
                        let ret = match maybe_job_config_string {
                            Ok(job_config_string) => JobConfig::from_str(&job_config_string)
                                .map(|JobConfig{url, context}|
                                    Job {
                                        url,
                                        context,
                                        config: config_ref,
                                        #[cfg(feature = "cache")]
                                        cache: cache_ref
                                    }.r#do()
                                )
                                .map_err(MakeJobError::MakeJobConfigError),
                            Err(e) => Err(MakeJobError::MakeJobConfigError(MakeJobConfigError::IoError(e)))
                        };

                        os.send(ret).expect("The receiver to still exist.");
                    }
                });
            }).for_each(drop);

            let some_ok_ref = &some_ok;
            let some_error_ref = &some_error;

            if json {
                s.spawn(move || {
                    let mut disconnected = 0usize;
                    let mut first_job = true;
                    let mut some_ok_ref_lock = some_ok_ref.lock().expect("No panics.");
                    let mut some_error_ref_lock = some_error_ref.lock().expect("No panics.");

                    print!("{{\"Ok\":{{\"urls\":[");
                    for or in out_recievers.iter().cycle() {
                        match or.recv() {
                            Ok(Ok(Ok(url))) => {
                                if !first_job {print!(",");}
                                print!("{{\"Ok\":{{\"Ok\":{}}}}}", str_to_json_str(url.as_str()));
                                *some_ok_ref_lock = true;
                                first_job = false;
                            },
                            Ok(Ok(Err(e))) => {
                                if !first_job {print!(",");}
                                print!("{{\"Ok\":{{\"Err\":{{\"message\":{},\"variant\":{}}}}}}}", str_to_json_str(&e.to_string()), str_to_json_str(&format!("{e:?}")));
                                *some_error_ref_lock = true;
                                first_job = false;
                            },
                            Ok(Err(e)) => {
                                if !first_job {print!(",");}
                                print!("{{\"Err\":{{\"message\":{},\"variant\":{}}}}}", str_to_json_str(&e.to_string()), str_to_json_str(&format!("{e:?}")));
                                *some_error_ref_lock = true;
                                first_job = false;
                            },
                            Err(_) => {
                                #[allow(clippy::arithmetic_side_effects, reason = "Can't even come close to usize::MAX threads and this is capped by thread count.")]
                                {disconnected += 1;}
                                if disconnected == threads {break;}
                            }
                        }
                    }

                    print!("]}}}}");
                });
            } else {
                s.spawn(move || {
                    let mut disconnected = 0usize;
                    let mut some_ok_ref_lock  = some_ok_ref .lock().expect("No panics.");
                    let mut some_error_ref_lock = some_error_ref.lock().expect("No panics.");

                    for or in out_recievers.iter().cycle() {
                        match or.recv() {
                            Ok(Ok(Ok(url))) => {
                                println!("{url}");
                                *some_ok_ref_lock = true;
                            },
                            Ok(Ok(Err(e))) => {
                                println!();
                                eprintln!("DoJobError\t{e:?}");
                                *some_error_ref_lock = true;
                            }
                            Ok(Err(e)) => {
                                println!();
                                eprintln!("MakeJobError\t{e:?}");
                                *some_error_ref_lock = true;
                            }
                            Err(_) => {
                                #[allow(clippy::arithmetic_side_effects, reason = "Can't even come close to usize::MAX threads and this is capped by thread count.")]
                                {disconnected += 1;}
                                if disconnected == threads {break;}
                            }
                        }
                    }
                });
            }
        })
    }

    #[cfg(not(feature = "experiment-parallel"))]
    {
        let mut jobs = Jobs {
            #[cfg(feature = "cache")]
            cache: args.cache_path.as_ref().unwrap_or(&config.cache_path).clone().into(),
            job_configs_source: {
                let ret = args.urls.into_iter().map(|url| JobConfig::from_str(&url));
                if !io::stdin().is_terminal() {
                    Box::new(ret.chain(io::stdin().lines().map(|line| JobConfig::from_str(&line?))))
                } else {
                    Box::new(ret)
                }
            },
            config: Cow::Owned(config)
        };

        if json {
            print!("{{\"Ok\":{{\"urls\":[");
            let mut first_job = true;

            for job in jobs.iter() {
                if !first_job {print!(",");}
                match job {
                    Ok(job) => match job.r#do() {
                        Ok(url) => {
                            print!("{{\"Ok\":{{\"Ok\":{}}}}}", str_to_json_str(url.as_str()));
                            *some_ok.lock().expect("No panics.") = true;
                        },
                        Err(e) => {
                            print!("{{\"Ok\":{{\"Err\":{{\"message\":{},\"variant\":{}}}}}}}", str_to_json_str(&e.to_string()), str_to_json_str(&format!("{e:?}")));
                            *some_error.lock().expect("No panics.") = true;
                        }
                    },
                    Err(e) => {
                        print!("{{\"Err\":{{\"message\":{},\"variant\":{}}}}}", str_to_json_str(&e.to_string()), str_to_json_str(&format!("{e:?}")));
                        *some_error.lock().expect("No panics.") = true;
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
                            *some_ok.lock().expect("No panics.") = true;
                        },
                        Err(e) => {
                            println!();
                            eprintln!("DoJobError\t{e:?}");
                            *some_error.lock().expect("No panics.") = true;
                        }
                    },
                    Err(e) => {
                        println!();
                        eprintln!("MakeJobError\t{e:?}");
                        *some_error.lock().expect("No panics.") = true;
                    }
                }
            }
        }
    }

    return Ok(match (*some_ok.lock().expect("No panics."), *some_error.lock().expect("No panics.")) {
        (false, false) => 0,
        (false, true ) => 1,
        (true , false) => 0,
        (true , true ) => 2
    }.into());
}
