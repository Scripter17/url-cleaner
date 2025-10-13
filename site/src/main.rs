//! A basic HTTP server and userscript to allow automatically applying [URL Cleaner](https://github.com/Scripter17/url-cleaner) to every URL on every webpage you visit.

use std::net::IpAddr;
use std::path::PathBuf;
use std::fs::read_to_string;
use std::str::FromStr;
use std::sync::Mutex;
use std::num::NonZero;
use std::sync::LazyLock;
use std::borrow::Cow;

#[macro_use] extern crate rocket;
use rocket::{
    serde::json::Json,
    http::Status,
    data::Limits,
    State,
    response::Responder,
    http::{ContentType, MediaType}
};
use clap::Parser;

use better_url::BetterUrl;
use url_cleaner_engine::types::*;
use url_cleaner_engine::glue::prelude::*;
use url_cleaner_site_types::*;

/// The source code of this instance.
static SOURCE_CODE: LazyLock<BetterUrl> = LazyLock::new(|| env!("CARGO_PKG_REPOSITORY").parse().expect("The CARGO_PKG_REPOSITORY enviroment vairable to be a valid BetterUrl."));
/// The version of this instance.
const VERSION     : &str = env!("CARGO_PKG_VERSION");

/// The base info to return when getting `/`.
const INFO: &str = concat!("URL Cleaner Site ", env!("CARGO_PKG_VERSION"), r#"
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html
"#, env!("CARGO_PKG_REPOSITORY"), r#"

See /info     to get the ServerInfo.
See /cleaner  to get the Cleaner.
See /profiles to get the ProfilesConfig."#);

/// The default max size of a payload to the [`clean`] route.
const DEFAULT_MAX_PAYLOAD: &str = "25MiB";
/// The default IP to listen to.
const DEFAULT_IP         : &str = "127.0.0.1";
/// The default port to listen to.
const DEFAULT_PORT       : u16  = 9149;

/// An [`Unthreader`] that doesn't do any unthreading.
///
/// Used when unthrading is defaulted to or set to off.
static NO_UNTHREADER: LazyLock<Unthreader> = LazyLock::new(|| UnthreaderMode::Multithread.into());

/// Clap doesn't like `<rocket::data::ByteUnit as FromStr>::Error`.
fn parse_byte_unit(s: &str) -> Result<rocket::data::ByteUnit, String> {
    rocket::data::ByteUnit::from_str(s).map_err(|x| x.to_string())
}

#[allow(rustdoc::bare_urls, reason = "It'd look bad in the console.")]
/// URL Cleaner Site - Explicit non-consent to URL spytext.
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
    /// The max size of a POST request to the `/clean` endpoint.
    #[arg(long, default_value = DEFAULT_MAX_PAYLOAD, value_parser = parse_byte_unit)]
    max_payload: rocket::data::ByteUnit,
    /// The IP to listen to.
    #[arg(long, default_value = DEFAULT_IP)]
    ip: IpAddr,
    /// The port to listen to.
    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,
    /// The cache to use.
    #[cfg(feature = "cache")]
    #[arg(long, default_value = "url-cleaner-site-cache.sqlite", value_name = "PATH")]
    cache: CachePath,
    /// Whether or not to read from the cache.
    #[cfg(feature = "cache")]
    #[arg(long)]
    default_no_read_cache: bool,
    /// Whether or not to write to the cache.
    #[cfg(feature = "cache")]
    #[arg(long)]
    default_no_write_cache: bool,
    /// Defaults whether or not to use cache delay in jobs that don't specify otherwise.
    #[cfg(feature = "cache")]
    #[arg(long)]
    default_cache_delay: bool,
    /// Makes network requests, cache reads, etc. effectively single threaded while keeping most of the speed boost from multithreading.
    #[arg(long)]
    default_unthread: bool,
    /// When unthreading, also ratelimit unthreaded things.
    #[arg(long, value_name = "SECONDS")]
    unthread_ratelimit: Option<f64>,
    /// Amount of threads to process tasks in.
    ///
    /// Zero uses the CPU's thread count.
    #[arg(long, default_value_t = 0)]
    threads: usize,
    /// The accounts file to use.
    ///
    /// The format looks like this:
    ///
    /// {
    ///   "allow_guests": true,
    ///   "users": {
    ///     "username1": "password1",
    ///     "username2": "password2"
    ///   }
    /// }
    #[arg(verbatim_doc_comment, long, value_name = "PATH")]
    accounts: Option<PathBuf>,
    /// The TLS/HTTPS cert. If specified, requires `--key`.
    #[arg(long, requires = "key", value_name = "PATH")]
    cert: Option<PathBuf>,
    /// The TLS/HTTPS key. If specified, requires `--cert`.
    #[arg(long, requires = "cert", value_name = "PATH")]
    key: Option<PathBuf>,
    /// The mTLS client's certificate.
    #[arg(long, requires = "key", requires = "cert", value_name = "PATH")]
    mtls_cert: Option<PathBuf>
}

/// The config for the server.
#[derive(Debug)]
struct ServerConfig {
    /// A [`String`] version of the [`Cleaner`] used to make [`Self::profiled_cleaner`].
    cleaner_string: String,
    /// The [`ProfiledCleaner`] to use.
    profiled_cleaner: ProfiledCleaner<'static>,
    /// The string [`ProfilesConfig`] used.
    profiles_config_string: String,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    cache: Cache,
    /// The default value for [`JobConfig::cache_handle_config`]'s [`CacheHandleConfig::delay`].
    #[cfg(feature = "cache")]
    default_cache_delay: bool,
    /// [`CacheHandleConfig::read`].
    #[cfg(feature = "cache")]
    default_read_cache: bool,
    /// [`CacheHandleConfig::write`].
    #[cfg(feature = "cache")]
    default_write_cache: bool,
    /// The number of threads to spawn for each [`CleanPayload`].
    threads: NonZero<usize>,
    /// If [`true`], defaults unthreading to use [`ServerState::unthreader`].
    ///
    /// If [`false`], defaults unthreading to use [`NO_UNTHREADER`].
    default_unthread: bool,
    /// The max size for a [`CleanPayload`]'s JSON representation.
    max_payload: rocket::data::ByteUnit,
    /// The [`Accounts`] to use.
    accounts: Accounts
}

/// The state of the server.
#[derive(Debug)]
struct ServerState {
    /// The [`ServerConfig`] to use.
    config: ServerConfig,
    /// The default [`UnthreaderMode`].
    unthreader: Unthreader,
    /// The number of [`CleanPayload`]s handled. Used for naming threads.
    job_count: Mutex<usize>,
}

/// Make the server.
#[launch]
async fn rocket() -> _ {
    let args = Args::parse();

    #[cfg(feature = "default-cleaner")]
    let cleaner_string = args.cleaner.as_deref().map(|path| read_to_string(path).expect("The cleaner file to be readable.")).unwrap_or(DEFAULT_CLEANER_STR.to_string());
    #[cfg(not(feature = "default-cleaner"))]
    let cleaner_string = read_to_string(&args.cleaner).expect("The cleaner file to be readable.");
    let cleaner = Box::leak(Box::new(serde_json::from_str::<Cleaner<'static>>(&cleaner_string).expect("The cleaner file to contain a valid Cleaner."))).borrowed();

    let profiles_config_string = match (args.profiles, args.profiles_string) {
        (None      , None        ) => "{}".into(),
        (Some(path), None        ) => std::fs::read_to_string(path).expect("The ProfilesConfig file to be readable."),
        (None      , Some(string)) => string,
        (Some(_)   , Some(_)     ) => panic!("Cannot have both --profiles and --profiles-string.")
    };
    let profiles_config = serde_json::from_str::<ProfilesConfig>(&profiles_config_string).expect("The ProfilesConfig to be a valid ProfilesConfig.");
    // For my personal server, the leaking and borrowed()ing saves about half a megabyte of RAM.
    let profiled_cleaner = ProfiledCleanerConfig { cleaner, profiles_config }.make();

    let server_state = ServerState {
        config: ServerConfig {
            cleaner_string,
            profiled_cleaner,
            profiles_config_string,
            #[cfg(feature = "cache")]
            cache: args.cache.into(),
            #[cfg(feature = "cache")]
            default_cache_delay: args.default_cache_delay,
            #[cfg(feature = "cache")]
            default_read_cache : !args.default_no_read_cache,
            #[cfg(feature = "cache")]
            default_write_cache: !args.default_no_write_cache,
            threads: NonZero::new(args.threads).unwrap_or_else(|| std::thread::available_parallelism().expect("To be able to get the available parallelism.")),
            default_unthread: args.default_unthread,
            max_payload: args.max_payload,
            accounts: match args.accounts {
                Some(accounts) => serde_json::from_str(&std::fs::read_to_string(accounts).expect("The accounts file to be readable.")).expect("The accounts file to be valid."),
                None => Default::default()
            }
        },
        unthreader: match args.unthread_ratelimit {
            None    => UnthreaderMode::Unthread,
            Some(d) => UnthreaderMode::UnthreadAndRatelimit(std::time::Duration::from_secs_f64(d))
        }.into(),
        job_count: Mutex::new(0)
    };

    let tls = match (args.key, args.cert) {
        (Some(key), Some(cert)) => {
            let mut tls = rocket::config::TlsConfig::from_paths(cert, key);
            if let Some(mtls_cert) = args.mtls_cert {
                tls = tls.with_mutual(rocket::config::MutualTls::from_path(mtls_cert).mandatory(true));
            }
            Some(tls)
        },
        _ => None
    };

    rocket::custom(rocket::Config {
        address: args.ip,
        port: args.port,
        limits: Limits::default().limit("json", args.max_payload).limit("string", args.max_payload),
        tls,
        ..rocket::Config::default()
    }).mount("/", routes![index, info, clean, cleaner, profiles]).manage(server_state)
}

/// The `/` route.
#[get("/")]
async fn index() -> &'static str {
    &INFO
}

/// The `/info` endpoint.
#[get("/info")]
async fn info(state: &State<ServerState>) -> Json<ServerInfo<'_>> {
    Json(ServerInfo {
        source_code        : Cow::Borrowed(&SOURCE_CODE),
        version            : Cow::Borrowed(VERSION),
        max_payload        : state.config.max_payload.as_u64(),
        #[cfg(feature = "cache")]
        default_read_cache : state.config.default_read_cache,
        #[cfg(feature = "cache")]
        default_write_cache: state.config.default_write_cache,
        #[cfg(feature = "cache")]
        default_cache_delay: state.config.default_cache_delay,
        default_unthread   : state.config.default_unthread,
        unthreader_mode    : state.unthreader.mode
    })
}

/// The `/cleaner` route.
#[get("/cleaner")]
async fn cleaner(state: &State<ServerState>) -> impl Responder<'_, '_> {
    (
        ContentType(MediaType::JSON),
        &*state.config.cleaner_string
    )
}

/// The `/profiles` route.
#[get("/profiles")]
async fn profiles(state: &State<ServerState>) -> impl Responder<'_, '_> {
    (
        ContentType(MediaType::JSON),
        &*state.config.profiles_config_string
    )
}

/// The `/clean` route.
#[post("/clean", data="<clean_payload>")]
async fn clean(state: &State<ServerState>, clean_payload: &str) -> (Status, Json<CleanResult>) {
    match inner_clean(state, clean_payload) {
        Ok (clean_success) => (Status::Ok                       , Json(Ok (clean_success))),
        Err(clean_error  ) => (Status {code: clean_error.status}, Json(Err(clean_error  )))
    }
}

/// The actual `/clean` route handler, separated for easier error handling.
fn inner_clean(state: &State<ServerState>, clean_payload: &str) -> CleanResult {
    let clean_payload = match serde_json::from_str::<CleanPayload>(clean_payload) {
        Ok(clean_payload) => clean_payload,
        Err(e) => Err(CleanError {status: 422, message: e.to_string()})?
    };

    if !state.config.accounts.auth(clean_payload.config.auth.as_ref()) {
        Err(CleanError {status: 401, message: "Unauthorized".into()})?
    }

    let Some(mut cleaner) = state.config.profiled_cleaner.with_profile(clean_payload.config.profile.as_deref()) else {
        Err(CleanError {status: 422, message: format!("Unknown profile: {:?}", clean_payload.config.profile)})?
    };

    if let Some(params_diff) = clean_payload.config.params_diff {
        params_diff.apply_once(&mut cleaner.params);
    }

    let (in_senders , in_recievers ) = (0..state.config.threads.get()).map(|_| std::sync::mpsc::channel::<Result<LazyTask<'_, '_>, MakeLazyTaskError>>()).collect::<(Vec<_>, Vec<_>)>();
    let (out_senders, out_recievers) = (0..state.config.threads.get()).map(|_| std::sync::mpsc::channel::<Result<BetterUrl, DoTaskError>>()).collect::<(Vec<_>, Vec<_>)>();

    let mut ret_urls = Vec::with_capacity(clean_payload.tasks.len());

    let mut temp = state.job_count.lock().expect("No panics.");
    let id = *temp;
    *temp += 1;
    drop(temp);

    let unthreader = match (clean_payload.config.unthread, state.config.default_unthread) {
        (None, false) | (Some(false), _) => &NO_UNTHREADER,
        (None, true ) | (Some(true ), _) => &state.unthreader
    };

    std::thread::scope(|s| {
        std::thread::Builder::new().name(format!("({id}) Task collector")).spawn_scoped(s, || {
            let job = Job {
                config: &JobConfig {
                    context: &clean_payload.config.context,
                    cleaner: &cleaner,
                    #[cfg(feature = "cache")]
                    cache: &state.config.cache,
                    #[cfg(feature = "cache")]
                    cache_handle_config: CacheHandleConfig {
                        delay: clean_payload.config.cache_delay.unwrap_or(state.config.default_cache_delay),
                        read : clean_payload.config.read_cache .unwrap_or(state.config.default_read_cache ),
                        write: clean_payload.config.write_cache.unwrap_or(state.config.default_write_cache)
                    },
                    unthreader
                },
                lazy_task_configs: Box::new(clean_payload.tasks.into_iter().map(Ok))
            };
            for (in_sender, maybe_task_source) in {in_senders}.iter().cycle().zip(job) {
                in_sender.send(maybe_task_source).expect("To successfully send the LazyTask.");
            }
        }).expect("Spawning a thread to work fine.");

        in_recievers.into_iter().zip(out_senders).enumerate().map(|(i, (ir, os))| {
            std::thread::Builder::new().name(format!("({id}) Worker {i}")).spawn_scoped(s, move || {
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
            }).expect("Spawning a thread to work fine.");
        }).for_each(drop);

        std::thread::Builder::new().name(format!("({id}) Task returner")).spawn_scoped(s, || {
            for or in {out_recievers}.iter().cycle() {
                match or.recv() {
                    Ok(x) => ret_urls.push(x.map_err(|e| e.to_string())),
                    Err(_) => break
                }
            }
        }).expect("Spawning a thread to work fine.");
    });

    Ok(CleanSuccess {
        urls: ret_urls
    })
}
