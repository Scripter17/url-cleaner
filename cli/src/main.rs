//! URL Cleaner CLI - Explicit non-consent to URL spytext.
//!
//! See [url_cleaner_engine] to integrate URL Cleaner with your own projects.

use std::path::PathBuf;
use std::io::IsTerminal;
use std::fmt::Debug;

use clap::Parser;
use thiserror::Error;
use bytes::Bytes;
use tokio::io::AsyncReadExt;

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
    #[arg(long, verbatim_doc_comment, default_value_t = 0)]
    workers: usize
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

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let args = Args::parse();

    #[cfg(feature = "bundled-cleaner")]
    let mut cleaner = Cleaner::load_or_get_bundled_no_cache(args.cleaner)?;
    #[cfg(not(feature = "bundled-cleaner"))]
    let mut cleaner = Cleaner::load_from_file(args.cleaner)?;

    if let Some(path) = args.profiles {
        cleaner = ProfiledCleanerConfig {
            cleaner,
            profiles_config: ProfilesConfig::load_from_file(path)?
        }.into_profile(args.profile.as_deref()).ok_or(CliError::ProfileNotFound)?;
    }

    if let Some(path) = args.params_diff {
        ParamsDiff::load_from_file(path)?.apply(&mut cleaner.params);
    }

    for flag in args.flag {
        cleaner.params.flags.to_mut().insert(flag);
    }
    for mut x in args.var {
        cleaner.params.vars.to_mut().insert(x.remove(0), x.remove(0));
    }

    let job: &_ = Box::leak(Box::new(Job {
        context: args.job_context.map(JobContext::load_from_file).transpose()?.unwrap_or_default(),
        cleaner,
        unthreader: Box::leak(Box::new(Unthreader::r#if(args.unthread))),
        #[cfg(feature = "cache")]
        cache: Cache {
            inner: Box::leak(Box::new(args.cache.into())),
            config: CacheConfig {
                read : !args.no_read_cache,
                write: !args.no_write_cache,
                delay:  args.cache_delay,
            }
        },
        #[cfg(feature = "http")]
        http_client: Box::leak(Box::new(HttpClient::new()))
    }));

    // Do the job.

    let threads = match args.workers {
        0 => std::thread::available_parallelism().expect("To be able to get the available parallelism.").get(),
        x => x
    };

    let (iss,     irs) = (0..threads).map(|_| tokio::sync::mpsc::channel::<Bytes >(512)).collect::<(Vec<_>, Vec<_>)>();
    let (oss, mut ors) = (0..threads).map(|_| tokio::sync::mpsc::channel::<String>(512)).collect::<(Vec<_>, Vec<_>)>();

    let input = tokio::spawn(async move {
        let mut isi = (0..iss.len()).cycle();

        for task in args.tasks.into_iter() {
            iss.get(isi.next().expect("???")).expect("???").send(task.into()).await.expect("The in receiver to still exist.");
        }

        if !std::io::stdin().is_terminal() {
            let stdin = &mut tokio::io::stdin();
            let mut buf = Vec::new();

            while tokio::time::timeout(std::time::Duration::from_millis(1), stdin.take(2u64.pow(18)).read_to_end(&mut buf)).await.map(Result::unwrap) != Ok(0) {
                if let Some(i) = buf.iter().rev().position(|b| *b == b'\n').map(|i| buf.len() - i) {
                    let temp = buf.split_off(i);
                    buf.pop();
                    let bytes = Bytes::from_owner(buf);
                    for line in bytes.split(|b| *b == b'\n').map(|x| x.strip_suffix(b"\r").unwrap_or(x)) {
                        if !line.is_empty() {
                            iss.get(isi.next().expect("???")).expect("???").send(bytes.slice_ref(line)).await.expect("The in receiever to still exist.");
                        }
                    }
                    buf = temp;
                }
            }

            if buf.ends_with(b"\n") {
                buf.pop();
                buf.pop_if(|b| *b == b'\r');
            }

            if !buf.is_empty() {
                iss.get(isi.next().expect("???")).expect("???").send(buf.into()).await.expect("The in receiver to still exist");
            }
        }
    });

    for (mut ir, os) in irs.into_iter().zip(oss) {
        std::thread::spawn(move || {
            while let Some(task) = ir.blocking_recv() {
                os.blocking_send(match job.r#do(&*task) {
                    Ok (x) => x.into(),
                    Err(e) => format!("-{e:?}")
                }).expect("The out receiver to still exist.");
            }
        });
    }

    let output = tokio::spawn(async move {
        let mut buf = String::new();
        let mut ori = (0..ors.len()).cycle();
        let mut or  = ors.get_mut(ori.next().expect("???")).expect("???");

        loop {
            match tokio::time::timeout(std::time::Duration::from_millis(1), or.recv()).await {
                Ok(Some(x)) => {
                    buf.push_str(&x);
                    buf.push('\n');

                    if buf.len() >= 2usize.pow(18) {
                        print!("{buf}");
                        buf.clear();
                    }

                    or = ors.get_mut(ori.next().expect("???")).expect("???");
                },
                Ok(None) => {
                    if !buf.is_empty() {
                        print!("{buf}");
                    }
                    break;
                },
                Err(_) => {
                    if !buf.is_empty() {
                        print!("{buf}");
                    }
                    buf.clear();
                }
            }
        }
    });

    tokio::try_join!(input, output).expect("???");

    Ok(())
}
