//! A basic HTTP server and userscript to allow automatically applying [URL Cleaner](https://github.com/Scripter17/url-cleaner) to every URL on every webpage you visit.

use std::net::IpAddr;
use std::path::PathBuf;
use std::fs::read_to_string;
use std::str::FromStr;
use std::num::NonZero;
use std::collections::HashSet;

#[macro_use] extern crate rocket;
use rocket::{
    serde::json::Json,
    data::Limits,
    State,
    response::Responder,
    http::{ContentType, MediaType}
};
use clap::Parser;

use url_cleaner_engine::prelude::*;
use url_cleaner_site_types::prelude::*;

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
    /// The ProfilesConfig file to use.
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    profiles: Option<PathBuf>,
    /// The passwords file to use.
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    passwords: Option<PathBuf>,
    /// The IP to listen to.
    #[arg(long, verbatim_doc_comment, default_value = DEFAULT_IP)]
    ip: IpAddr,
    /// The port to listen to.
    #[arg(long, verbatim_doc_comment, default_value_t = DEFAULT_PORT)]
    port: u16,
    /// The max size of a POST request to the `/clean` endpoint.
    #[arg(long, verbatim_doc_comment, default_value = DEFAULT_MAX_PAYLOAD, value_parser = parse_byte_unit)]
    max_payload: rocket::data::ByteUnit,
    /// The cache to use.
    #[cfg(feature = "cache")]
    #[arg(long, verbatim_doc_comment, default_value = "url-cleaner-site-cache.sqlite", value_name = "PATH")]
    cache: PathBuf,
    /// Amount of threads to process tasks in.
    /// Zero uses the CPU's thread count.
    #[arg(long, verbatim_doc_comment, default_value_t = 0)]
    threads: usize,
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
    /// The passwords.
    passwords: Option<HashSet<String>>,
    /// The number of threads to spawn for clean.
    threads: NonZero<usize>,
    /// The max size for a clean payload.
    max_payload: rocket::data::ByteUnit
}

/// The state of the server.
#[derive(Debug)]
struct ServerState {
    /// The [`Unthreader`] to use.
    unthreader: Unthreader,
    /// The [`ServerConfig`] to use.
    config: ServerConfig,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    inner_cache: InnerCache,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    http_client: HttpClient
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

    let profiles_config_string = match args.profiles {
        None       => "{}".into(),
        Some(path) => std::fs::read_to_string(path).expect("The ProfilesConfig file to be readable."),
    };
    let profiles_config = serde_json::from_str::<ProfilesConfig>(&profiles_config_string).expect("The ProfilesConfig to be a valid ProfilesConfig.");
    let profiled_cleaner = ProfiledCleanerConfig { cleaner, profiles_config }.make();

    let passwords = args.passwords.map(|path| serde_json::from_str(&std::fs::read_to_string(path).expect("The passwords file to be readable.")).expect("The passwords file to be valid."));

    let state: &'static ServerState = Box::leak(Box::new(ServerState {
        config: ServerConfig {
            cleaner_string,
            profiled_cleaner,
            profiles_config_string,
            passwords,
            threads: NonZero::new(args.threads).unwrap_or_else(|| std::thread::available_parallelism().expect("To be able to get the available parallelism.")),
            max_payload: args.max_payload
        },
        unthreader: Unthreader::on(),
        #[cfg(feature = "cache")]
        inner_cache: args.cache.into(),
        #[cfg(feature = "http")]
        http_client: HttpClient::new(),
    }));

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
        limits: Limits::default().limit("bytes", args.max_payload).limit("string", args.max_payload),
        tls,
        ..rocket::Config::default()
    })
        .mount("/", routes![index::index, info::info, cleaner::cleaner, profiles::profiles, clean::clean, clean_ws::clean_ws])
        .manage(state)
}
