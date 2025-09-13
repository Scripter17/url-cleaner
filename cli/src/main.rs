//! URL Cleaner - Explicit non-consent to URL spytext.
//!
//! See [url_cleaner_engine] to integrate URL Cleaner with your own projects.

#![allow(rustdoc::bare_urls, reason = "All useless.")]

use std::path::PathBuf;
use std::io::{self, IsTerminal, BufRead};
use std::process::ExitCode;

use clap::Parser;
use thiserror::Error;

use url_cleaner_engine::types::*;
use url_cleaner_engine::glue::*;
use url_cleaner_engine::helpers::*;
use url_cleaner_engine::testing::*;

#[allow(rustdoc::bare_urls, reason = "It'd look bad in the console.")]
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
#[derive(Debug, Parser)]
struct Args {
    /// The URLs to clean before STDIN.
    ///
    /// The following are all equivalent:
    ///
    /// https://example.com
    /// "https://example.com"
    /// {"url": "https://example.com"}
    /// {"url": "https://example.com", "context": {}}
    /// {"url": "https://example.com", "context": {"vars": []}}
    ///
    /// The following also sets the TaskContext var `a` to `2`.
    ///
    /// {"url": "https://example.com", "context": {"vars": {"a": "2"}}}
    #[arg(verbatim_doc_comment)]
    urls: Vec<LazyTaskConfig<'static>>,
    /// The config file to use.
    ///
    /// Omit to use the built in default cleaner.
    #[cfg(feature = "default-cleaner")]
    #[arg(long, value_name = "PATH")]
    cleaner: Option<PathBuf>,
    /// The cleaner file to use.
    #[cfg(not(feature = "default-cleaner"))]
    #[arg(long, value_name = "PATH")]
    cleaner: PathBuf,
    /// Export the cleaner after --params-diff, --flag, etc., if specified, are applied, then exit.
    #[arg(long)]
    export_cleaner: bool,
    /// Output results as JSON.
    ///
    /// The format looks like this, but minified:
    ///
    /// {"Ok": {
    ///   "urls": [
    ///     {"Ok": "https://example.com/success"},
    ///     {"Err": "Error message"}
    ///   ]
    /// }}
    ///
    /// The surrounding `{"Ok": {...}}` is to let URL Cleaner Site return `{"Err": {...}}` on invalid input.
    #[arg(short, long, verbatim_doc_comment)]
    json: bool,
    /// The ProfilesConfig file.
    ///
    /// Cannot be used with --profiles-string.
    #[arg(long, value_name = "PATH")]
    profiles: Option<PathBuf>,
    /// The ProfilesConfig string.
    ///
    /// Cannot be used with --profiles.
    #[arg(long, value_name = "JSON STRING")]
    profiles_string: Option<String>,
    /// The Profile to use.
    ///
    /// Applied before ParamsDiffs and --flag/--var.
    #[arg(long, requires = "profiles")]
    profile: Option<String>,
    /// A standalone ParamsDiff file.
    ///
    /// Applied after Profiles and before --flag/--var.
    ///
    /// Cannot be used with --params-diff-string.
    #[arg(long, value_name = "PATH")]
    params_diff: Option<PathBuf>,
    /// A standalone ParamsDiff string.
    ///
    /// Applied after Profiles and before --flag/--var.
    ///
    /// Cannot be used with --params-diff.
    #[arg(long, value_name = "JSON STRING")]
    params_diff_string: Option<String>,
    /// Flags to insert into the params.
    ///
    /// Applied after Profiles ParamsDiff.
    #[arg(short, long)]
    flag: Vec<String>,
    /// Vars to insert into the params.
    ///
    /// Applied after Profiles ParamsDiff.
    #[arg(short, long, value_names = ["NAME", "VALUE"], num_args = 2)]
    var: Vec<Vec<String>>,
    /// The JobContext file to use.
    ///
    /// Cannot be used with --job-context-string.
    #[arg(long, value_name = "PATH")]
    job_context: Option<PathBuf>,
    /// The JobContext string to use.
    ///
    /// Cannot be used with --job-context.
    #[arg(long, value_name = "JSON STRING")]
    job_context_string: Option<String>,
    /// The path of the cache to use.
    #[cfg(feature = "cache")]
    #[arg(long, default_value = "url-cleaner-cache.sqlite", value_name = "PATH")]
    cache: CachePath,
    /// Whether or not to read from the cache. If unspecified, defaults to true.
    #[cfg(feature = "cache")]
    #[arg(long, default_value = "true", default_missing_value = "true", action = clap::ArgAction::Set, value_name = "BOOL")]
    read_cache: bool,
    /// Whether or not to write to the cache. If unspecified, defaults to true.
    #[cfg(feature = "cache")]
    #[arg(long, default_value = "true", default_missing_value = "true", action = clap::ArgAction::Set, value_name = "BOOL")]
    write_cache: bool,
    /// Artificially delay cache reads about as long as the initial run to defend against cache detection.
    #[cfg(feature = "cache")]
    #[arg(long, default_value = "false", default_missing_value = "true", action = clap::ArgAction::Set, value_name = "BOOL")]
    cache_delay: bool,
    /// If true, makes requests, cache reads, etc. effectively single threaded to hide thread count.
    #[arg(long, default_value = "false", default_missing_value = "true", action = clap::ArgAction::Set, value_name = "BOOL")]
    hide_thread_count: bool,
    /// The number of worker threads to use.
    ///
    /// Zero uses the CPU's thread count.
    #[arg(long, default_value_t = 0)]
    threads: usize,
    /// Test files to run.
    #[arg(long, value_name = "PATH")]
    tests: Vec<PathBuf>,
    /// Asserts the "suitability" of the loaded cleaner.
    #[arg(long)]
    test_suitability: bool
}

/// The enum of errors [`main`] can return.
#[derive(Debug, Error)]
pub enum CliError {
    /// Returned when a [`GetCleanerError`] is encountered.
    #[error(transparent)] GetCleanerError(#[from] GetCleanerError),
    /// Returned when unable to load a [`ParamsDiff`] file.
    #[error(transparent)] CantLoadProfilesFile(std::io::Error),
    /// Returned when unable to parse a [`ParamsDiff`] file.
    #[error(transparent)] CantParseProfilesFile(serde_json::Error),
    /// Returned when unable to parse a [`ParamsDiff`] string.
    #[error(transparent)] CantParseProfilesString(serde_json::Error),
    /// Returned when both `--profiles-config` and `--profiles-config-string` are specified.
    #[error("Can't have both --profiles-config and --profiles-config-string")]
    CantHaveBothProfilesAndProfilesString,
    /// Returned when unable to load a [`ParamsDiff`] file.
    #[error(transparent)] CantLoadParamsDiffFile(std::io::Error),
    /// Returned when unable to parse a [`ParamsDiff`] file.
    #[error(transparent)] CantParseParamsDiffFile(serde_json::Error),
    /// Returned when unable to parse a [`ParamsDiff`] string.
    #[error(transparent)] CantParseParamsDiffString(serde_json::Error),
    /// Returned when both `--params-diff` and `--params-diff-string` are specified.
    #[error("Can't have both --params-diff and --params-diff-string")]
    CantHaveBothParamsDiffAndParamsDiffString,
    /// Returned when unable to load a [`JobContext`] file.
    #[error(transparent)] CantLoadJobContextFile(std::io::Error),
    /// Returned when unable to parse a [`JobContext`] file.
    #[error(transparent)] CantParseJobContextFile(serde_json::Error),
    /// Returned when unable to parse a [`JobContext`] string.
    #[error(transparent)] CantParseJobContextString(serde_json::Error),
    /// Returned when both `--job-context` and `--job-context-string` are specified.
    #[error("Can't have both --job-context and --job-context-string")]
    CantHaveBothJobContextAndJobContextString,
    /// Returned when unable to load a [`Tests`] file.
    #[error(transparent)] CantLoadTests(io::Error),
    /// Returned when unable to parse a [`Tests`] file.
    #[error(transparent)] CantParseTests(serde_json::Error),
    /// Returned when the requested [`Profile`] isn't found.
    #[error("The requested Profile wasn't found.")]
    ProfileNotFound
}

fn main() -> Result<ExitCode, CliError> {
    let args = Args::parse();

    #[cfg(feature = "default-cleaner")]
    let mut cleaner = Cleaner::load_or_get_default_no_cache(args.cleaner.as_deref())?;
    #[cfg(not(feature = "default-cleaner"))]
    let mut cleaner = Cleaner::load_from_file(&args.cleaner)?;

    // Get and apply [`ParamsDiff`]s.

    let profiles_config = match (args.profiles, args.profiles_string) {
        (None      , None        ) => None,
        (Some(path), None        ) => Some(serde_json::from_str::<ProfilesConfig>(&std::fs::read_to_string(path).map_err(CliError::CantLoadProfilesFile)?).map_err(CliError::CantParseProfilesFile)?),
        (None      , Some(string)) => Some(serde_json::from_str::<ProfilesConfig>(&string).map_err(CliError::CantParseProfilesString)?),
        (Some(_)   , Some(_)     ) => Err(CliError::CantHaveBothProfilesAndProfilesString)?
    };

    if let Some(profiles_config) = profiles_config {
        cleaner.params = profiles_config.into_params(args.profile.as_deref(), cleaner.params).ok_or(CliError::ProfileNotFound)?;
    }

    let params_diff = match (args.params_diff, args.params_diff_string) {
        (None      , None        ) => None,
        (Some(path), None        ) => Some(serde_json::from_str::<ParamsDiff>(&std::fs::read_to_string(path).map_err(CliError::CantLoadParamsDiffFile)?).map_err(CliError::CantParseParamsDiffFile)?),
        (None      , Some(string)) => Some(serde_json::from_str::<ParamsDiff>(&string).map_err(CliError::CantParseParamsDiffString)?),
        (Some(_)   , Some(_)     ) => Err(CliError::CantHaveBothParamsDiffAndParamsDiffString)?
    };

    if let Some(params_diff) = params_diff {
        params_diff.apply_once(&mut cleaner.params);
    }

    cleaner.params.flags.to_mut().extend(args.flag);
    for var in args.var {
        let [name, value] = var.try_into().expect("The clap parser to work");
        cleaner.params.vars.to_mut().insert(name, value);
    }

    // Get the [`JobContext`].

    let job_context = match (args.job_context, args.job_context_string) {
        (None      , None        ) => Default::default(),
        (Some(path), None        ) => serde_json::from_str(&std::fs::read_to_string(path).map_err(CliError::CantLoadJobContextFile)?).map_err(CliError::CantParseJobContextFile)?,
        (None      , Some(string)) => serde_json::from_str(&string).map_err(CliError::CantParseJobContextString)?,
        (Some(_)   , Some(_)     ) => Err(CliError::CantHaveBothJobContextAndJobContextString)?
    };

    // Testing and stuff.

    let no_cleaning = args.test_suitability || !args.tests.is_empty() || args.export_cleaner;

    if args.test_suitability {
        cleaner.assert_suitability();
        println!("The chosen cleaner is suitable to be the default cleaner!");
    }

    if !args.tests.is_empty() {
        for test_path in args.tests {
            cleaner.run_tests(serde_json::from_str::<Tests>(&std::fs::read_to_string(test_path).map_err(CliError::CantLoadTests)?).map_err(CliError::CantParseTests)?);
        }
        println!("\nAll tests passed!");
    }

    if args.export_cleaner {println!("{}", serde_json::to_string(&cleaner).expect("Serializing the cleaner to always work."));}

    if no_cleaning {std::process::exit(0);}

    // Do the job.

    #[cfg(feature = "cache")]
    let cache = args.cache.into();
    #[cfg(feature = "cache")]
    let cache_handle_config = CacheHandleConfig {
        delay: args.cache_delay,
        read : args.read_cache,
        write: args.write_cache
    };
    let unthreader = Unthreader::r#if(args.hide_thread_count);

    let threads = match args.threads {
        0 => std::thread::available_parallelism().expect("To be able to get the available parallelism.").get(),
        1.. => args.threads
    };
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
                #[cfg(feature = "cache")]
                cache_handle_config,
                unthreader: &unthreader,
                lazy_task_configs: {
                    let ret = args.urls.into_iter().map(Ok);
                    if !io::stdin().is_terminal() {
                        Box::new(ret.chain(io::stdin().lock().split(b'\n').map(|x| match x {
                            Ok(mut line) => {line.pop_if(|last| *last == b'\r'); Ok(line.into())},
                            Err(e) => Err(e.into())
                        })))
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
                            println!("{url}");
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
