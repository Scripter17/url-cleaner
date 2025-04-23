//! URL Cleaner - Explicit non-consent to URL spytext.

use std::path::PathBuf;
use std::io::{self, IsTerminal};
use std::process::ExitCode;

use clap::{Parser, CommandFactory};
use thiserror::Error;

mod glue;
use glue::*;
mod types;
use types::*;
mod testing;
mod util;

/// URL Cleaner - Explicit non-consent to URL spytext.
///
/// Licensed under the Aferro GNU Public License version 3.0 or later (SPDX: AGPL-3.0-or-later)
///
/// Source code available at https://github.com/Scripter17/url-cleaner
///
/// Enabled features:
#[cfg_attr(feature = "default-config"     , doc = "default-config")]
#[cfg_attr(feature = "regex"              , doc = "regex"         )]
#[cfg_attr(feature = "glob"               , doc = "glob"          )]
#[cfg_attr(feature = "http"               , doc = "http"          )]
#[cfg_attr(feature = "cache"              , doc = "cache"         )]
#[cfg_attr(feature = "base64"             , doc = "base64"        )]
#[cfg_attr(feature = "commands"           , doc = "commands"      )]
#[cfg_attr(feature = "custom"             , doc = "custom"        )]
#[cfg_attr(feature = "debug"              , doc = "debug"         )]
///
/// Disabled features:
#[cfg_attr(not(feature = "default-config"), doc = "default-config")]
#[cfg_attr(not(feature = "regex"         ), doc = "regex"         )]
#[cfg_attr(not(feature = "glob"          ), doc = "glob"          )]
#[cfg_attr(not(feature = "http"          ), doc = "http"          )]
#[cfg_attr(not(feature = "cache"         ), doc = "cache"         )]
#[cfg_attr(not(feature = "base64"        ), doc = "base64"        )]
#[cfg_attr(not(feature = "commands"      ), doc = "commands"      )]
#[cfg_attr(not(feature = "custom"        ), doc = "custom"        )]
#[cfg_attr(not(feature = "debug"         ), doc = "debug"         )]
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(version = env!("VERSION_INFO"))]
pub struct Args {
    /// The URLs to clean before STDIN.
    ///
    /// Can contain a task context by using {"url":"https://example.com","context":{...}}
    pub urls: Vec<LazyTaskConfig>,
    /// The config file to use.
    ///
    /// Omit to use the built in default config.
    #[cfg(feature = "default-config")]
    #[arg(short      , long)]
    pub config: Option<PathBuf>,
    /// The config file to use.
    ///
    /// Required as the `default-config` feature is disabled.
    #[cfg(not(feature = "default-config"))]
    #[arg(short      , long)]
    pub config: PathBuf,
    /// The cache to use.
    ///
    /// Defaults to the value specified by the config.
    #[cfg(feature = "cache")]
    #[arg(             long)]
    pub cache_path: Option<CachePath>,
    /// JSON output.
    #[arg(short      , long)]
    pub json: bool,
    /// The ParamsDiff files to apply to the config's Params.
    #[arg(             long)]
    pub params_diff: Vec<PathBuf>,
    /// Generate a ParamsDiff from CLI arguments.
    #[command(flatten)]
    pub params_diff_args: ParamsDiffArgParser,
    /// The context to share between all Tasks.
    #[arg(             long)]
    pub job_context: Option<String>,
    /// Test files to run.
    #[arg(             long, verbatim_doc_comment)]
    pub tests: Option<Vec<PathBuf>>,
    /// Asserts the "suitability" of the loaded config.
    #[arg(             long, verbatim_doc_comment)]
    pub test_suitability: bool,
    /// The number of worker threads to use.
    ///
    /// 0 uses the CPU's thread count.
    #[arg(long, default_value_t = 0)]
    pub threads: usize,
    /// (For debug build) Only print timing information.
    #[cfg(feature = "debug")]
    #[arg(long)]
    pub debug_just_print_times: bool,
    /// Debug option to get more accurate time measurements.
    #[arg(long)]
    pub debug_no_printing: bool
}

/// The enum of errors [`main`] can return.
#[derive(Debug, Error)]
pub enum CliError {
    /// Returned when a [`GetConfigError`] is encountered.
    #[error(transparent)] GetConfigError(#[from] GetConfigError),
    /// Returned when unable to load a [`ParamsDiff`] file.
    #[error(transparent)] CantLoadParamsDiffFile(std::io::Error),
    /// Returned when unable to parse a [`ParamsDiff`] file.
    #[error(transparent)] CantParseParamsDiffFile(serde_json::Error),
    /// Returned when unable to parse the [`Args::job_context`] parameter.
    #[error(transparent)] CantParseJobContext(serde_json::Error),
    /// Returned when unable to load a [`Tests`] file.
    #[error(transparent)] CantLoadTests(io::Error),
    /// Returned when unable to parse a [`Tests`] file.
    #[error(transparent)] CantParseTests(serde_json::Error)
}

/// Helper function to convert [`str`]s into JSON strings.
fn str_to_json_str(s: &str) -> String {
    serde_json::to_string(s).expect("Serializing a string to never fail.")
}

fn main() -> Result<ExitCode, CliError> {
    let some_ok  = std::sync::Mutex::new(false);
    let some_err = std::sync::Mutex::new(false);

    let args = Args::parse();

    #[cfg(feature = "debug")]
    util::DEBUG_JUST_PRINT_TIMES.set(args.debug_just_print_times).expect("No poisoning.");

    #[cfg(feature = "default-config")]
    let mut config = Config::load_or_get_default_no_cache(args.config.as_deref())?;
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
                .error(clap::error::ErrorKind::WrongNumberOfValues, e.as_str())
                .exit()
        }
    }

    for params_diff in params_diffs {
        params_diff.apply(&mut config.params);
    }

    let job_context = if let Some(job_context_string) = args.job_context {
        serde_json::from_str(&job_context_string).map_err(CliError::CantParseJobContext)?
    } else {
        Default::default()
    };

    #[cfg(feature = "cache")]
    let cache = args.cache_path.as_ref().unwrap_or(&config.cache_path).clone().into();

    let json = args.json;

    let tests            = args.tests;
    let test_suitability = args.test_suitability;

    let no_cleaning = test_suitability || tests.is_some();

    if test_suitability {config.assert_suitability();}

    if let Some(tests) = tests {
        for test_path in tests {
            config.run_tests(serde_json::from_str::<testing::Tests>(&std::fs::read_to_string(test_path).map_err(CliError::CantLoadTests)?).map_err(CliError::CantParseTests)?);
        }
        println!("\nAll tests passed!");
    }

    if no_cleaning {std::process::exit(0);}

    let mut threads = args.threads;
    if threads == 0 {threads = std::thread::available_parallelism().expect("To be able to get the available parallelism.").into();}
    let (in_senders , in_recievers ) = (0..threads).map(|_| std::sync::mpsc::channel::<Result<LazyTask<'_>, MakeLazyTaskError>>()).collect::<(Vec<_>, Vec<_>)>();
    let (out_senders, out_recievers) = (0..threads).map(|_| std::sync::mpsc::channel::<Result<BetterUrl, DoTaskError>>()).collect::<(Vec<_>, Vec<_>)>();
    let config_ref = &config;
    let job_context_ref = &job_context;
    #[cfg(feature = "cache")]
    let cache_ref = &cache;
    std::thread::scope(|s| {

        // Task getter thread.

        std::thread::Builder::new().name("Task Getter".to_string()).spawn_scoped(s, move || {
            let job = Job {
                config : config_ref,
                context: job_context_ref,
                #[cfg(feature = "cache")]
                cache: cache_ref,
                lazy_task_configs: {
                    let ret = args.urls.into_iter().map(Ok);
                    if !io::stdin().is_terminal() {
                        Box::new(ret.chain(io::stdin().lines().map(|x| Ok(x?.into()))))
                    } else {
                        Box::new(ret)
                    }
                }
            };

            for (in_sender, task_config_string) in in_senders.iter().cycle().zip(job) {
                in_sender.send(task_config_string).expect("The in receiver to still exist.");
            }
        }).expect("Making threads to work fine.");

        // Worker threads.

        in_recievers.into_iter().zip(out_senders).enumerate().map(|(i, (ir, os))| {
            std::thread::Builder::new().name(format!("Worker {i}")).spawn_scoped(s, move || {
                while let Ok(maybe_task_source) = ir.recv() {
                    let ret = match maybe_task_source {
                        Ok(task_source) => match task_source.make() {
                            Ok(task) => task.r#do(),
                            Err(e) => Err(e.into())
                        },
                        Err(e) => Err(DoTaskError::MakeTaskError(e.into()))
                    };

                    os.send(ret).expect("The out receiver to still exist.");
                }
            }).expect("Making threads to work fine.");
        }).for_each(drop);

        let some_ok_ref  = &some_ok;
        let some_err_ref = &some_err;

        // Stdout thread.

        std::thread::Builder::new().name("Stdout".to_string()).spawn_scoped(s, move || {
            let mut some_ok_ref_lock  = some_ok_ref .lock().expect("No panics.");
            let mut some_err_ref_lock = some_err_ref.lock().expect("No panics.");

            if !args.debug_no_printing {
                if json {
                    let mut first_job = true;

                    print!("{{\"Ok\":{{\"urls\":[");
                    for or in out_recievers.iter().cycle() {
                        match or.recv() {
                            Ok(Ok(url)) => {
                                if !first_job {print!(",");}
                                print!("{{\"Ok\":{}}}", str_to_json_str(url.as_str()));
                                *some_ok_ref_lock = true;
                            },
                            Ok(Err(e)) => {
                                if !first_job {print!(",");}
                                print!("{{\"Err\":{}}}", str_to_json_str(&format!("{e:?}")));
                                *some_err_ref_lock = true;
                            },
                            Err(_) => break
                        }
                        first_job = false;
                    }

                    print!("]}}}}");
                } else {
                    for or in out_recievers.iter().cycle() {
                        match or.recv() {
                            Ok(Ok(url)) => {
                                println!("{}", url.as_str());
                                *some_ok_ref_lock = true;
                            },
                            Ok(Err(e)) => {
                                println!();
                                eprintln!("{e:?}");
                                *some_err_ref_lock = true;
                            }
                            Err(_) => break
                        }
                    }
                }
            } else {
                out_recievers.iter().cycle().find(|x| x.recv().is_err());
            }
        }).expect("Making threads to work fine.");
    });

    Ok(match (*some_ok.lock().expect("No panics."), *some_err.lock().expect("No panics.")) {
        (false, false) => 0,
        (false, true ) => 1,
        (true , false) => 0,
        (true , true ) => 2
    }.into())
}
