

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
mod testing;
mod util;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[cfg_attr(feature = "default-config"     , doc = "default-config")]
#[cfg_attr(feature = "regex"              , doc = "regex"         )]
#[cfg_attr(feature = "glob"               , doc = "glob"          )]
#[cfg_attr(feature = "http"               , doc = "http"          )]
#[cfg_attr(feature = "cache"              , doc = "cache"         )]
#[cfg_attr(feature = "base64"             , doc = "base64"        )]
#[cfg_attr(feature = "commands"           , doc = "commands"      )]
#[cfg_attr(feature = "custom"             , doc = "custom"        )]
#[cfg_attr(feature = "debug"              , doc = "debug"         )]
#[cfg_attr(not(feature = "default-config"), doc = "default-config")]
#[cfg_attr(not(feature = "regex"         ), doc = "regex"         )]
#[cfg_attr(not(feature = "glob"          ), doc = "glob"          )]
#[cfg_attr(not(feature = "http"          ), doc = "http"          )]
#[cfg_attr(not(feature = "cache"         ), doc = "cache"         )]
#[cfg_attr(not(feature = "base64"        ), doc = "base64"        )]
#[cfg_attr(not(feature = "commands"      ), doc = "commands"      )]
#[cfg_attr(not(feature = "custom"        ), doc = "custom"        )]
#[cfg_attr(not(feature = "debug"         ), doc = "debug"         )]
pub struct Args {
    pub urls: Vec<String>,
    #[cfg(feature = "default-config")]
    #[arg(short      , long)]
    pub config: Option<PathBuf>,
    #[cfg(not(feature = "default-config"))]
    #[arg(short      , long)]
    pub config: PathBuf,
    #[cfg(feature = "cache")]
    #[arg(             long)]
    pub cache_path: Option<CachePath>,
    #[arg(short      , long)]
    pub json: bool,
    #[arg(             long)]
    pub params_diff: Vec<PathBuf>,
    #[command(flatten)]
    pub params_diff_args: ParamsDiffArgParser,
    #[arg(             long)]
    pub jobs_context: Option<String>,
    #[arg(             long, verbatim_doc_comment)]
    pub print_args: bool,
    #[arg(             long, verbatim_doc_comment)]
    pub print_params_diffs: bool,
    #[arg(             long, verbatim_doc_comment)]
    pub print_params: bool,
    #[arg(             long, verbatim_doc_comment)]
    pub print_config: bool,
    #[arg(             long, verbatim_doc_comment)]
    pub tests: Option<Vec<PathBuf>>,
    #[arg(             long, verbatim_doc_comment)]
    pub test_suitability: bool,
    #[arg(long, default_value_t = 0)]
    pub threads: usize,
    #[cfg(feature = "debug")]
    #[arg(long)]
    pub debug_just_print_times: bool
}
#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)] GetConfigError(#[from] GetConfigError),
    #[error(transparent)] CantLoadParamsDiffFile(std::io::Error),
    #[error(transparent)] CantParseParamsDiffFile(serde_json::Error),
    #[error(transparent)] CantParseJobsContext(serde_json::Error),
    #[error(transparent)] SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)] CantLoadTests(io::Error),
    #[error(transparent)] CantParseTests(serde_json::Error)
}
fn str_to_json_str(s: &str) -> String {
    serde_json::to_string(s).expect("Serializing a string to never fail.")
}

fn main() -> Result<ExitCode, CliError> {
    let some_ok  = std::sync::Mutex::new(false);
    let some_err = std::sync::Mutex::new(false);

    let args = Args::parse();

    #[cfg(feature = "debug")]
    util::DEBUG_JUST_PRINT_TIMES.set(args.debug_just_print_times).expect("No poisoning.");

    let print_args = args.print_args;
    if print_args {println!("{args:?}");}

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

    let print_params_diffs = args.print_params_diffs;
    if print_params_diffs {println!("{}", serde_json::to_string(&params_diffs)?);}

    for params_diff in params_diffs {
        params_diff.apply(&mut config.params);
    }

    let json = args.json;

    let print_params     = args.print_params;
    let print_config     = args.print_config;
    let tests            = args.tests;
    let test_suitability = args.test_suitability;

    let no_cleaning = print_args || print_params_diffs || print_params || print_config || test_suitability || tests.is_some();

    if print_params {println!("{}", serde_json::to_string(&config.params)?);}
    if print_config {println!("{}", serde_json::to_string(&config)?);}
    if test_suitability {config.assert_suitability()}
    if let Some(tests) = tests {
        for test_path in tests {
            config.run_tests(serde_json::from_str::<testing::Tests>(&std::fs::read_to_string(test_path).map_err(CliError::CantLoadTests)?).map_err(CliError::CantParseTests)?);
        }
        println!("\nAll tests passed!");
    }

    if no_cleaning {std::process::exit(0);}

    let mut threads = args.threads;
    if threads == 0 {threads = std::thread::available_parallelism().expect("To be able to get the available parallelism.").into();}
    let (in_senders , in_recievers ) = (0..threads).map(|_| std::sync::mpsc::channel::<Result<String, io::Error>>()).collect::<(Vec<_>, Vec<_>)>();
    let (out_senders, out_recievers) = (0..threads).map(|_| std::sync::mpsc::channel::<Result<Result<BetterUrl, DoJobError>, MakeJobError>>()).collect::<(Vec<_>, Vec<_>)>();

    let jobs_config = JobsConfig {
        #[cfg(feature = "cache")]
        cache: args.cache_path.as_ref().unwrap_or(&config.cache_path).clone().into(),
        config: Cow::Owned(config)
    };
    let jobs_config_ref = &jobs_config;
    let jobs_context = if let Some(jobs_context_string) = args.jobs_context {
        serde_json::from_str(&jobs_context_string).map_err(CliError::CantParseJobsContext)?
    } else {
        Default::default()
    };
    let jobs_context_ref = &jobs_context;

    std::thread::scope(|s| {
        std::thread::Builder::new().name("Job Getter".to_string()).spawn_scoped(s, move || {
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
                in_senders.get(i % threads).expect("The amount of senders to not exceed the count of senders to make.").send(job_config_string).expect("To successfully send the Job.");
            }
        }).expect("Making threads to work fine.");

        in_recievers.into_iter().zip(out_senders).enumerate().map(|(i, (ir, os))| {
            std::thread::Builder::new().name(format!("Worker {i}")).spawn_scoped(s, move || {
                while let Ok(maybe_job_config_string) = ir.recv() {
                    let ret = match maybe_job_config_string {
                        Ok(job_config_string) => match JobConfig::from_str(&job_config_string) {
                            Ok(job_config) => Ok(jobs_config_ref.new_job(job_config, jobs_context_ref).r#do()),
                            Err(e) => Err(MakeJobError::MakeJobConfigError(e))
                        },
                        Err(e) => Err(MakeJobError::MakeJobConfigError(MakeJobConfigError::IoError(e)))
                    };

                    os.send(ret).expect("The receiver to still exist.");
                }
            }).expect("Making threads to work fine.");
        }).for_each(drop);

        let some_ok_ref  = &some_ok;
        let some_err_ref = &some_err;

        std::thread::Builder::new().name("Stdout".to_string()).spawn_scoped(s, move || {
            let mut disconnected = 0usize;
            let mut some_ok_ref_lock  = some_ok_ref .lock().expect("No panics.");
            let mut some_err_ref_lock = some_err_ref.lock().expect("No panics.");

            if json {
                let mut first_job = true;

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
                            *some_err_ref_lock = true;
                            first_job = false;
                        },
                        Ok(Err(e)) => {
                            if !first_job {print!(",");}
                            print!("{{\"Err\":{{\"message\":{},\"variant\":{}}}}}", str_to_json_str(&e.to_string()), str_to_json_str(&format!("{e:?}")));
                            *some_err_ref_lock = true;
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
            } else {
                for or in out_recievers.iter().cycle() {
                    match or.recv() {
                        Ok(Ok(Ok(url))) => {
                            println!("{}", url.as_str());
                            *some_ok_ref_lock = true;
                        },
                        Ok(Ok(Err(e))) => {
                            println!();
                            eprintln!("DoJobError\t{e:?}");
                            *some_err_ref_lock = true;
                        }
                        Ok(Err(e)) => {
                            println!();
                            eprintln!("MakeJobError\t{e:?}");
                            *some_err_ref_lock = true;
                        }
                        Err(_) => {
                            #[allow(clippy::arithmetic_side_effects, reason = "Can't even come close to usize::MAX threads and this is capped by thread count.")]
                            {disconnected += 1;}
                            if disconnected == threads {break;}
                        }
                    }
                }
            }
        }).expect("Making threads to work fine.");
    });

    return Ok(match (*some_ok.lock().expect("No panics."), *some_err.lock().expect("No panics.")) {
        (false, false) => 0,
        (false, true ) => 1,
        (true , false) => 0,
        (true , true ) => 2
    }.into());
}
