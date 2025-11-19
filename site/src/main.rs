//! A basic HTTP server and userscript to allow automatically applying [URL Cleaner](https://github.com/Scripter17/url-cleaner) to every URL on every webpage you visit.

use std::net::IpAddr;
use std::path::PathBuf;
use std::fs::read_to_string;
use std::str::FromStr;
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

use url_cleaner_engine::prelude::*;
use url_cleaner_site_types::*;
use better_url::BetterUrl;

mod index;
mod info;
mod cleaner;
mod profiles;
mod clean;
mod clean_ws;

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
    /// The config file to use.
    /// Omit to use the built in bundled cleaner.
    #[cfg(feature = "bundled-cleaner")]
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    cleaner: Option<PathBuf>,
    /// The cleaner file to use.
    #[cfg(not(feature = "bundled-cleaner"))]
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    cleaner: PathBuf,
    /// The ProfilesConfig file.
    /// Cannot be used with --profiles-string.
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    profiles: Option<PathBuf>,
    /// The ProfilesConfig string.
    /// Cannot be used with --profiles.
    #[arg(long, value_name = "JSON STRING")]
    profiles_string: Option<String>,
    /// The max size of a POST request to the `/clean` endpoint.
    #[arg(long, verbatim_doc_comment, default_value = DEFAULT_MAX_PAYLOAD, value_parser = parse_byte_unit)]
    max_payload: rocket::data::ByteUnit,
    /// The IP to listen to.
    #[arg(long, verbatim_doc_comment, default_value = DEFAULT_IP)]
    ip: IpAddr,
    /// The port to listen to.
    #[arg(long, verbatim_doc_comment, default_value_t = DEFAULT_PORT)]
    port: u16,
    /// The proxy to use for HTTP/HTTPS requests.
    #[cfg(feature = "http")]
    #[arg(long, verbatim_doc_comment)]
    proxy: Option<HttpProxyConfig>,
    /// The cache to use.
    #[cfg(feature = "cache")]
    #[arg(long, verbatim_doc_comment, default_value = "url-cleaner-site-cache.sqlite", value_name = "PATH")]
    cache: PathBuf,
    /// When unthreading, also ratelimit unthreaded things.
    #[arg(long, verbatim_doc_comment, value_name = "SECONDS")]
    unthread_ratelimit: Option<f64>,
    /// Amount of threads to process tasks in.
    /// Zero uses the CPU's thread count.
    #[arg(long, verbatim_doc_comment, default_value_t = 0)]
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
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    accounts: Option<PathBuf>,
    /// The TLS/HTTPS cert. If specified, requires `--key`.
    #[arg(long, verbatim_doc_comment, requires = "key", value_name = "PATH")]
    cert: Option<PathBuf>,
    /// The TLS/HTTPS key. If specified, requires `--cert`.
    #[arg(long, verbatim_doc_comment, requires = "cert", value_name = "PATH")]
    key: Option<PathBuf>,
    /// The mTLS client's certificate.
    #[arg(long, verbatim_doc_comment, requires = "key", requires = "cert", value_name = "PATH")]
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
    /// The number of threads to spawn for each [`CleanPayload`].
    threads: NonZero<usize>,
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
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    inner_cache: InnerCache,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    http_client: HttpClient,
    /// The [`Unthreader] to use.
    unthreader: Unthreader
}

/// Make the server.
#[launch]
async fn rocket() -> _ {
    let args = Args::parse();

    #[cfg(feature = "bundled-cleaner")]
    let cleaner_string = args.cleaner.as_deref().map(|path| read_to_string(path).expect("The cleaner file to be readable.")).unwrap_or(BUNDLED_CLEANER_STR.to_string());
    #[cfg(not(feature = "bundled-cleaner"))]
    let cleaner_string = read_to_string(&args.cleaner).expect("The cleaner file to be readable.");
    // Boxing, leaking, then borrowing allows not cloning unchanged parts of the Params.
    // For my personal server this saves about 500KB.
    let cleaner = Box::leak(Box::new(serde_json::from_str::<Cleaner<'static>>(&cleaner_string).expect("The cleaner file to contain a valid Cleaner."))).borrowed();

    let profiles_config_string = match (args.profiles, args.profiles_string) {
        (None      , None        ) => "{}".into(),
        (Some(path), None        ) => std::fs::read_to_string(path).expect("The ProfilesConfig file to be readable."),
        (None      , Some(string)) => string,
        (Some(_)   , Some(_)     ) => panic!("Cannot have both --profiles and --profiles-string.")
    };
    let profiles_config = serde_json::from_str::<ProfilesConfig>(&profiles_config_string).expect("The ProfilesConfig to be a valid ProfilesConfig.");
    let profiled_cleaner = ProfiledCleanerConfig { cleaner, profiles_config }.make();

    let server_state = ServerState {
        config: ServerConfig {
            cleaner_string,
            profiled_cleaner,
            profiles_config_string,
            threads: NonZero::new(args.threads).unwrap_or_else(|| std::thread::available_parallelism().expect("To be able to get the available parallelism.")),
            max_payload: args.max_payload,
            accounts: match args.accounts {
                Some(accounts) => serde_json::from_str(&std::fs::read_to_string(accounts).expect("The accounts file to be readable.")).expect("The accounts file to be valid."),
                None => Default::default()
            }
        },
        #[cfg(feature = "cache")]
        inner_cache: args.cache.into(),
        #[cfg(feature = "http")]
        http_client: HttpClient::new(args.proxy.into_iter().map(|proxy| proxy.make()).collect::<Result<Vec<_>, _>>().expect("The proxies to be valid.")),
        unthreader: match args.unthread_ratelimit {
            None    => UnthreaderMode::Unthread,
            Some(d) => UnthreaderMode::Ratelimit(std::time::Duration::from_secs_f64(d))
        }.into()
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
        limits: Limits::default()
            .limit("json", args.max_payload)
            .limit("string", args.max_payload),
        tls,
        ..rocket::Config::default()
    }).mount("/", routes![index::index, info::info, cleaner::cleaner, profiles::profiles, clean::clean, clean_ws::clean_ws]).manage(server_state)
}
