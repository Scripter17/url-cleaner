//! A basic HTTP server and userscript to allow automatically applying [URL Cleaner](https://github.com/Scripter17/url-cleaner) to every URL on every webpage you visit.

use std::net::IpAddr;
use std::path::PathBuf;
use std::fs::read_to_string;
use std::str::FromStr;
use std::sync::Mutex;
use std::num::NonZero;

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::data::Limits;
use rocket::State;
use clap::Parser;

use url_cleaner_engine::types::*;
use url_cleaner_engine::glue::*;
use url_cleaner_site_types::*;

/// The default max size of a payload to the [`clean`] route.
const DEFAULT_MAX_JSON_SIZE: &str = "25MiB";
/// The default IP to listen to.
const DEFAULT_BIND_IP      : &str = "127.0.0.1";
/// The default port to listen to.
const DEFAULT_PORT         : u16  = 9149;

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
    /// A url_cleaner::types::Cleaner JSON file. If none is provided, uses URL Cleaner's default cleaner.
    #[cfg(feature = "default-cleaner")]
    #[arg(long, short)]
    cleaner: Option<PathBuf>,
    /// A url_cleaner::types::Cleaner JSON file. Has to be set because this instance of URL Cleaner Site was compiled without a default cleaner.
    #[cfg(not(feature = "default-cleaner"))]
    #[arg(long, short)]
    cleaner: PathBuf,
    /// Export the cleaner after --params-diff, --flag, etc., if specified, are applied, then exit.
    #[arg(long)]
    export_cleaner: bool,
    /// A url_cleaner::types::ParamsDiff JSON file to apply to the cleaner by default.
    #[arg(long)]
    params_diff: Vec<PathBuf>,
    /// Flags to insert into the params.
    #[arg(short, long)]
    flag: Vec<String>,
    /// Vars to insert into the params.
    #[arg(short, long, num_args = 2)]
    var: Vec<Vec<String>>,
    /// Whether or not to read from the cache. If the argument is omitted, defaults to true.
    #[cfg(feature = "cache")]
    #[arg(long, default_missing_value = "true")]
    read_cache: Option<bool>,
    /// Whether or not to write to the cache. If the argument is omitted, defaults to true.
    #[cfg(feature = "cache")]
    #[arg(long, default_missing_value = "true")]
    write_cache: Option<bool>,
    /// The max size of a POST request to the `/clean` endpoint.
    ///
    /// The included userscript uses the `/get-max-json-size` endpoint to query this value and adjust its batch sizes accordingly.
    #[arg(long, default_value = DEFAULT_MAX_JSON_SIZE, value_parser = parse_byte_unit)]
    max_size: rocket::data::ByteUnit,
    /// The IP to listen to.
    #[arg(long, default_value = DEFAULT_BIND_IP, aliases = ["ip", "address"])]
    bind: IpAddr,
    /// The port to listen to.
    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,
    /// The cache to use.
    ///
    /// Defaults to "url-cleaner-site-cache.sqlite"
    #[arg(long)]
    #[cfg(feature = "cache")]
    cache: Option<CachePath>,
    /// Defaults whether or not to use cache delay in jobs that don't specify otherwise.
    #[arg(long, default_value_t = false)]
    #[cfg(feature = "cache")]
    cache_delay_default: bool,
    /// If true, makes requests, cache reads, etc. effectively single threaded to hide thread count.
    #[arg(long, default_missing_value = "true")]
    hide_thread_count: bool,
    /// Amount of threads to process tasks in.
    ///
    /// Zero uses the CPU's thread count.
    #[arg(long, default_value_t = 0)]
    threads: usize,
    /// The (optional) TLS/HTTPS cert. If specified, requires `--key`.
    #[arg(long, requires = "key")]
    cert: Option<PathBuf>,
    /// The (optional) TLS/HTTPS key. If specified, requires `--cert`.
    #[arg(long, requires = "cert")]
    key: Option<PathBuf>
}

/// The config for the server.
#[derive(Debug)]
struct ServerConfig {
    /// The [`Cleaner`] to use.
    cleaner: Cleaner<'static>,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    cache: Cache,
    /// The default value for [`Job::cache_handle_config`]'s [`CacheHandleConfig::delay`].
    #[cfg(feature = "cache")]
    cache_delay_default: bool,
    /// A [`String`] version of [`Self::cleaner`].
    cleaner_string: String,
    /// The number of threads to spawn for each [`JobConfig`].
    threads: NonZero<usize>,
    /// The default value for if [`Job::unthreader`] is [`Unthreader::No`] or [`Unthreader::Yes`].
    hide_thread_count_default: bool,
    /// The max size for a [`JobConfig`]'s JSON representation.
    max_json_size: rocket::data::ByteUnit
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
    let mut cleaner: Cleaner = serde_json::from_str(&cleaner_string).expect("The cleaner file to contain a valid Cleaner.");
    for params_diff in args.params_diff {
        serde_json::from_str::<ParamsDiff>(&std::fs::read_to_string(params_diff).expect("Reading the ParamsDiff file to a string to not error.")).expect("The read ParamsDiff file to be a valid ParamsDiff.")
            .apply_once(cleaner.params.to_mut());
    }
    cleaner.params.to_mut().flags.extend(args.flag);
    for var in args.var {
        let [name, value] = var.try_into().expect("The clap parser to work");
        cleaner.params.to_mut().vars.insert(name, value);
    }
    #[cfg(feature = "cache")]
    if let Some(read_cache) = args.read_cache {
        cleaner.params.to_mut().read_cache = read_cache;
    }
    #[cfg(feature = "cache")]
    if let Some(write_cache) = args.write_cache {
        cleaner.params.to_mut().write_cache = write_cache;
    }

    if args.export_cleaner {
        println!("{}", serde_json::to_string(&cleaner).expect("Cleaners to always serialize to JSON."));
        std::process::exit(0);
    }

    let server_state = ServerState {
        config: ServerConfig {
            #[cfg(feature = "cache")]
            cache: args.cache.unwrap_or("url-cleaner-site-cache.sqlite".into()).into(),
            #[cfg(feature = "cache")]
            cache_delay_default: args.cache_delay_default,
            cleaner,
            cleaner_string,
            threads: match args.threads {
                0 => std::thread::available_parallelism().expect("To be able to get the available parallelism."),
                1.. => NonZero::new(args.threads).expect("The 1.. pattern to mean \"not zero\"")
            },
            hide_thread_count_default: args.hide_thread_count,
            max_json_size: args.max_size
        },
        job_count: Mutex::new(0)
    };

    rocket::custom(rocket::Config {
        address: args.bind,
        port: args.port,
        limits: Limits::default().limit("json", args.max_size).limit("string", args.max_size),
        tls: args.cert.zip(args.key).map(|(cert, key)| rocket::config::TlsConfig::from_paths(cert, key)), // No unwraps.
        ..rocket::Config::default()
    }).mount("/", routes![index, clean, get_max_json_size, get_cleaner]).manage(server_state)
}

/// The `/` route.
#[get("/")]
async fn index() -> &'static str {
    r#"URL Cleaner Site is licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later).
https://www.gnu.org/licenses/agpl-3.0.html
https://github.com/Scripter17/url-cleaner"#
}

/// The `/get-cleaner` route.
#[get("/get-cleaner")]
async fn get_cleaner(state: &State<ServerState>) -> &str {
    &state.config.cleaner_string
}

/// The `/clean` route.
#[post("/clean", data="<job_config>")]
async fn clean(state: &State<ServerState>, job_config: &str) -> (Status, Json<CleanResult>) {
    match serde_json::from_str::<JobConfig>(job_config) {
        Ok(job_config) => {
            let mut cleaner = state.config.cleaner.borrowed();
            if let Some(params_diff) = job_config.params_diff {
                params_diff.apply_once(cleaner.params.to_mut());
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
                            delay: job_config.cache_delay.unwrap_or(state.config.cache_delay_default)
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

            (
                Status {code: 200},
                Json(Ok(CleanSuccess {
                    urls: ret_urls.into_inner().expect("No panics.")
                }))
            )
        },
        Err(e) => (
            Status {code: 422},
            Json(Err(CleanError {
                status: 422,
                message: e.to_string()
            }))
        )
    }
}

/// The `get-max-json-size` route.
#[get("/get-max-json-size")]
async fn get_max_json_size(state: &State<ServerState>) -> String {
    state.config.max_json_size.as_u64().to_string()
}
