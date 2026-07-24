//! URL Cleaner SIte - Explicit non-consent to URL spytext.
//!
//! See [url_cleaner_engine] to integrate URL Cleaner with your own projects.

use clap::Parser;
use thiserror::Error;

mod run;
mod keygen;

#[allow(rustdoc::bare_urls, reason = "It'd look bad in the console.")]
/// URL Cleaner Site - Explicit non-consent to URL spytext.
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
#[allow(clippy::missing_docs_in_private_items, reason = "Makes Clap propogate docs.")]
enum Args {
    Run   (run   ::Args),
    Keygen(keygen::Args),
}

/// [`main`]
#[derive(Debug, Error)]
enum SiteError {
    /** [`run::RunError`],       **/ #[error(transparent)] RunError   (#[from] run   ::RunError   ),
    /** [`keygen::KeygenError`], **/ #[error(transparent)] KeygenError(#[from] keygen::KeygenError),
}

#[tokio::main]
async fn main() -> Result<(), SiteError> {
    match Args::parse() {
        Args::Run   (args) => args.r#do().await?,
        Args::Keygen(args) => args.r#do().await?,
    }

    Ok(())
}
