//! URL Cleaner SIte - Explicit non-consent to URL spytext.
//!
//! See [url_cleaner_engine] to integrate URL Cleaner with your own projects.

use std::borrow::Cow;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use clap::Parser;
use axum::{
    routing::{get, any},
    Router,
    Json,
    extract::{Request, FromRequest, FromRequestParts, ws::{WebSocketUpgrade, Message}},
    body::Body,
    response::{IntoResponse, Response},
    http::{StatusCode, request::Parts},
};
use futures_util::StreamExt;
use bytes::Bytes;
use axum_server::tls_rustls::RustlsConfig;
use thiserror::Error;
use async_stream::stream;

use url_cleaner_engine::prelude::*;
use url_cleaner_site_types::prelude::*;

mod clean;

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

GET /info     to get the Info.
GET /cleaner  to get the Cleaner.
GET /profiles to get the ProfilesConfig.
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
    /// The Secrets to use.
    #[arg(long)]
    secrets: Option<PathBuf>,
    /// The ProfilesConfig to use.
    #[arg(long)]
    profiles: Option<PathBuf>,
    /// The CacheLocation to use.
    #[arg(long, default_value = "url-cleaner-site-cache.sqlite")]
    #[cfg(feature = "cache")]
    cache: PathBuf,
    /// The number of threads to use per job. 0 = CPU thread count.
    #[arg(long, default_value_t = 0)]
    threads_per_job: usize,
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

/// The state of the server.
#[derive(Debug)]
struct State {
    /// The number of worker threads to use.
    threads_per_job: usize,
    /// The [`ProfiledCleaner`].
    profiled_cleaner: ProfiledCleaner<'static>,
    /// The [`Cleaner`] string.
    cleaner_string: Cow<'static, str>,
    /// The [`ProfilesConfig`] string.
    profiles_string: Cow<'static, str>,
    /// The [`Unthreader`].
    unthreader: Unthreader,
    /// The [`Secrets`].
    secrets: Secrets,
    /// The [`InnerCache`].
    #[cfg(feature = "cache")]
    inner_cache: InnerCache,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    http_client: HttpClient,
}

/// [`main`].
#[derive(Debug, Error)]
enum SiteError {
    /** [`LoadCleanerError`].        **/ #[error(transparent)] LoadCleanerError       (#[from] LoadCleanerError       ),
    /** [`LoadParamsDiffError`].     **/ #[error(transparent)] LoadParamsDiffError    (#[from] LoadParamsDiffError    ),
    /** [`LoadProfilesConfigError`]. **/ #[error(transparent)] LoadProfilesConfigError(#[from] LoadProfilesConfigError),
    /** [`LoadJobContextError`].     **/ #[error(transparent)] LoadJobContextError    (#[from] LoadJobContextError    ),
    /** [`LoadSecretsError`].        **/ #[error(transparent)] LoadSecretsError       (#[from] LoadSecretsError       ),

    /// [`RustlsConfig::from_pem_file`].
    #[error(transparent)]
    LoadTlsError(std::io::Error),
    /// [`axum_server::bind_rustls`]/[`axum_server::bind`].
    #[error(transparent)]
    FatalError(std::io::Error),
}

/// The [`Cleaner`].
static CLEANER: OnceLock<Cleaner<'static>> = OnceLock::new();
/// The [`State`].
static STATE  : OnceLock<State           > = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), SiteError> {
    let args = Args::parse();

    let addr = std::net::SocketAddr::new(args.ip, args.port);

    println!("{WELCOME}");
    println!();
    match args.key.is_some() {
        true  => println!("https://{addr}"),
        false => println!("http://{addr}" ),
    }

    let (cleaner_string, cleaner) = cfg_select! {
        feature = "bundled-cleaner" => Cleaner::load_or_new_bundled(args.cleaner)?,
        _                           => {{let (x, y) = Cleaner::load(args.cleaner)?; (x.into(), y)}},
    };

    let cleaner = CLEANER.get_or_init(|| cleaner);

    let (profiles_string, profiles) = ProfilesConfig::load_or_default(args.profiles)?;

    let profiled_cleaner = profiles.make(cleaner);

    let secrets = Secrets::load_or_default(args.secrets)?;

    let threads_per_job = match args.threads_per_job {
        0 => std::thread::available_parallelism().expect("To be able to get the available parallelism.").into(),
        x => x,
    };

    let state: &'static State = STATE.get_or_init(|| State {
        threads_per_job,
        profiled_cleaner,
        cleaner_string,
        profiles_string,
        unthreader: Unthreader::on(),
        secrets,
        #[cfg(feature = "cache")]
        inner_cache: args.cache.into(),
        #[cfg(feature = "http")]
        http_client: Default::default(),
    });

    let app = Router::new()
        .route("/"    , get(async || WELCOME))
        .route("/info", get(async || Json(Info {
            source_code: env!("CARGO_PKG_REPOSITORY").into(),
            version    : env!("CARGO_PKG_VERSION"   ).into(),
            auth_mode  : state.secrets.auth_info.mode(),
        })))
        .route("/cleaner" , get(async || &*state.cleaner_string ))
        .route("/profiles", get(async || &*state.profiles_string))
        .route("/clean"   , any(clean::clean))
        .with_state(state).into_make_service();

    match args.key.zip(args.cert) {
        Some((key, cert)) => axum_server::bind_rustls(addr, RustlsConfig::from_pem_file(cert, key).await.map_err(SiteError::LoadTlsError)?).serve(app).await.map_err(SiteError::FatalError)?,
        None              => axum_server::bind       (addr                                                                                ).serve(app).await.map_err(SiteError::FatalError)?,
    }

    Ok(())
}
