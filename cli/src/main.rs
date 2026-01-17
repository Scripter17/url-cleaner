//! URL Cleaner CLI - Explicit non-consent to URL spytext.
//!
//! See [url_cleaner_engine] to integrate URL Cleaner with your own projects.

use std::path::PathBuf;
use std::io::{IsTerminal, BufRead, Write};
use std::fmt::Debug;
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
    /// Unvalidated task lines to do before STDIN.
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

    let mut profiled_cleaner_config = ProfiledCleanerConfig {
        #[cfg(    feature = "bundled-cleaner") ] cleaner: Cleaner::load_or_get_bundled_no_cache(args.cleaner)?,
        #[cfg(not(feature = "bundled-cleaner"))] cleaner: Cleaner::load_from_file(args.cleaner)?,
        profiles_config: args.profiles.map(ProfilesConfig::load_from_file).transpose()?.unwrap_or_default()
    };

    let pd_file = args.params_diff.map(ParamsDiff::load_from_file).transpose()?.unwrap_or_default();
    let pd_args = ParamsDiff {
        flags: args.flag.into_iter().collect(),
        vars: args.var.into_iter().map(|mut kv| (kv.remove(0), kv.remove(0))).collect(),
        ..Default::default()
    };

    if args.assert_suitability {
        for (name, mut profile) in profiled_cleaner_config.profiles_config.clone().into_each() {
            let name = name.as_deref().unwrap_or("Base");

            profile.params_diff.with_then(pd_file.clone());
            profiled_cleaner_config.profiles_config.named.insert(format!("{name} + ParamsDiff file"), profile.clone());

            profile.params_diff.with_then(pd_args.clone());
            profiled_cleaner_config.profiles_config.named.insert(format!("{name} + ParamsDiff file + ParamsDiff args"), profile);
        }

        profiled_cleaner_config.make().assert_suitability();

        return Ok(());
    }

    let job_context = args.job_context.map(JobContext::load_from_file).transpose()?.unwrap_or_default();

    let mut cleaner = profiled_cleaner_config.into_profile(args.profile.as_deref()).ok_or(CliError::ProfileNotFound)?;

    pd_file.apply(&mut cleaner.params);
    pd_args.apply(&mut cleaner.params);

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

    // Do the job.

    let threads = match args.threads {
        0 => std::thread::available_parallelism().expect("To be able to get the available parallelism.").get(),
        1.. => args.threads
    };

    // "In" channels from the getter thread to the worker threads.
    let (iss, irs) = (0..threads).map(|_| std::sync::mpsc::channel::<Vec<u8>>()).collect::<(Vec<_>, Vec<_>)>();
    // "Recycling" channel used to avoid allocating new buffers for each task line.
    // TODO: 512 seems to give good results but further testing is required.
    let (rs, rr) = std::sync::mpsc::sync_channel::<Vec<u8>>(512);
    // "Out" channels from the worker threads to the output thread.
    let (oss, ors) = (0..threads).map(|_| std::sync::mpsc::channel::<String>()).collect::<(Vec<_>, Vec<_>)>();

    std::thread::scope(|s| {

        // Input thread.

        s.spawn(move || {
            let mut iss = iss.iter().cycle();

            for task in args.tasks.into_iter() {
                iss.next().expect("???").send(task.into()).expect("The in receiver to still exist.");
            }

            let stdin = std::io::stdin();

            if !stdin.is_terminal() {
                let mut stdin = stdin.lock();

                let mut buf = Vec::new();

                while stdin.read_until(b'\n', &mut buf).expect("Reading from STDIN to always work.") > 0 {
                    if buf.ends_with(b"\n") {
                        buf.pop();
                        if buf.ends_with(b"\r") {
                            buf.pop();
                        }
                    }

                    if buf.is_empty() {
                        continue;
                    }

                    iss.next().expect("???").send(buf).expect("The in receiver to still exist.");

                    buf = rr.recv().expect("The recycle sender to still exist.");

                    buf.clear();
                }
            }
        });

        // Worker threads.

        for (ir, os) in irs.into_iter().zip(oss) {
            let rs = rs.clone();
            s.spawn(move || {
                while let Ok(buf) = ir.recv() {
                    let task = (&buf).make_task();
                    let _ = rs.try_send(buf);
                    os.send(match job.r#do(task) {
                        Ok (x) => x.into(),
                        Err(e) => format!("-{e:?}")
                    }).expect("The out receiver to still exist.");
                }
            });
        }

        // Output thread.

        let mut stdout = std::io::stdout().lock();
        let mut buf    = String::new();
        let mut ors    = ors.iter().cycle();
        let mut or     = ors.next().expect("???");

        loop {
            match or.recv_timeout(std::time::Duration::from_millis(1)) {
                Ok(x) => {
                    buf.push_str(&x);
                    buf.push('\n');

                    let _ = rs.try_send(x.into());

                    if buf.len() >= 2usize.pow(20) {
                        stdout.write_all(buf.as_bytes()).expect("Writing to STDOUT to always work.");
                        stdout.flush().expect("Flushing STDOUT to always work.");
                        buf.clear();
                    }

                    or = ors.next().expect("???");
                },
                Err(RecvTimeoutError::Disconnected) => {
                    if !buf.is_empty() {
                        stdout.write_all(buf.as_bytes()).expect("Writing to STDOUT to always work.");
                        stdout.flush().expect("Flushing STDOUT to always work.");
                    }
                    break;
                },
                Err(RecvTimeoutError::Timeout) => if !buf.is_empty() {
                    stdout.write_all(buf.as_bytes()).expect("Writing to STDOUT to always work.");
                    stdout.flush().expect("Flushing STDOUT to always work.");
                    buf.clear();
                }
            }
        }
    });

    Ok(())
}
