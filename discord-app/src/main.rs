//! A discord app for URL Cleaner

use std::path::PathBuf;
use std::borrow::Cow;

use clap::Parser;

use url_cleaner_engine::prelude::*;

mod ready_handler;
mod help;
mod clean_url;
mod clean_urls;

mod prelude {
    pub use poise::serenity_prelude as serenity;

    pub use super::ready_handler::*;
    pub use super::help::*;
    pub use super::clean_url::*;
    pub use super::clean_urls::*;

    pub use super::{Error, Context};
}
use prelude::*;

/// The introduction to the /help message.
const INFO: &str = concat!(r#"URL Cleaner Discord App
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html
"#, env!("CARGO_PKG_REPOSITORY"));

#[allow(rustdoc::bare_urls, reason = "It'd look bad in the console.")]
/// URL Cleaner Discord App - Explicit non-consent to URL spytext.
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
    /// The cleaner file to use.
    /// Omit to use the built in bundled cleaner.
    #[cfg(feature = "bundled-cleaner")]
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    cleaner: Option<PathBuf>,
    /// The cleaner file to use.
    #[cfg(not(feature = "bundled-cleaner"))]
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    cleaner: PathBuf,

    /// The ProfilesConfig file.
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    profiles: Option<PathBuf>,

    /// The proxy to use for HTTP/HTTPS requests.
    #[cfg(feature = "http")]
    #[arg(long, verbatim_doc_comment)]
    proxy: Option<HttpProxyConfig>,

    /// The path cache to use.
    #[cfg(feature = "cache")]
    #[arg(long, verbatim_doc_comment, value_name = "PATH", default_value = "url-cleaner-discord-app-cache.sqlite")]
    cache: PathBuf,
    /// If true, read from the cache.
    #[cfg(feature = "cache")]
    #[arg(long, verbatim_doc_comment)]
    no_read_cache: bool,
    /// If true, write to the cache.
    #[cfg(feature = "cache")]
    #[arg(long, verbatim_doc_comment)]
    no_write_cache: bool,
    /// If true, artificially delay cache reads.
    #[cfg(feature = "cache")]
    #[arg(long, verbatim_doc_comment)]
    cache_delay: bool
}

/// The bot's state.
#[derive(Debug)]
pub struct State {
    /// The [`Cleaner`] used to make the [`ProfiledCleaner`] as a string.
    cleaner_string: String,
    /// The [`ProfilesConfig`] used to make the [`ProfiledCleaner`] as a string.
    profiles_config_string: String,
    /// The [`ProfiledCleaner`].
    profiled_cleaner: &'static ProfiledCleaner<'static>,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    cache: Cache<'static>,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    http_client: HttpClient
}

/// The error type.
pub type Error = Box<dyn std::error::Error + Send + Sync>;
/// The context type.
pub type Context<'a> = poise::Context<'a, State, Error>;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("{INFO}");

    let mut commands = vec![help(), clean_url()];

    #[cfg(feature = "bundled-cleaner")]
    let cleaner_string = match args.cleaner {
        Some(path) => std::fs::read_to_string(path).expect("The Cleaner file to be readable."),
        None       => BUNDLED_CLEANER_STR.into()
    };
    #[cfg(not(feature = "bundled-cleaner"))]
    let cleaner_string = std::fs::read_to_string(args.cleaner).expect("The Cleaner file to be readable.");
    let cleaner = Box::leak(Box::new(serde_json::from_str::<Cleaner<'static>>(&cleaner_string).expect("The Cleaner string to be valid."))).borrowed();

    let profiles_config_string = match args.profiles {
        None       => "{}".to_string(),
        Some(path) => std::fs::read_to_string(path).expect("The ProfilesConfig file to be readable."),
    };
    let profiles_config: ProfilesConfig = serde_json::from_str(&profiles_config_string).expect("The ProfilesConfig to be valid.");
    let profiled_cleaner = Box::leak(Box::new(ProfiledCleanerConfig { cleaner, profiles_config }.make()));

    let state = State {
        cleaner_string,
        profiles_config_string,
        profiled_cleaner,
        #[cfg(feature = "cache")]
        cache: Cache {
            config: CacheConfig {
                read : !args.no_read_cache,
                write: !args.no_write_cache,
                delay:  args.cache_delay
            },
            inner: Box::leak(Box::new(args.cache.into()))
        },
        #[cfg(feature = "http")]
        http_client: HttpClient::new(args.proxy.into_iter().map(|proxy| proxy.make()).collect::<Result<Vec<_>, _>>().expect("The proxies to be valid."))
    };

    for (name, cleaner) in state.profiled_cleaner.get_each() {
        commands.push(poise::Command {
            name: match name {
                Some(name) => Cow::Owned(format!("Clean URLs ({name})")),
                None => Cow::Borrowed("Clean URLs")
            },
            context_menu_action: Some(poise::structs::ContextMenuCommandAction::Message(|ctx: poise::structs::ApplicationContext<'_, State, Error>, msg: serenity::Message| {
                Box::pin(async move {
                    clean_urls(
                        ctx.into(),
                        msg,
                        ctx.command().custom_data.downcast_ref::<Cleaner>().expect("The custom data to be a Cleaner")
                    ).await.map_err(|error| poise::FrameworkError::new_command(ctx.into(), error.into()))
                })
            })),
            custom_data: Box::new(cleaner),
            install_context: Some(vec![serenity::InstallationContext::User, serenity::InstallationContext::Guild]),
            interaction_context: Some(vec![serenity::InteractionContext::Guild, serenity::InteractionContext::BotDm, serenity::InteractionContext::PrivateChannel]),
            ..Default::default()
        });
    }

    let token = std::env::var("URLCDA_TOKEN").expect("No discord app token found in the URLCDA_TOKEN environment variable.");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(state)
            })
        })
        .build();

    serenity::ClientBuilder::new(token, intents)
        .event_handler(ReadyHandler)
        .framework(framework)
        .await.expect("Making the client failed.")
        .start().await.expect("Starting the app failed. Maybe the app token in the URLCDA_TOKEN environment variable was invalid?");
}
