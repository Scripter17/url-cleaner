//! URL Cleaner - Explicit non-consent to URL spytext.

use std::path::PathBuf;
use std::io::{self, IsTerminal};
use std::process::ExitCode;
use std::num::NonZero;

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
#[cfg_attr(feature = "default-cleaner", doc = "default-cleaner")]
#[cfg_attr(feature = "regex"          , doc = "regex"          )]
#[cfg_attr(feature = "http"           , doc = "http"           )]
#[cfg_attr(feature = "cache"          , doc = "cache"          )]
#[cfg_attr(feature = "base64"         , doc = "base64"         )]
#[cfg_attr(feature = "commands"       , doc = "commands"       )]
#[cfg_attr(feature = "custom"         , doc = "custom"         )]
#[cfg_attr(feature = "debug"          , doc = "debug"          )]
///
/// Disabled features:
#[cfg_attr(not(feature = "default-cleaner"), doc = "default-cleaner")]
#[cfg_attr(not(feature = "regex"          ), doc = "regex"          )]
#[cfg_attr(not(feature = "http"           ), doc = "http"           )]
#[cfg_attr(not(feature = "cache"          ), doc = "cache"          )]
#[cfg_attr(not(feature = "base64"         ), doc = "base64"         )]
#[cfg_attr(not(feature = "commands"       ), doc = "commands"       )]
#[cfg_attr(not(feature = "custom"         ), doc = "custom"         )]
#[cfg_attr(not(feature = "debug"          ), doc = "debug"          )]
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(version = env!("VERSION_INFO"))]
pub struct Args {
    /// The URLs to clean before STDIN.
    /// Can contain a task context by using {"url":"https://example.com","context":{...}}
    #[arg(verbatim_doc_comment)]
    pub urls: Vec<LazyTaskConfig>,
    /// The config file to use.
    /// Omit to use the built in default cleaner.
    #[cfg(feature = "default-cleaner")]
    #[arg(short, long, verbatim_doc_comment)]
    pub cleaner: Option<PathBuf>,
    /// The cleaner file to use.
    /// Required as the `default-cleaner` feature is disabled.
    #[cfg(not(feature = "default-cleaner"))]
    #[arg(short, long, verbatim_doc_comment)]
    pub cleaner: PathBuf,
    /// The cache to use.
    /// Defaults to the value specified by the cleaner.
    #[cfg(feature = "cache")]
    #[arg(long, verbatim_doc_comment)]
    pub cache_path: Option<CachePath>,
    /// Output results as JSON.
    ///
    /// The format looks like this, but minified:
    /// 
    /// {"Ok": {
    ///   "urls": [
    ///     {"Ok": "https://example.com/success"},
    ///     {"Err": "https://example.com/failure"}
    ///   ]
    /// }}
    /// 
    /// The surrounding `{"Ok": {...}}` is to let URL Cleaner Site return `{"Err": {...}}` on invalid input.
    #[arg(short, long, verbatim_doc_comment)]
    pub json: bool,
    /// The ParamsDiff files to apply to the cleaner's Params.
    #[arg(long, verbatim_doc_comment)]
    pub params_diff: Vec<PathBuf>,
    /// Generate a ParamsDiff from CLI arguments.
    #[command(flatten)]
    pub params_diff_args: ParamsDiffArgParser,
    /// The context to share between all Tasks.
    #[arg(long, verbatim_doc_comment)]
    pub job_context: Option<String>,
    /// The number of worker threads to use.
    /// Omit to use the current CPU's thread count.
    #[arg(long, verbatim_doc_comment)]
    pub threads: Option<NonZero<usize>>,
    /// Test files to run.
    #[arg(long, verbatim_doc_comment)]
    pub tests: Vec<PathBuf>,
    /// Asserts the "suitability" of the loaded cleaner.
    #[arg(long, verbatim_doc_comment)]
    pub test_suitability: bool,
    /// Print the cleaner after all ParamsDiffs are applied.
    /// Exact output isn't stable due to HashSets/HashMaps having a random order.
    #[arg(long, verbatim_doc_comment)]
    pub export_cleaner: bool
}

/// The enum of errors [`main`] can return.
#[derive(Debug, Error)]
pub enum CliError {
    /// Returned when a [`GetCleanerError`] is encountered.
    #[error(transparent)] GetCleanerError(#[from] GetCleanerError),
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

fn main() -> Result<ExitCode, CliError> {
    let args = Args::parse();

    #[cfg(feature = "default-cleaner")]
    let mut cleaner = Cleaner::load_or_get_default_no_cache(args.cleaner.as_deref())?;
    #[cfg(not(feature = "default-cleaner"))]
    let mut cleaner = Cleaner::load_from_file(&args.cleaner)?;

    // Get and apply [`ParamsDiff`]s.

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
        params_diff.apply(&mut cleaner.params);
    }

    // Get the [`JobContext`].

    let job_context = if let Some(job_context_string) = args.job_context {
        serde_json::from_str(&job_context_string).map_err(CliError::CantParseJobContext)?
    } else {
        Default::default()
    };

    // Testing and stuff.

    let no_cleaning = args.test_suitability || !args.tests.is_empty() || args.export_cleaner;

    if args.test_suitability {
        cleaner.assert_suitability();
        println!("The chosen cleaner is suitable to be the default cleaner!");
    }

    if !args.tests.is_empty() {
        for test_path in args.tests {
            cleaner.run_tests(serde_json::from_str::<testing::Tests>(&std::fs::read_to_string(test_path).map_err(CliError::CantLoadTests)?).map_err(CliError::CantParseTests)?);
        }
        println!("\nAll tests passed!");
    }

    if args.export_cleaner {println!("{}", serde_json::to_string(&cleaner).expect("Serializing the cleaner to always work."));}

    if no_cleaning {std::process::exit(0);}

    // Do the job.

    #[cfg(feature = "cache")]
    let cache = args.cache_path.as_ref().unwrap_or(&cleaner.cache_path).clone().into();

    let threads = args.threads.unwrap_or_else(|| std::thread::available_parallelism().expect("To be able to get the available parallelism.")).get();
    let (in_senders , in_recievers ) = (0..threads).map(|_| std::sync::mpsc::channel::<Result<LazyTask<'_>, MakeLazyTaskError>>()).collect::<(Vec<_>, Vec<_>)>();
    let (out_senders, out_recievers) = (0..threads).map(|_| std::sync::mpsc::channel::<Result<BetterUrl, DoTaskError>>()).collect::<(Vec<_>, Vec<_>)>();

    let some_ok  = std::sync::Mutex::new(false);
    let some_err = std::sync::Mutex::new(false);

    std::thread::scope(|s| {

        // Task getter thread.

        std::thread::Builder::new().name("Task Getter".to_string()).spawn_scoped(s, || {
            let job = Job {
                cleaner: &cleaner,
                context: &job_context,
                #[cfg(feature = "cache")]
                cache  : &cache,
                lazy_task_configs: {
                    let ret = args.urls.into_iter().map(Ok);
                    if !io::stdin().is_terminal() {
                        Box::new(ret.chain(io::stdin().lines().map(|x| Ok(x?.into()))))
                    } else {
                        Box::new(ret)
                    }
                }
            };

            for (in_sender, task_config_string) in {in_senders}.iter().cycle().zip(job) {
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

        // Stdout thread.

        std::thread::Builder::new().name("Stdout".to_string()).spawn_scoped(s, || {
            let mut some_ok_lock  = some_ok .lock().expect("No panics.");
            let mut some_err_lock = some_err.lock().expect("No panics.");

            if args.json {
                let mut first_job = true;

                print!("{{\"Ok\":{{\"urls\":[");

                for or in {out_recievers}.iter().cycle() {
                    match or.recv() {
                        Ok(Ok(url)) => {
                            if !first_job {print!(",");}
                            print!("{{\"Ok\":{}}}", serde_json::to_string(url.as_str()).expect("Serializing a string to never fail."));
                            *some_ok_lock = true;
                        },
                        Ok(Err(e)) => {
                            if !first_job {print!(",");}
                            print!("{{\"Err\":{}}}", serde_json::to_string(&format!("{e:?}")).expect("Serializing a string to never fail."));
                            *some_err_lock = true;
                        },
                        Err(_) => break
                    }
                    first_job = false;
                }

                print!("]}}}}");
            } else {
                for or in {out_recievers}.iter().cycle() {
                    match or.recv() {
                        Ok(Ok(url)) => {
                            println!("{}", url.as_str());
                            *some_ok_lock = true;
                        },
                        Ok(Err(e)) => {
                            println!();
                            eprintln!("{e:?}");
                            *some_err_lock = true;
                        }
                        Err(_) => break
                    }
                }
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
