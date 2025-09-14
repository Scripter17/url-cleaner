//! A basic HTTP server and userscript to allow automatically applying [URL Cleaner](https://github.com/Scripter17/url-cleaner) to every URL on every webpage you visit.

use std::net::IpAddr;
use std::path::PathBuf;
use std::fs::read_to_string;
use std::str::FromStr;
use std::sync::Mutex;
use std::num::NonZero;
use std::io::Write;
use std::sync::{OnceLock, LazyLock};
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

use url_cleaner_engine::types::*;
use url_cleaner_engine::glue::*;
use url_cleaner_engine::helpers::*;
use url_cleaner_site_types::*;

/// The base info to return when getting `/`.
const INFO: &str = r#"URL Cleaner Site
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html
https://github.com/Scripter17/url-cleaner

See /get-cleaner       to get the loaded Cleaner.
See /get-profiles      to get the loaded Profiles.
See /get-max-json-size to get the max size of a JobConfig's JSON."#;

/// The default max size of a payload to the [`clean`] route.
const DEFAULT_MAX_JSON_SIZE: &str = "25MiB";
/// The default IP to listen to.
const DEFAULT_IP           : &str = "127.0.0.1";
/// The default port to listen to.
const DEFAULT_PORT         : u16  = 9149;

/// The root directory to put logs into.
static LOGGING_DIR_ROOT: OnceLock<time::format_description::OwnedFormatItem> = OnceLock::new();

/// The string format of the directories to write logs to.
const LOGGING_DIR_PATH_FORMAT_STR : &str = "/[year]/[month]/[day]/[hour]";
/// The string format of the files to wrtie logs to.
const LOGGING_FILE_PATH_FORMAT_STR: &str = "/[year]/[month]/[day]/[hour]/[minute]-[second]-[subsecond].json";

/// The parsed format of the directories to write logs to.
static LOGGING_DIR_PATH_FORMAT : LazyLock<Vec<time::format_description::OwnedFormatItem>> = LazyLock::new(|| {
    let mut ret = vec![LOGGING_DIR_ROOT.get().expect("LOGGING_DIR_ROOT to always be set before getting LOGGING_DIR_PATH_FORMAT").clone()];
    ret.push(time::format_description::parse_owned::<2>(LOGGING_DIR_PATH_FORMAT_STR ).expect("The logging directory format to be valid."));
    ret
});

/// The parsed format of the files to wrtie logs to.
static LOGGING_FILE_PATH_FORMAT: LazyLock<Vec<time::format_description::OwnedFormatItem>> = LazyLock::new(|| {
    let mut ret = vec![LOGGING_DIR_ROOT.get().expect("LOGGING_DIR_ROOT to always be set before getting LOGGING_FILE_PATH_FORMAT").clone()];
    ret.push(time::format_description::parse_owned::<2>(LOGGING_FILE_PATH_FORMAT_STR).expect("The logging file format to be valid."));
    ret
});

/// Clap doesn't like `<rocket::data::ByteUnit as FromStr>::Error`.
fn parse_byte_unit(s: &str) -> Result<rocket::data::ByteUnit, String> {
    rocket::data::ByteUnit::from_str(s).map_err(|x| x.to_string())
}

#[allow(rustdoc::bare_urls, reason = "It'd look bad in the console.")]
/// A basic HTTP server and userscript to allow automatically applying URL Cleaner to every URL on every webpage you visit.
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
    #[arg(long, default_value = DEFAULT_MAX_JSON_SIZE, value_parser = parse_byte_unit)]
    max_size: rocket::data::ByteUnit,
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
    #[arg(long, default_value = "true", action = clap::ArgAction::Set, value_name = "BOOL")]
    read_cache_default: bool,
    /// Whether or not to write to the cache.
    #[cfg(feature = "cache")]
    #[arg(long, default_value = "true", action = clap::ArgAction::Set, value_name = "BOOL")]
    write_cache_default: bool,
    /// Defaults whether or not to use cache delay in jobs that don't specify otherwise.
    #[cfg(feature = "cache")]
    #[arg(long, default_value = "false", action = clap::ArgAction::Set, value_name = "BOOL")]
    cache_delay_default: bool,
    /// If true, makes requests, cache reads, etc. effectively single threaded to hide thread count.
    #[arg(long, default_value = "true", action = clap::ArgAction::Set, value_name = "BOOL")]
    hide_thread_count_default: bool,
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
    mtls_cert: Option<PathBuf>,
    /// Log all jobs and their results.
    #[arg(long)]
    log: bool,
    /// The directory to write logs to.
    #[arg(long, default_value = "logs")]
    log_dir: String
}

/// The config for the server.
#[derive(Debug)]
struct ServerConfig {
    /// The [`Cleaner`] to use.
    cleaner: ProfiledCleaner<'static>,
    /// A [`String`] version of [`Self::cleaner`].
    cleaner_string: String,
    /// A [`String`] version of the [`ProfilesConfig`]..
    profiles_config_string: String,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    cache: Cache,
    /// The default value for [`Job::cache_handle_config`]'s [`CacheHandleConfig::delay`].
    #[cfg(feature = "cache")]
    cache_delay_default: bool,
    /// [`CacheHandleConfig::read`].
    #[cfg(feature = "cache")]
    read_cache_default: bool,
    /// [`CacheHandleConfig::write`].
    #[cfg(feature = "cache")]
    write_cache_default: bool,
    /// The number of threads to spawn for each [`JobConfig`].
    threads: NonZero<usize>,
    /// The default value for if [`Job::unthreader`] is [`Unthreader::No`] or [`Unthreader::Yes`].
    hide_thread_count_default: bool,
    /// The max size for a [`JobConfig`]'s JSON representation.
    max_json_size: rocket::data::ByteUnit,
    /// The [`Accounts`] to use.
    accounts: Accounts,
    /// If [`true`], log all jobs and their results.
    log: bool
}

/// The state of the server.
#[derive(Debug)]
struct ServerState {
    /// The [`ServerConfig`] to use.
    config: ServerConfig,
    /// The number of [`JobConfig`]s handled. Used for naming threads.
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
    let cleaner: Cleaner = serde_json::from_str(&cleaner_string).expect("The cleaner file to contain a valid Cleaner.");

    if args.export_cleaner {
        println!("{}", serde_json::to_string(&cleaner).expect("Cleaners to always serialize to JSON."));
        std::process::exit(0);
    }

    let profiles_config_string = match (args.profiles, args.profiles_string) {
        (None      , None        ) => "{}".into(),
        (Some(path), None        ) => std::fs::read_to_string(path).expect("The ProfilesConfig file to be readable."),
        (None      , Some(string)) => string,
        (Some(_)   , Some(_)     ) => panic!("Cannot have both --profiles and --profiles-string.")
    };
    let profiles_config = serde_json::from_str::<ProfilesConfig>(&profiles_config_string).expect("The ProfilesConfig to be a valid ProfilesConfig.");
    let cleaner = cleaner.with_profiles(profiles_config);

    LOGGING_DIR_ROOT.set(time::format_description::OwnedFormatItem::Literal(args.log_dir.into_bytes().into_boxed_slice())).expect("LOGGING_DIR_ROOT to only be set once.");

    let server_state = ServerState {
        config: ServerConfig {
            cleaner,
            cleaner_string,
            profiles_config_string,
            #[cfg(feature = "cache")]
            cache: args.cache.into(),
            #[cfg(feature = "cache")]
            cache_delay_default: args.cache_delay_default,
            #[cfg(feature = "cache")]
            read_cache_default : args.read_cache_default,
            #[cfg(feature = "cache")]
            write_cache_default: args.write_cache_default,
            threads: NonZero::new(args.threads).unwrap_or_else(|| std::thread::available_parallelism().expect("To be able to get the available parallelism.")),
            hide_thread_count_default: args.hide_thread_count_default,
            max_json_size: args.max_size,
            accounts: match args.accounts {
                Some(accounts) => serde_json::from_str(&std::fs::read_to_string(accounts).expect("The accounts file to be readable.")).expect("The accounts file to be valid."),
                None => Default::default()
            },
            log: args.log
        },
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
        limits: Limits::default().limit("json", args.max_size).limit("string", args.max_size),
        tls,
        ..rocket::Config::default()
    }).mount("/", routes![index, clean, get_max_json_size, get_cleaner, get_profiles]).manage(server_state)
}

/// The `/` route.
#[get("/")]
async fn index(state: &State<ServerState>) -> String {
    if state.config.log {
        format!("{INFO}\n\nThis instance has logging enabled.")
    } else {
        format!("{INFO}\n\nThis instance has logging disabled.")
    }
}

/// The `/get-cleaner` route.
#[get("/get-cleaner")]
async fn get_cleaner(state: &State<ServerState>) -> impl Responder<'_, '_> {
    (
        ContentType(MediaType::JSON),
        &*state.config.cleaner_string
    )
}

/// The `/get-profiles` route.
#[get("/get-profiles")]
async fn get_profiles(state: &State<ServerState>) -> impl Responder<'_, '_> {
    (
        ContentType(MediaType::JSON),
        &*state.config.profiles_config_string
    )
}

/// The `get-max-json-size` route.
#[get("/get-max-json-size")]
async fn get_max_json_size(state: &State<ServerState>) -> String {
    state.config.max_json_size.as_u64().to_string()
}

/// The `/clean` route.
#[post("/clean", data="<job_config>")]
async fn clean(state: &State<ServerState>, job_config: &str) -> (Status, Json<CleanResult>) {
    match serde_json::from_str::<JobConfig>(job_config) {
        Ok(job_config) => {
            let job_config_for_logging = state.config.log.then(|| job_config.clone());

            if !state.config.accounts.auth(job_config.auth.as_ref()) {
                return (Status {code: 401}, Json(Err(CleanError {status: 401, message: "Unauthorized".into()})));
            }

            let Some(mut cleaner) = state.config.cleaner.with_profile(job_config.profile.as_deref()) else {
                return (
                    Status {code: 422},
                    Json(Err(CleanError {
                        status: 422,
                        message: format!("Unknown profile: {:?}", job_config.profile)
                    }))
                );
            };
            if let Some(params_diff) = job_config.params_diff {
                params_diff.apply_once(&mut cleaner.params);
            }

            let (in_senders , in_recievers ) = (0..state.config.threads.get()).map(|_| std::sync::mpsc::channel::<Result<LazyTask<'_>, MakeLazyTaskError>>()).collect::<(Vec<_>, Vec<_>)>();
            let (out_senders, out_recievers) = (0..state.config.threads.get()).map(|_| std::sync::mpsc::channel::<Result<BetterUrl, DoTaskError>>()).collect::<(Vec<_>, Vec<_>)>();

            let ret_urls = std::sync::Mutex::new(Vec::with_capacity(job_config.tasks.len()));

            let mut temp = state.job_count.lock().expect("No panics.");
            let id = *temp;
            #[allow(clippy::arithmetic_side_effects, reason = "Not gonna happen.")]
            {*temp += 1;}
            drop(temp);

            let unthreader = Unthreader::r#if(job_config.hide_thread_count.unwrap_or(state.config.hide_thread_count_default));

            std::thread::scope(|s| {
                std::thread::Builder::new().name(format!("({id}) Task collector")).spawn_scoped(s, || {
                    let job = Job {
                        context: &job_config.context,
                        cleaner: &cleaner,
                        #[cfg(feature = "cache")]
                        cache: &state.config.cache,
                        #[cfg(feature = "cache")]
                        cache_handle_config: CacheHandleConfig {
                            delay: job_config.cache_delay.unwrap_or(state.config.cache_delay_default),
                            read : job_config.read_cache .unwrap_or(state.config.read_cache_default ),
                            write: job_config.write_cache.unwrap_or(state.config.write_cache_default)
                        },
                        unthreader: &unthreader,
                        lazy_task_configs: Box::new(job_config.tasks.into_iter().map(Ok))
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
                    let mut ret_urls_lock = ret_urls.lock().expect("No panics.");

                    for or in {out_recievers}.iter().cycle() {
                        match or.recv() {
                            Ok(x) => ret_urls_lock.push(x.map_err(|e| e.to_string())),
                            Err(_) => break
                        }
                    }
                }).expect("Spawning a thread to work fine.");
            });

            let result = CleanSuccess {
                urls: ret_urls.into_inner().expect("No panics.")
            };

            if state.config.log {
                let now = time::UtcDateTime::now();
                std::fs::create_dir_all(now.format(&LOGGING_DIR_PATH_FORMAT).expect("Formatting the time to work")).expect("Creating the logging directory to work.");
                std::fs::OpenOptions::new().create_new(true).write(true).open(now.format(&LOGGING_FILE_PATH_FORMAT).expect("Formatting the time to work."))
                    .expect("Creating the log file to work.")
                    .write_all(serde_json::to_string(&JobLog::Ok {
                        job_config: Box::new(job_config_for_logging.expect("???")),
                        result: Cow::Borrowed(&result)
                    }).expect("Serializing the JobLog to always work.").as_bytes())
                    .expect("Writing the log to work.");
            }

            (
                Status {code: 200},
                Json(Ok(result))
            )
        },
        Err(e) => {
            let result = CleanError {
                status: 422,
                message: e.to_string()
            };
            
            if state.config.log {
                let now = time::UtcDateTime::now();
                std::fs::create_dir_all(now.format(&LOGGING_DIR_PATH_FORMAT).expect("Formatting the time to work.")).expect("Creating the logging directory to work.");
                std::fs::OpenOptions::new().create_new(true).write(true).open(now.format(&LOGGING_FILE_PATH_FORMAT).expect("Formatting the time to work."))
                    .expect("Creating the log file to work.")
                    .write_all(serde_json::to_string(&JobLog::Err {
                        job_config: Cow::Borrowed(job_config),
                        result: Cow::Borrowed(&result)
                    }).expect("Serializing the JobLog to always work.").as_bytes())
                    .expect("Writing the log to work.");
            }

            (
                Status {code: 422},
                Json(Err(result))
            )
        }
    }
}
