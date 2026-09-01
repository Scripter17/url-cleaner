//! Run URL Cleaner Site.

use std::num::NonZero;
use std::borrow::Cow;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use clap::Parser;
use serde::Serialize;
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

mod clean;
mod userscript;

/** The verson.     **/ const VERSION   : &str = env!("CARGO_PKG_VERSION"   );
/** The repository. **/ const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

/// The welcome message.
const WELCOME: &str = const_str::format!(
r#"URL Cleaner Site {VERSION}

GET /info       to get the Info.
GET /cleaner    to get the Cleaner.
GET /profiles   to get the ProfilesConfig.
GET /userscript to get the userscript.

POST/PUT/WebSocket /clean to clean URLs.

Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html

{REPOSITORY}
"#);

/// Run URL Cleaner Site.
#[derive(Debug, Parser)]
pub struct Args {
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

    /// The Secrets to use.
    #[arg(long, value_name = "PATH")]
    secrets: Option<PathBuf>,

    /// Disable the HTTP client.
    #[cfg(feature = "http")]
    #[arg(long, short = 'H')]
    no_http: bool,

    /// The CacheTarget to use.
    #[cfg(feature = "cache")]
    #[arg(long, default_value = "url-cleaner-site.sqlite")]
    cache: CacheTarget,

    /// The number of worker threads to use for each job.
    #[arg(long, short = 't')]
    threads: Option<NonZero<usize>>,

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
pub struct State {
    /// The [`Info`].
    info: Info,
    /// The number of worker threads to use.
    threads: NonZero<usize>,
    /// The [`ProfiledCleaner`].
    profiled_cleaner: ProfiledCleaner<'static>,
    /// The [`Cleaner`] string.
    cleaner_string: Cow<'static, str>,
    /// The [`ProfilesConfig`] string.
    profiles_string: Cow<'static, str>,
    /// The [`Secrets`].
    secrets: Secrets,
    /// If TLS is being used.
    tls: bool,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    http_client: Option<HttpClient>,
    /// The [`CacheClient`].
    #[cfg(feature = "cache")]
    cache_client: CacheClient,
}

/// [`Args::do`].
#[derive(Debug, Error)]
pub enum RunError {
    /** [`LoadCleanerError`].        **/ #[error(transparent)] LoadCleanerError       (#[from] LoadCleanerError       ),
    /** [`LoadParamsDiffError`].     **/ #[error(transparent)] LoadParamsDiffError    (#[from] LoadParamsDiffError    ),
    /** [`LoadProfilesConfigError`]. **/ #[error(transparent)] LoadProfilesConfigError(#[from] LoadProfilesConfigError),
    /** [`LoadJobContextError`].     **/ #[error(transparent)] LoadJobContextError    (#[from] LoadJobContextError    ),
    /** [`LoadSecretsError`].        **/ #[error(transparent)] LoadSecretsError       (#[from] LoadSecretsError       ),

    /** [`RustlsConfig::from_pem_file`]. **/ #[error(transparent)] LoadTlsError(std::io::Error),
    /** [`axum_server::Server::serve`].  **/ #[error(transparent)] ServeError  (std::io::Error),
}

/** The [`Cleaner`]. **/ static CLEANER: OnceLock<Cleaner<'static>> = OnceLock::new();
/** The [`State`].   **/ static STATE  : OnceLock<State           > = OnceLock::new();

/// Info about the instance.
#[derive(Debug, Serialize)]
struct Info {
    /** The version.                       **/ version          : &'static str,
    /** The link to the source code.       **/ source_code      : &'static str,
    /** If `/clean` requires a password.   **/ requires_password: bool,
    /** If the `http` feature is enabled.  **/ supports_http    : bool,
    /** If the `cache` feature is enabled. **/ supports_cache   : bool,
}

impl Args {
    /// Do the command.
    pub async fn r#do(self) -> Result<(), RunError> {
        let addr = std::net::SocketAddr::new(self.ip, self.port);

        let (cleaner_string, cleaner) = cfg_select! {
            feature = "bundled-cleaner" => Cleaner::load_or_new_bundled(self.cleaner)?,
            _                           => {{let (x, y) = Cleaner::load(self.cleaner)?; (x.into(), y)}},
        };

        let cleaner = CLEANER.get_or_init(|| cleaner);

        let (profiles_string, profiles) = ProfilesConfig::load_or_default(self.profiles)?;

        let profiled_cleaner = profiles.make(cleaner);

        let secrets = Secrets::load_or_default(self.secrets)?;

        let threads = self.threads.unwrap_or_else(|| std::thread::available_parallelism().expect("To be able to get the available parallelism."));

        #[cfg(feature = "http" )] let http_client  = HttpClient ::new(          ).await;
        #[cfg(feature = "cache")] let cache_client = CacheClient::new(self.cache).await;

        let state = STATE.get_or_init(|| State {
            info: Info {
                version          : VERSION,
                source_code      : REPOSITORY,
                requires_password: secrets.requires_password(),
                supports_http    : cfg_select!(feature = "http"  => true, _ => false),
                supports_cache   : cfg_select!(feature = "cache" => true, _ => false),
            },
            threads,
            profiled_cleaner,
            cleaner_string,
            profiles_string,
            secrets,
            tls: self.key.is_some(),
            #[cfg(feature = "http" )] http_client: (!self.no_http).then_some(http_client),
            #[cfg(feature = "cache")] cache_client,
        });

        let app = Router::new()
            .route("/"          , get(async || WELCOME))
            .route("/info"      , get(async |state: &'static State| Json(&state.info)))
            .route("/cleaner"   , get(async |state: &'static State| &*state.cleaner_string ))
            .route("/profiles"  , get(async |state: &'static State| &*state.profiles_string))
            .route("/clean"     , any(clean::clean))
            .route("/userscript", get(userscript::userscript))
            .with_state(state).into_make_service();

        println!("{WELCOME}");

        match state.tls {
            true  => println!("https://{addr}"),
            false => println!("http://{addr}" ),
        }

        match self.key.zip(self.cert) {
            Some((key, cert)) => axum_server::bind_rustls(addr, RustlsConfig::from_pem_file(cert, key).await.map_err(RunError::LoadTlsError)?).serve(app).await.map_err(RunError::ServeError)?,
            None              => axum_server::bind       (addr                                                                               ).serve(app).await.map_err(RunError::ServeError)?,
        }

        Ok(())
    }
}
