//! URL Cleaner CLI - Explicit non-consent to URL spytext.
//!
//! See [url_cleaner_engine] to integrate URL Cleaner with your own projects.


use std::path::PathBuf;
use std::io::{IsTerminal, BufRead, Write, BufWriter};
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
    /// Omit to use the built in bundled cleaner.
    #[cfg(feature = "bundled-cleaner")]
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    cleaner: Option<PathBuf>,
    /// The cleaner file to use.
    #[cfg(not(feature = "bundled-cleaner"))]
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    cleaner: PathBuf,
    /// The size of the STDOUT buffer.
    /// When STDOUT is a terminal, defaults to 0.
    /// Otherwise defaults to 8192.
    #[arg(long, verbatim_doc_comment)]
    output_buffer: Option<usize>,
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
    /// The JobContext file to use.
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    job_context: Option<PathBuf>,
    /// The proxy to use for HTTP/HTTPS requests.
    #[cfg(feature = "http")]
    #[arg(long, verbatim_doc_comment)]
    proxy: Option<HttpProxyConfig>,
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
    threads: usize
}

/// The enum of errors [`main`] can return.
#[derive(Debug, Error)]
pub enum CliError {
    /// Returned when a [`GetCleanerError`] is encountered.
    #[error(transparent)] GetCleanerError(#[from] GetCleanerError),
    /// Returned when unable to load a [`ParamsDiff`] file.
    #[error(transparent)] CantLoadProfiles(std::io::Error),
    /// Returned when unable to parse a [`ParamsDiff`] file.
    #[error(transparent)] CantParseProfiles(serde_json::Error),
    /// Returned when unable to load a [`ParamsDiff`] file.
    #[error(transparent)] CantLoadParamsDiff(std::io::Error),
    /// Returned when unable to parse a [`ParamsDiff`] file.
    #[error(transparent)] CantParseParamsDiff(serde_json::Error),
    /// Returned when unable to load a [`JobContext`] file.
    #[error(transparent)] CantLoadJobContext(std::io::Error),
    /// Returned when unable to parse a [`JobContext`] file.
    #[error(transparent)] CantParseJobContext(serde_json::Error),
    /// Returned when the requested [`Profile`] isn't found.
    #[error("The requested Profile wasn't found.")]
    ProfileNotFound,
    /// Returned when a [`MakeHttpProxyError`] is encountered.
    #[cfg(feature = "http")]
    #[error(transparent)] MakeHttpProxyError(#[from] MakeHttpProxyError)
}

fn main() -> Result<(), CliError> {
    let args = Args::parse();

    #[cfg(feature = "bundled-cleaner")]
    let mut cleaner = Cleaner::load_or_get_bundled_no_cache(args.cleaner.as_deref())?;
    #[cfg(not(feature = "bundled-cleaner"))]
    let mut cleaner = Cleaner::load_from_file(&args.cleaner)?;

    // Get and apply [`ParamsDiff`]s.

    if let Some(path) = args.profiles {
        cleaner = ProfiledCleanerConfig {
            cleaner,
            profiles_config: serde_json::from_str(&std::fs::read_to_string(path).map_err(CliError::CantLoadProfiles)?).map_err(CliError::CantParseProfiles)?
        }.into_profile(args.profile.as_deref()).ok_or(CliError::ProfileNotFound)?;
    }

    if let Some(path) = args.params_diff {
        serde_json::from_str::<ParamsDiff>(&std::fs::read_to_string(path).map_err(CliError::CantLoadParamsDiff)?)
            .map_err(CliError::CantParseParamsDiff)?
            .apply(&mut cleaner.params);
    }

    cleaner.params.flags.to_mut().extend(args.flag);
    for var in args.var {
        let [name, value] = var.try_into().expect("The clap parser to work");
        cleaner.params.vars.to_mut().insert(name, value);
    }

    // Get the [`JobContext`].

    let job_context = match args.job_context {
        None       => Default::default(),
        Some(path) => serde_json::from_str(&std::fs::read_to_string(path).map_err(CliError::CantLoadJobContext)?).map_err(CliError::CantParseJobContext)?,
    };

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

    let (buf_in_senders, buf_in_recievers) = (0..threads).map(|_| std::sync::mpsc::channel::<Result<Vec<u8>, (Vec<u8>, std::io::Error)>>()).collect::<(Vec<_>, Vec<_>)>();
    let (buf_ret_sender, buf_ret_reciever) = std::sync::mpsc::channel::<Vec<u8>>();
    let (out_senders, out_recievers) = (0..threads).map(|_| std::sync::mpsc::channel::<Box<str>>()).collect::<(Vec<_>, Vec<_>)>();

    let job_config = JobConfig {
        cleaner: &cleaner,
        context: &job_context,
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
        http_client: &HttpClient::new(args.proxy.into_iter().map(|proxy| proxy.make()).collect::<Result<Vec<_>, _>>()?)
    };

    let mut buffers = args.urls.len() as u64;

    std::thread::scope(|s| {

        // Task getter thread.

        std::thread::Builder::new().name("LazyTaskConfig Getter".to_string()).spawn_scoped(s, || {
            let buf_ret_reciever = buf_ret_reciever;
            let buf_in_senders   = buf_in_senders  ;
            let mut biss = buf_in_senders.iter().cycle();

            for (url, bis) in args.urls.into_iter().zip(&mut biss) {
                bis.send(Ok(url.into())).expect("The current buffer in reciever to stay open while needed.");
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
                            if buf.ends_with(b"\n") {
                                buf.pop();
                                if buf.ends_with(b"\r") {
                                    buf.pop();
                                }
                            }
                            bis.send(Ok(buf)).expect("The current buffer in reciever to stay open while needed.");
                        },
                        Err(e) => bis.send(Err((buf, e))).expect("The current buffer in reciever to stay open while needed.")
                    }
                }
            }
        }).expect("Making threads to work fine.");

        // Worker threads.

        for (i, (bir, os)) in buf_in_recievers.into_iter().zip(out_senders).enumerate() {
            let brs = buf_ret_sender.clone();
            std::thread::Builder::new().name(format!("Worker {i}")).spawn_scoped(s, move || {
                while let Ok(x) = bir.recv() {
                    let ret = match x {
                        Ok(buf) => {
                            let maybe_task = job_config.make_lazy_task(&buf).make();
                            // The buffer return reciever will hang up when there's no more tasks to do, so this returning Err is expected.
                            let _ = brs.send(buf);
                            match maybe_task {
                                Ok(task) => match task.r#do() {
                                    Ok(x) => x.into(),
                                    Err(e) => format!("-{e:?}")
                                },
                                Err(e) => format!("-{:?}", DoTaskError::from(e))
                            }
                        },
                        Err((buf, e)) => {
                            let _ = brs.send(buf);
                            format!("-{:?}", DoTaskError::from(MakeTaskError::from(GetTaskError::from(e))))
                        }
                    };

                    os.send(ret.into_boxed_str()).expect("The result out reciever to stay open while needed.");
                }
            }).expect("Making threads to work fine.");
        }

        // Stdout thread.

        std::thread::Builder::new().name("Stdout".to_string()).spawn_scoped(s, || {
            let stdout = std::io::stdout().lock();
            let buffer_size = match args.output_buffer {
                Some(x) => x,
                None => if stdout.is_terminal() {
                    0
                } else {
                    8192
                }
            };
            let mut stdout = BufWriter::with_capacity(buffer_size, stdout);

            for or in {out_recievers}.iter().cycle() {
                match or.recv() {
                    Ok(x) => writeln!(stdout, "{x}").expect("Writing to STDOUT to never fail."),
                    Err(_) => break
                }
            }
        }).expect("Making threads to work fine.");
    });

    Ok(())
}
