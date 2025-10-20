//! URL Cleaner CLI - Explicit non-consent to URL spytext.
//!
//! See [url_cleaner_engine] to integrate URL Cleaner with your own projects.

#![allow(rustdoc::bare_urls, reason = "All useless.")]

use std::path::PathBuf;
use std::io::{self, IsTerminal, BufRead, Write, BufWriter};
use std::process::ExitCode;
use std::fmt::Debug;
use std::time::Duration;
use std::sync::mpsc::RecvTimeoutError;

use clap::Parser;
use thiserror::Error;
use serde::{Serialize, ser::Serializer};

use url_cleaner_engine::prelude::*;
use url_cleaner_engine::testing::*;

#[allow(rustdoc::bare_urls, reason = "It'd look bad in the console.")]
/// URL Cleaner CLI - Explicit non-consent to URL spytext.
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
#[cfg_attr(feature = "command"        , doc = "command"        )]
#[cfg_attr(feature = "custom"         , doc = "custom"         )]
#[cfg_attr(feature = "debug"          , doc = "debug"          )]
///
/// Disabled features:
#[cfg_attr(not(feature = "default-cleaner"), doc = "default-cleaner")]
#[cfg_attr(not(feature = "regex"          ), doc = "regex"          )]
#[cfg_attr(not(feature = "http"           ), doc = "http"           )]
#[cfg_attr(not(feature = "cache"          ), doc = "cache"          )]
#[cfg_attr(not(feature = "base64"         ), doc = "base64"         )]
#[cfg_attr(not(feature = "command"        ), doc = "command"        )]
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
    urls: Vec<String>,
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
    /// Always buffer output.
    ///
    /// By default, output is flushed after each line if and only if STDOUT is a tty.
    #[arg(long, conflicts_with = "unbuffer_output")]
    buffer_output: bool,
    /// Never buffer output.
    ///
    /// By default, output is flushed after each line if and only if STDOUT is a tty.
    #[arg(long, conflicts_with = "buffer_output")]
    unbuffer_output: bool,
    /// The ProfilesConfig file.
    ///
    /// Cannot be used with --profiles-string.
    #[arg(long, conflicts_with = "profiles_string", value_name = "PATH")]
    profiles: Option<PathBuf>,
    /// The ProfilesConfig string.
    ///
    /// Cannot be used with --profiles.
    #[arg(long, conflicts_with = "profiles", value_name = "JSON STRING")]
    profiles_string: Option<String>,
    /// The Profile to use.
    ///
    /// Applied before ParamsDiffs and --flag/--var.
    #[arg(long)]
    profile: Option<String>,
    /// A standalone ParamsDiff file.
    ///
    /// Applied after Profiles and before --flag/--var.
    ///
    /// Cannot be used with --params-diff-string.
    #[arg(long, conflicts_with = "params_diff_string", value_name = "PATH")]
    params_diff: Option<PathBuf>,
    /// A standalone ParamsDiff string.
    ///
    /// Applied after Profiles and before --flag/--var.
    ///
    /// Cannot be used with --params-diff.
    #[arg(long, conflicts_with = "params_diff", value_name = "JSON STRING")]
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
    #[arg(long, conflicts_with = "job_context_string", value_name = "PATH")]
    job_context: Option<PathBuf>,
    /// The JobContext string to use.
    ///
    /// Cannot be used with --job-context.
    #[arg(long, conflicts_with = "job_context", value_name = "JSON STRING")]
    job_context_string: Option<String>,
    /// The proxy to use for HTTP/HTTPS requests.
    ///
    /// Overrided by --http-proxy and --https-proxy.
    #[cfg(feature = "http")]
    #[arg(long)]
    proxy: Option<HttpProxyConfig>,
    /// the proxy to use for HTTP requests.
    ///
    /// Overrides --proxy.
    #[cfg(feature = "http")]
    #[arg(long)]
    http_proxy: Option<HttpProxyConfig>,
    /// the proxy to use for HTTPS requests.
    ///
    /// Overrides --proxy.
    #[cfg(feature = "http")]
    #[arg(long)]
    https_proxy: Option<HttpProxyConfig>,
    /// The path of the cache to use.
    #[cfg(feature = "cache")]
    #[arg(long, default_value = "url-cleaner-cache.sqlite", value_name = "PATH")]
    cache: CachePath,
    /// Whether or not to read from the cache. If unspecified, defaults to true.
    #[cfg(feature = "cache")]
    #[arg(long)]
    no_read_cache: bool,
    /// Whether or not to write to the cache. If unspecified, defaults to true.
    #[cfg(feature = "cache")]
    #[arg(long)]
    no_write_cache: bool,
    /// Artificially delay cache reads about as long as the initial run to defend against cache detection.
    #[cfg(feature = "cache")]
    #[arg(long)]
    cache_delay: bool,
    /// Makes network requests, cache reads, etc. effectively single threaded while keeping most of the speed boost from multithreading.
    #[arg(long)]
    unthread: bool,
    /// When used with --unthread, also ratelimit unthreaded things.
    #[arg(long, requires = "unthread", value_name = "SECONDS")]
    unthread_ratelimit: Option<f64>,
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
    /// Returned when unable to load a [`ParamsDiff`] file.
    #[error(transparent)] CantLoadParamsDiffFile(std::io::Error),
    /// Returned when unable to parse a [`ParamsDiff`] file.
    #[error(transparent)] CantParseParamsDiffFile(serde_json::Error),
    /// Returned when unable to parse a [`ParamsDiff`] string.
    #[error(transparent)] CantParseParamsDiffString(serde_json::Error),
    /// Returned when unable to load a [`JobContext`] file.
    #[error(transparent)] CantLoadJobContextFile(std::io::Error),
    /// Returned when unable to parse a [`JobContext`] file.
    #[error(transparent)] CantParseJobContextFile(serde_json::Error),
    /// Returned when unable to parse a [`JobContext`] string.
    #[error(transparent)] CantParseJobContextString(serde_json::Error),
    /// Returned when unable to load a [`Tests`] file.
    #[error(transparent)] CantLoadTests(io::Error),
    /// Returned when unable to parse a [`Tests`] file.
    #[error(transparent)] CantParseTests(serde_json::Error),
    /// Returned when the requested [`Profile`] isn't found.
    #[error("The requested Profile wasn't found.")]
    ProfileNotFound,
    /// Returned when a [`MakeHttpProxyError`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)] MakeHttpProxyError(#[from] MakeHttpProxyError)
}

/// Helper type to print task errors to JSON output.
struct ErrorToSerdeString<E: Debug>(E);

impl<E: Debug> Serialize for ErrorToSerdeString<E> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        format!("{:?}", self.0).serialize(serializer)
    }
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
        (Some(_)   , Some(_)     ) => unreachable!()
    };

    if let Some(profiles_config) = profiles_config {
        cleaner = ProfiledCleanerConfig {
            cleaner,
            profiles_config
        }.into_profile(args.profile.as_deref()).ok_or(CliError::ProfileNotFound)?;
    }

    let params_diff = match (args.params_diff, args.params_diff_string) {
        (None      , None        ) => None,
        (Some(path), None        ) => Some(serde_json::from_str::<ParamsDiff>(&std::fs::read_to_string(path).map_err(CliError::CantLoadParamsDiffFile)?).map_err(CliError::CantParseParamsDiffFile)?),
        (None      , Some(string)) => Some(serde_json::from_str::<ParamsDiff>(&string).map_err(CliError::CantParseParamsDiffString)?),
        (Some(_)   , Some(_)     ) => unreachable!()
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
        (Some(_)   , Some(_)     ) => unreachable!()
    };

    // Testing and stuff.

    let no_cleaning = args.test_suitability || !args.tests.is_empty();

    if args.test_suitability {
        cleaner.assert_suitability();
        println!("The chosen cleaner is suitable to be the default cleaner!");
    }

    if !args.tests.is_empty() {
        for test_path in args.tests {
            serde_json::from_str::<Tests>(&std::fs::read_to_string(test_path).map_err(CliError::CantLoadTests)?).map_err(CliError::CantParseTests)?
                .r#do(&cleaner);
        }
        println!("\nAll tests passed!");
    }

    if no_cleaning {std::process::exit(0);}

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

    let (buf_in_senders, buf_in_recievers) = (0..threads).map(|_| std::sync::mpsc::channel::<(Vec<u8>, Option<GetLazyTaskConfigError>)>()).collect::<(Vec<_>, Vec<_>)>();
    let (buf_ret_sender, buf_ret_reciever) = std::sync::mpsc::channel::<Vec<u8>>();
    let (out_senders, out_recievers) = (0..threads).map(|_| std::sync::mpsc::channel::<Result<BetterUrl, DoTaskError>>()).collect::<(Vec<_>, Vec<_>)>();

    let mut some_ok  = false;
    let mut some_err = false;


    let job_config = JobConfig {
        cleaner: &cleaner,
        context: &job_context,
        unthreader: &match (args.unthread, args.unthread_ratelimit) {
            (false, None   ) => UnthreaderMode::Multithread,
            (false, Some(_)) => unreachable!(),
            (true , None   ) => UnthreaderMode::Unthread,
            (true , Some(d)) => UnthreaderMode::Ratelimit(Duration::from_secs_f64(d))
        }.into(),
        #[cfg(feature = "cache")]
        cache_handle: CacheHandle {
            cache: &args.cache.into(),
            config: CacheHandleConfig {
                delay: args.cache_delay,
                read : !args.no_read_cache,
                write: !args.no_write_cache
            }
        },
        #[cfg(feature = "http")]
        http_client: &HttpClient::new([args.proxy, args.http_proxy, args.https_proxy].into_iter().flatten().map(|proxy| proxy.make()).collect::<Result<Vec<_>, _>>()?)
    };

    let mut buffers = args.urls.len() as u64;

    std::thread::scope(|s| {

        // Task getter thread.

        std::thread::Builder::new().name("Task Getter".to_string()).spawn_scoped(s, || {
            let buf_ret_reciever = buf_ret_reciever;
            let buf_in_senders   = buf_in_senders  ;
            let mut biss = buf_in_senders.iter().cycle();

            for (url, bis) in args.urls.into_iter().zip(&mut biss) {
                bis.send((url.into(), None)).expect("The current buffer in reciever to stay open while needed.");
            }

            let stdin = std::io::stdin();

            if !stdin.is_terminal() {
                let mut stdin = stdin.lock();

                for bis in biss {
                    // If there are no buffers available within the ratelimit, makea a new one.
                    // The ratelimit can reduce memory usage by up to 10x with, if properly tuned, minimal performance impact.
                    
                    let mut buf = match buf_ret_reciever.recv_timeout(Duration::from_nanos(buffers / 8)) {
                        Ok(mut buf) => {buf.clear(); buf},
                        Err(RecvTimeoutError::Timeout) => {
                            buffers += 1;
                            Vec::new()
                        },
                        Err(RecvTimeoutError::Disconnected) => panic!("Expected buffer return senders to stay open while needed.")
                    };

                    match stdin.read_until(b'\n', &mut buf) {
                        Ok(0) => break,
                        Ok(_) => {
                            if buf.ends_with(b"\r\n") {
                                buf.truncate(buf.len() - 2);
                            } else {
                                buf.truncate(buf.len() - 1);
                            }
                            bis.send((buf, None)).expect("The current buffer in reciever to stay open while needed.");
                        },
                        Err(e) => bis.send((buf, Some(e.into()))).expect("The current buffer in reciever to stay open while needed.")
                    }
                }
            }
        }).expect("Making threads to work fine.");

        // Worker threads.

        buf_in_recievers.into_iter().zip(out_senders).enumerate().map(|(i, (bir, os))| {
            let brs = buf_ret_sender.clone();
            std::thread::Builder::new().name(format!("Worker {i}")).spawn_scoped(s, move || {
                while let Ok((buf, err)) = bir.recv() {
                    let ret = match err {
                        None => {
                            let maybe_task = job_config.make_lazy_task(LazyTaskConfig::ByteSlice(&buf)).make();
                            // The buffer return reciever will hang up when there's no more tasks to do, so this returning Err is expected.
                            let _ = brs.send(buf);
                            match maybe_task {
                                Ok(task) => task.r#do(),
                                Err(e) => Err(e.into())
                            }
                        },
                        Some(e) => Err(DoTaskError::from(MakeTaskError::from(MakeLazyTaskError::from(e))))
                    };

                    os.send(ret).expect("The result out reciever to stay open while needed.");
                }
            }).expect("Making threads to work fine.");
        }).for_each(drop);

        // Stdout thread.

        std::thread::Builder::new().name("Stdout".to_string()).spawn_scoped(s, || {
            let mut stdout = std::io::stdout().lock();

            if args.json {
                let mut first_job = true;

                stdout.write_all(b"{\"Ok\":{\"urls\":[").expect("Writing JSON prelude to STDOUT to work.");

                for or in {out_recievers}.iter().cycle() {
                    match or.recv() {
                        Ok(Ok(url)) => {
                            if !first_job {stdout.write_all(b",").expect("Writing task result separators STDOUT to work.");}
                            serde_json::to_writer(&mut stdout, &Ok::<_, ()>(url)).expect("Writing task results to STDOUT to work.");
                            some_ok = true;
                        },
                        Ok(Err(e)) => {
                            if !first_job {stdout.write_all(b",").expect("Writing task result separators STDOUT to work.");}
                            serde_json::to_writer(&mut stdout, &Err::<(), _>(ErrorToSerdeString(e))).expect("Writing task results to STDOUT to work.");
                            some_err = true;
                        },
                        Err(_) => break
                    }
                    first_job = false;
                }

                stdout.write_all(b"]}}").expect("Writing JSON epilogue to STDOUT to work.");
            } else if (stdout.is_terminal() || args.unbuffer_output) && !args.buffer_output {
                for or in {out_recievers}.iter().cycle() {
                    match or.recv() {
                        Ok(Ok(url)) => {
                            stdout.write_all(url.as_str().as_bytes()).expect("Writing task results to STDOUT to work.");
                            stdout.write_all(b"\n").expect("Writing newlines to STDOUT to work.");
                            some_ok = true;
                        },
                        Ok(Err(e)) => {
                            stdout.write_all(b"\n").expect("Writing blank lines to STDOUT to work.");
                            eprintln!("{e:?}");
                            some_err = true;
                        }
                        Err(_) => break
                    }
                }
            } else {
                let mut stdout = BufWriter::with_capacity(8192, stdout);

                for or in {out_recievers}.iter().cycle() {
                    match or.recv() {
                        Ok(Ok(url)) => {
                            stdout.write_all(url.as_str().as_bytes()).expect("Writing task results to STDOUT to work.");
                            stdout.write_all(b"\n").expect("Writing newlines to STDOUT to work.");
                            some_ok = true;
                        },
                        Ok(Err(e)) => {
                            stdout.write_all(b"\n").expect("Writing blank lines to STDOUT to work.");
                            stdout.flush().expect("Flushing STDOUT to work.");
                            eprintln!("{e:?}");
                            some_err = true;
                        }
                        Err(_) => break
                    }
                }
            }
        }).expect("Making threads to work fine.");
    });

    Ok(match (some_ok, some_err) {
        (false, false) => 0,
        (false, true ) => 1,
        (true , false) => 0,
        (true , true ) => 2
    }.into())
}
