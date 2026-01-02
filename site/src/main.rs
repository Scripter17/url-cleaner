//! URL Cleaner SIte - Explicit non-consent to URL spytext.
//!
//! See [url_cleaner_engine] to integrate URL Cleaner with your own projects.

use std::net::IpAddr;
use std::path::PathBuf;
use std::num::NonZero;
use std::collections::HashSet;
use axum_server::tls_rustls::RustlsConfig;

use clap::Parser;
use axum::{
    routing::{get, post},
    Router,
    Json
};

use url_cleaner_engine::prelude::*;
use url_cleaner_site_types::prelude::*;

mod clean;
mod clean_ws;

/// The verson.
const VERSION   : &str = env!("CARGO_PKG_VERSION");
/// The repository.
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

/// The welcome message.
const WELCOME: &str = const_str::format!(
r#"URL Cleaner Site {VERSION}

Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html

{REPOSITORY}

See /info     to get the Info.
See /cleaner  to get the Cleaner.
See /profiles to get the ProfilesConfig.
"#);

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
    /// The Cleaner to use.
    #[cfg(feature = "bundled-cleaner")]
    #[arg(long)]
    cleaner: Option<PathBuf>,
    /// The Cleaner to use.
    #[cfg(not(feature = "bundled-cleaner"))]
    #[arg(long)]
    cleaner: PathBuf,
    /// The ProfilesConfig to use.
    #[arg(long)]
    profiles: Option<PathBuf>,
    /// The CacheLocation to use.
    #[arg(long, default_value = "url-cleaner-site-cache.sqlite")]
    #[cfg(feature = "cache")]
    cache: PathBuf,
    /// The number of worker threads to use per job.
    #[arg(long, default_value = "cpu-threads")]
    worker_threads: WorkerThreads,
    /// The passwords to use.
    #[arg(long)]
    passwords: Option<PathBuf>,
    /// The IP to bind to.
    #[arg(long, default_value = "127.0.0.1")]
    ip: IpAddr,
    /// The port to bind to.
    #[arg(long, default_value_t = 9149)]
    port: u16,
    /// The TLS key.
    #[arg(long, requires = "cert")]
    key: Option<PathBuf>,
    /// The TLS certificate.
    #[arg(long, requires = "key")]
    cert: Option<PathBuf>
}

/// Configuration on how many worker threads to use.
#[derive(Debug, Clone)]
enum WorkerThreads {
    /// Use as many as the CPU has threads.
    ///
    /// Parses from `cpu-threads`.
    CpuThreads,
    /// Use the specified amount.
    ///
    /// Parses from anything else.
    Literal(NonZero<usize>)
}

impl std::str::FromStr for WorkerThreads {
    type Err = std::num::ParseIntError;

    /// Parses `cpu-threads` to [`Self::CpuThreads`] and everything else to [`Self::Literal`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "cpu-threads" => Self::CpuThreads,
            _             => Self::Literal(s.parse()?)
        })
    }
}

/// The state of the server.
#[derive(Debug)]
struct State {
    /// The [`ProfiledCleaner`].
    profiled_cleaner: ProfiledCleaner<'static>,
    /// The [`Cleaner`] string.
    cleaner_string: String,
    /// The [`ProfilesConfig`] string.
    profiles_string: String,
    /// The [`InnerCache`].
    #[cfg(feature = "cache")]
    inner_cache: InnerCache,
    /// The [`Unthreader`].
    unthreader: Unthreader,
    /// The number of worker threads to use.
    worker_threads: NonZero<usize>,
    /// The passwords.
    passwords: Option<HashSet<String>>
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    #[cfg(feature = "bundled-cleaner")]
    let cleaner_string = match args.cleaner {
        Some(path) => std::fs::read_to_string(path).expect("The Cleaner to be readable."),
        None       => BUNDLED_CLEANER_STR.into()
    };
    #[cfg(not(feature = "bundled-cleaner"))]
    let cleaner_string = std::fs::read_to_string(args.cleaner).expect("The Cleaner to be readable.");

    let cleaner = Box::leak(Box::new(serde_json::from_str::<Cleaner>(&cleaner_string).expect("The Cleaner to be valid."))).borrowed();

    let profiles_string = match args.profiles {
        Some(path) => std::fs::read_to_string(path).expect("The ProfilesConfig to be readable."),
        None       => "{}".into()
    };

    let profiled_cleaner = ProfiledCleanerConfig {
        cleaner,
        profiles_config: serde_json::from_str(&profiles_string).expect("The ProfilesConfig to be valid.")
    }.make();

    let state: &'static State = Box::leak(Box::new(State {
        profiled_cleaner,
        cleaner_string,
        profiles_string,
        #[cfg(feature = "cache")]
        inner_cache: args.cache.into(),
        unthreader: Unthreader::on(),
        worker_threads: match args.worker_threads {
            WorkerThreads::CpuThreads => std::thread::available_parallelism().expect("To be able to get the available parallelism."),
            WorkerThreads::Literal(x) => x
        },
        passwords: args.passwords.map(|path| serde_json::from_str::<HashSet<String>>(&std::fs::read_to_string(path).expect("The passwords file to be readable.")).expect("The passwords file to be valid")),
    }));

    let app = Router::new()
        .route("/"        , get(async || WELCOME))
        .route("/info"    , get(async || Json(Info {
            source_code: env!("CARGO_PKG_REPOSITORY").into(),
            version    : env!("CARGO_PKG_VERSION"   ).into(),
            password_required: false
        })))
        .route("/cleaner" , get(async || &*state.cleaner_string))
        .route("/profiles", get(async || &*state.profiles_string))
        .route("/clean"   , post(clean::clean))
        .route("/clean_ws", get(clean_ws::clean_ws))
        .with_state(state);

    let addr = std::net::SocketAddr::new(args.ip, args.port);

    println!("{WELCOME}");
    println!("Bound to {addr:?}");

    let tls_config = match args.key.zip(args.cert) {
        Some((key, cert)) => Some(RustlsConfig::from_pem_file(cert, key).await.expect("To be able to load the TLS key and cert.")),
        None              => None
    };
    match tls_config {
        Some(tls_config) => axum_server::bind_rustls(addr, tls_config).serve(app.into_make_service()).await.expect("The server to exit gracefully"),
        None             => axum_server::bind       (addr            ).serve(app.into_make_service()).await.expect("The server to exit gracefully")
    };
}
