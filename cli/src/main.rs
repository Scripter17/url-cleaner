//! URL Cleaner CLI - Explicit non-consent to URL spytext.
//!
//! See [url_cleaner_engine] to integrate URL Cleaner with your own projects.

use std::num::NonZero;
use std::path::PathBuf;
use std::io::IsTerminal;
use std::fmt::Debug;
use std::sync::OnceLock;
use std::borrow::Cow;

use clap::Parser;
use thiserror::Error;
use bytes::Bytes;
use tokio::io::AsyncReadExt;

use url_cleaner_engine::prelude::*;

#[expect(rustdoc::bare_urls, reason = "It'd look bad in the console.")]
/// URL Cleaner CLI - Explicit non-consent to URL spytext.
///
/// Licensed under the Aferro GNU Public License version 3.0 or later.
///
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
    /// Task lines to do before STDIN.
    tasks: Vec<String>,

    /// The Cleaner to use.
    #[cfg(feature = "bundled-cleaner")]
    #[arg(long, short = 'c', value_name = "PATH")]
    cleaner: Option<PathBuf>,
    /// The Cleaner to use.
    #[cfg(not(feature = "bundled-cleaner"))]
    #[arg(long, short = 'c', value_name = "PATH")]
    cleaner: PathBuf,

    /// The ProfilesConfig to use.
    #[arg(long, value_name = "PATH")]
    profiles: Option<PathBuf>,
    /// The profile to use.
    #[arg(long, value_name = "NAME", requires = "profiles")]
    profile: Option<String>,

    /// The ParamsDiff to apply on top of the profile.
    #[arg(long, value_name = "PATH")]
    params_diff: Option<PathBuf>,

    /// Flags to apply on top of the ParamsDiff.
    #[arg(long, short = 'f')]
    flag: Vec<String>,
    /// Vars to apply on top of the ParamsDiff.
    #[arg(long, short = 'v', value_names = ["NAME", "VALUE"], num_args = 2)]
    var: Vec<Vec<String>>,

    /// The JobContext.
    #[arg(long, value_name = "PATH")]
    job_context: Option<PathBuf>,

    /// The Secrets to use.
    #[arg(long, value_name = "PATH")]
    secrets: Option<PathBuf>,

    /// Replace unchanged lines with =.
    #[arg(long, short = 'u')]
    brief_unchanged: bool,
    /// Replace error lines with -.
    #[arg(long, short = 'e')]
    brief_error: bool,

    /// The number of worker threads to use.
    #[arg(long, short = 't')]
    threads: Option<NonZero<usize>>,
    /// Hide the thread count.
    #[arg(long, short = 'T')]
    hide_threads: bool,

    /// Disable the HTTP client.
    #[cfg(feature = "http")]
    #[arg(long, short = 'H')]
    no_http: bool,

    /// The CacheTarget to use.
    #[cfg(feature = "cache")]
    #[arg(long, default_value = "url-cleaner.sqlite")]
    cache: CacheTarget,
    /// Disable reading from the cache.
    #[cfg(feature = "cache")]
    #[arg(long, short = 'R')]
    no_read_cache: bool,
    /// Disable writing to the cache.
    #[cfg(feature = "cache")]
    #[arg(long, short = 'W')]
    no_write_cache: bool,
    /// Hide cache reads.
    #[cfg(feature = "cache")]
    #[arg(long, short = 'C')]
    hide_cache: bool,
}

/// The enum of errors [`main`] can return.
#[derive(Debug, Error)]
pub enum CliError {
    /** [`LoadCleanerError`].        **/ #[error(transparent)] LoadCleanerError       (#[from] LoadCleanerError       ),
    /** [`LoadParamsDiffError`].     **/ #[error(transparent)] LoadParamsDiffError    (#[from] LoadParamsDiffError    ),
    /** [`LoadProfilesConfigError`]. **/ #[error(transparent)] LoadProfilesConfigError(#[from] LoadProfilesConfigError),
    /** [`LoadJobContextError`].     **/ #[error(transparent)] LoadJobContextError    (#[from] LoadJobContextError    ),
    /** [`LoadSecretsError`].        **/ #[error(transparent)] LoadSecretsError       (#[from] LoadSecretsError       ),
    /// Returned when the requested profile isn't found.
    #[error("The requested profile wasn't found.")]
    ProfileNotFound
}

/** The [`Job`].         **/ static JOB       : OnceLock<Job    > = OnceLock::new();
/** The [`Secrets`].     **/ static SECRETS   : OnceLock<Secrets> = OnceLock::new();
/** The [`CacheClient`]. **/ #[cfg(feature = "cache")] static CACHE_CLIENT: OnceLock<CacheClient> = OnceLock::new();
/** The [`HttpClient`].  **/ #[cfg(feature = "http" )] static HTTP_CLIENT : OnceLock<HttpClient > = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let args = Args::parse();

    let (_, mut cleaner) = cfg_select! {
        feature = "bundled-cleaner" => Cleaner::load_or_new_bundled(args.cleaner)?,
        _                           => Cleaner::load               (args.cleaner)?,
    };

    let secrets = Secrets::load_or_default(args.secrets)?;

    if let Some(profiles) = args.profiles {
        let (_, mut profiles_config) = ProfilesConfig::load(profiles)?;

        profiles_config.base.apply(&mut cleaner.params);

        if let Some(profile) = args.profile {
            let diff = profiles_config.named.remove(&profile).ok_or(CliError::ProfileNotFound)?;
            diff.apply(&mut cleaner.params);
        }
    }

    if let Some(path) = args.params_diff {
        let (_, params_diff) = ParamsDiff::load(path)?;
        params_diff.apply(&mut cleaner.params);
    }

    if !args.flag.is_empty() {
        cleaner.params.flags.to_mut().extend(args.flag);
    }
    for mut x in args.var {
        cleaner.params.vars.to_mut().insert(x.remove(0), x.remove(0));
    }

    let (_, context) = JobContext::load_or_default(args.job_context)?;

    #[cfg(feature = "http" )] let http_client  = HttpClient ::new(          ).await;
    #[cfg(feature = "cache")] let cache_client = CacheClient::new(args.cache).await;

    let job = JOB.get_or_init(|| Job {
        context,
        cleaner,
        secrets: SECRETS.get_or_init(|| secrets),
        thread_hider: args.hide_threads.then(Default::default),
        #[cfg(feature = "http")]
        http_client: (!args.no_http).then(|| HTTP_CLIENT.get_or_init(|| http_client)),
        #[cfg(feature = "cache")]
        cache_client: CACHE_CLIENT.get_or_init(|| cache_client),
        #[cfg(feature = "cache")]
        cache_config: CacheConfig {
            read : !args.no_read_cache ,
            write: !args.no_write_cache,
            hide :  args.hide_cache    ,
        },
    });

    let threads = args.threads.unwrap_or_else(|| std::thread::available_parallelism().expect("To be able to get the available parallelism."));

    let (iss,     irs) = (0..threads.get()).map(|_| tokio::sync::mpsc::channel::<Bytes            >(512)).collect::<(Vec<_>, Vec<_>)>();
    let (oss, mut ors) = (0..threads.get()).map(|_| tokio::sync::mpsc::channel::<Cow<'static, str>>(512)).collect::<(Vec<_>, Vec<_>)>();

    let input = tokio::spawn(async move {
        let mut isi = (0..iss.len()).cycle();

        for task in args.tasks.into_iter() {
            iss.get(isi.next().expect("???")).expect("???").send(task.into()).await.expect("The in receiver to still exist.");
        }

        if !std::io::stdin().is_terminal() {
            let stdin = &mut tokio::io::stdin();
            let mut buf = Vec::new();

            while tokio::time::timeout(std::time::Duration::from_millis(1), stdin.take(2u64.pow(18)).read_to_end(&mut buf)).await.map(Result::unwrap) != Ok(0) {
                if let Some(i) = better_url::util::memrchr(&buf, b'\n') {
                    let temp = buf.split_off(i + 1);
                    let bytes = Bytes::from_owner(buf);

                    let lines = better_url::util::MemchrLines {remainder: Some(&bytes)};

                    for line in lines {
                        if !line.is_empty() {
                            iss.get(isi.next().expect("???")).expect("???").send(bytes.slice_ref(line)).await.expect("The in receiever to still exist.")
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
                    Ok((false, _  )) if args.brief_unchanged => "=".into(),
                    Ok((_    , url))                         => url.into(),

                    Err(_) if args.brief_error => "-"              .into(),
                    Err(e)                     => format!("-{e:?}").into(),
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

                    if buf.len() >= 2usize.pow(16) {
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
