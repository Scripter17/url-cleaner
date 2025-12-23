//! URL Cleaner CLI - Explicit non-consent to URL spytext.
//!
//! See [url_cleaner_engine] to integrate URL Cleaner with your own projects.

use std::path::PathBuf;
use std::io::{IsTerminal, BufRead, Write};
use std::fmt::Debug;
use std::time::Duration;
use std::sync::mpsc::RecvTimeoutError;

use clap::Parser;
use thiserror::Error;

use url_cleaner_engine::prelude::*;

#[allow(rustdoc::bare_urls, reason = "It'd look bad in the console.")]
/// URL Cleaner CLI - Explicit non-consent to URL spytext.
/// Licensed under the Aferro GNU Public License version 3.0 or later.
/// https://github.com/Scripter17/url-cleaner
///
/// Enabled features:
#[cfg_attr(feature = "bundled-cleaner", doc = "bundled-cleaner")]
#[cfg_attr(feature = "http"           , doc = "http"           )]
#[cfg_attr(feature = "cache"          , doc = "cache"          )]
/// 
/// Disabled features:
#[cfg_attr(not(feature = "bundled-cleaner"), doc = "bundled-cleaner")]
#[cfg_attr(not(feature = "http"           ), doc = "http"           )]
#[cfg_attr(not(feature = "cache"          ), doc = "cache"          )]
#[derive(Debug, Parser)]
struct Args {
    /// The tasks to clean before STDIN.
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
    tasks: Vec<String>,

    /// The cleaner file to use.
    /// Omit to use the built in bundled cleaner.
    #[cfg(feature = "bundled-cleaner")]
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    cleaner: Option<PathBuf>,
    /// The cleaner file to use.
    #[cfg(not(feature = "bundled-cleaner"))]
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    cleaner: PathBuf,

    /// The ProfilesConfig file.
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    profiles: Option<PathBuf>,
    /// The Profile to use.
    /// Applied before ParamsDiffs and --flag/--var.
    #[arg(long, verbatim_doc_comment)]
    profile: Option<String>,

    /// A standalone ParamsDiff file.
    /// Applied after Profiles and before --flag/--var.
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    params_diff: Option<PathBuf>,

    /// Flags to insert into the params.
    /// Applied after Profiles and ParamsDiff.
    #[arg(short, long, verbatim_doc_comment)]
    flag: Vec<String>,
    /// Vars to insert into the params.
    /// Applied after Profiles and ParamsDiff.
    #[arg(short, long, verbatim_doc_comment, value_names = ["NAME", "VALUE"], num_args = 2)]
    var: Vec<Vec<String>>,

    /// The JobContext.
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    job_context: Option<PathBuf>,

    /// The path of the cache to use.
    #[cfg(feature = "cache")]
    #[arg(long, verbatim_doc_comment, default_value = "url-cleaner-cache.sqlite", value_name = "PATH")]
    cache: PathBuf,
    /// Disables reading from the cache.
    /// Useful for overwriting stale entries.
    #[cfg(feature = "cache")]
    #[arg(long, verbatim_doc_comment)]
    no_read_cache: bool,
    /// Disables writing to the cache.
    /// Useful for not leaving records.
    #[cfg(feature = "cache")]
    #[arg(long, verbatim_doc_comment)]
    no_write_cache: bool,
    /// Make cache reads wait about as long as the cached operation originally took.
    /// Useful for not leaking what is and is not in the cache.
    #[cfg(feature = "cache")]
    #[arg(long, verbatim_doc_comment)]
    cache_delay: bool,

    /// Make HTTP requests and cache reads happen one after another instead of in parallel.
    /// Useful for not leaking the thread count.
    #[arg(long, verbatim_doc_comment)]
    unthread: bool,
    /// The number of worker threads to use.
    /// Zero uses the CPU's thread count.
    /// So on a 2 core 4 thread system it uses 4 threads.
    #[arg(long, verbatim_doc_comment, default_value_t = 0)]
    threads: usize,

    /// If set, check that the loaded Cleaner (+ParamsDiff and ProfilesConfig and whatnot) is "suitable" to be the Bundled Cleaner.
    /// If it is, exit without doing anything else. If it isn't panic with a message.
    /// Used for intenal testing; Exact details are unstable.
    #[arg(long, verbatim_doc_comment)]
    assert_suitability: bool
}

/// The enum of errors [`main`] can return.
#[derive(Debug, Error)]
pub enum CliError {
    /// Returned when a [`GetCleanerError`] is encountered.
    #[error(transparent)] GetCleanerError(#[from] GetCleanerError),
    /// Returned when a [`GetParamsDiffError`] is encountered.
    #[error(transparent)] GetParamsDiffError(#[from] GetParamsDiffError),
    /// Returned when a [`GetProfilesConfigError`] is encountered.
    #[error(transparent)] GetProfilesConfigError(#[from] GetProfilesConfigError),
    /// Returned when a [`GetJobContextError`] is encountered.
    #[error(transparent)] GetJobContextError(#[from] GetJobContextError),
    /// Returned when the requested [`Profile`] isn't found.
    #[error("The requested Profile wasn't found.")]
    ProfileNotFound
}

fn main() -> Result<(), CliError> {
    let args = Args::parse();

    #[cfg(feature = "bundled-cleaner")]
    let mut cleaner = Cleaner::load_or_get_bundled_no_cache(args.cleaner)?;
    #[cfg(not(feature = "bundled-cleaner"))]
    let mut cleaner = Cleaner::load_from_file(args.cleaner)?;

    let profiles_config = args.profiles   .map(ProfilesConfig::load_from_file).transpose()?;
    let params_diff     = args.params_diff.map(ParamsDiff    ::load_from_file).transpose()?;
    let job_context     = args.job_context.map(JobContext    ::load_from_file).transpose()?.unwrap_or_default();

    if args.assert_suitability {
        println!("Asserting the suitability of the loaded Cleaner, ProfilesConfig, and ParamsDiffs.");
        
        let profiled_cleaner = ProfiledCleanerConfig {
            cleaner,
            profiles_config: profiles_config.unwrap_or_default()
        }.make();

        let extra_params_diff = match (&*args.flag, &*args.var) {
            ([], []) => None,
            _ => Some(ParamsDiff {
                flags: args.flag.into_iter().collect(),
                #[allow(clippy::indexing_slicing, reason = "Clap should ensure there's exactly 2.")]
                vars: args.var.into_iter().map(|kv| <(_, _)>::from(<[String; 2]>::try_from(kv).expect("Clap to work."))).collect(),
                ..Default::default()
            })
        };

        for (name, mut cleaner) in profiled_cleaner.get_each() {
            println!("  Testing profile {name:?}");

            cleaner.assert_suitability();

            if let Some(params_diff) = params_diff.clone() {
                println!("    Testing with --params-diff");
                params_diff.apply(&mut cleaner.params);
                cleaner.assert_suitability();
            }

            if let Some(extra_params_diff) = extra_params_diff.clone() {
                println!("    Testing with --flag, --var, etc.");
                extra_params_diff.apply(&mut cleaner.params);
                cleaner.assert_suitability();
            }
        }

        return Ok(());
    }

    if let Some(profiles_config) = profiles_config {
        cleaner = ProfiledCleanerConfig {
            cleaner,
            profiles_config
        }.into_profile(args.profile.as_deref()).ok_or(CliError::ProfileNotFound)?;
    }

    if let Some(params_diff) = params_diff {
        params_diff.apply(&mut cleaner.params);
    }

    cleaner.params.flags.to_mut().extend(args.flag);
    for var in args.var {
        let [name, value] = var.try_into().expect("The clap parser to work");
        cleaner.params.vars.to_mut().insert(name, value);
    }

    // Do the job.

    // The general idea is:
    // 1. The getter thread, if needed, make a new buffer.
    // 2. Write a line of STDIN to that buffer.
    // 3. Send that buffer to a worker thread.
    // 4. The worker thread makes a Task.
    // 5. The worker thread returns the buffer to the task getter thread.
    // 6. *Then* the worker thread does the Task.
    // 7. The worker thread sends the Task's result to the output thread.

    let threads = match args.threads {
        0 => std::thread::available_parallelism().expect("To be able to get the available parallelism.").get(),
        1.. => args.threads
    };

    let (in_senders    , in_recievers    ) = (0..threads).map(|_| std::sync::mpsc::channel::<Result<Vec<u8>, std::io::Error>>()).collect::<(Vec<_>, Vec<_>)>();
    let (buf_ret_sender, buf_ret_reciever) = std::sync::mpsc::channel::<Vec<u8>>();
    let (out_senders   , out_recievers   ) = (0..threads).map(|_| std::sync::mpsc::channel::<Box<str>>()).collect::<(Vec<_>, Vec<_>)>();

    let job = &Job {
        context: job_context,
        cleaner,
        unthreader: &Unthreader::r#if(args.unthread),
        #[cfg(feature = "cache")]
        cache: Cache {
            inner: &args.cache.into(),
            config: CacheConfig {
                read : !args.no_read_cache,
                write: !args.no_write_cache,
                delay:  args.cache_delay,
            }
        },
        #[cfg(feature = "http")]
        http_client: &HttpClient::new()
    };

    std::thread::scope(|s| {

        // Task getter thread.

        let brs = buf_ret_sender.clone();
        std::thread::Builder::new().name("LazyTaskConfig Getter".to_string()).spawn_scoped(s, move || {
            let mut buffer_count = args.tasks.len() as u64;
            let mut in_senders = in_senders.iter().cycle();

            for (url, is) in args.tasks.into_iter().zip(&mut in_senders) {
                is.send(Ok(url.into())).expect("The current buffer in reciever to stay open while needed.");
            }

            let stdin = std::io::stdin();

            if !stdin.is_terminal() {
                let mut stdin = stdin.lock();

                for is in in_senders {
                    // If there are no buffers available within the ratelimit, makea a new one.
                    // The ratelimit can reduce memory usage by up to 10x with, if properly tuned, minimal performance impact.

                    let mut buf = match buf_ret_reciever.recv_timeout(Duration::from_nanos(buffer_count / 8)) {
                        Ok(buf) => buf,
                        Err(RecvTimeoutError::Timeout) => {
                            buffer_count += 1;
                            Vec::new()
                        },
                        Err(RecvTimeoutError::Disconnected) => panic!("Expected buffer return senders to stay open while needed.")
                    };

                    buf.clear();

                    match stdin.read_until(b'\n', &mut buf) {
                        Ok (0) => break,
                        Ok (_) => {
                            if buf.ends_with(b"\n") {
                                buf.pop();
                                if buf.ends_with(b"\r") {
                                    buf.pop();
                                }
                            }
                            if buf.is_empty() {
                                continue;
                            }
                            is.send(Ok(buf)).expect("The current buffer in reciever to stay open while needed.")
                        },
                        Err(e) => {
                            brs.send(buf).expect("The buffer return channel to be open.");
                            is.send(Err(e)).expect("The current buffer in reciever to stay open while needed.")
                        }
                    }
                }
            }
        }).expect("Making threads to work fine.");

        // Worker threads.

        for (i, (bir, os)) in in_recievers.into_iter().zip(out_senders).enumerate() {
            let brs = buf_ret_sender.clone();
            std::thread::Builder::new().name(format!("Worker {i}")).spawn_scoped(s, move || {
                while let Ok(x) = bir.recv() {
                    let ret = match x {
                        Ok(buf) => {
                            let task = (&buf).make_task();
                            let _ = brs.send(buf); // The buffer return reciever will hang up when there's no more tasks to do, so this returning Err is expected.
                            match job.r#do(task) {
                                Ok(x) => x.into(),
                                Err(e) => format!("-{e:?}")
                            }
                        },
                        Err(e) => format!("-{e:?}")
                    };

                    os.send(ret.into_boxed_str()).expect("The result out reciever to stay open while needed.");
                }
            }).expect("Making threads to work fine.");
        }

        // Stdout stuff.

        let mut stdout = std::io::stdout().lock();

        let mut buffer = String::new();

        for or in {out_recievers}.iter().cycle() {
            match or.recv_timeout(std::time::Duration::from_nanos(10)) {
                Ok(x) => {
                    if buffer.len() + x.len() < 65536 {
                        if !buffer.is_empty() {
                            buffer.push('\n');
                        }
                        buffer.push_str(&x);
                    } else {
                        if !buffer.is_empty() {
                            writeln!(stdout, "{buffer}").expect("Writing to STDOUT to never fail.");
                            buffer = String::new();
                        }
                        writeln!(stdout, "{x}").expect("Writing to STDOUT to never fail.");
                    }
                },
                Err(RecvTimeoutError::Timeout) => {
                    if !buffer.is_empty() {
                        writeln!(stdout, "{buffer}").expect("Writing to STDOUT to never fail.");
                        buffer = String::new();
                    }
                    match or.recv() {
                        Ok(x) => writeln!(stdout, "{x}").expect("Writing to STDOUT to never fail."),
                        Err(_) => break
                    }
                },
                Err(RecvTimeoutError::Disconnected) => {
                    if !buffer.is_empty() {
                        writeln!(stdout, "{buffer}").expect("Writing to STDOUT to never fail.");
                    }
                    break
                }
            }
        }
    });

    Ok(())
}
