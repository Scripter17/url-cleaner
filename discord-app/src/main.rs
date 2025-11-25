//! A discord app for URL Cleaner

use std::sync::{OnceLock, LazyLock};
use std::path::PathBuf;
use std::borrow::Cow;

use clap::Parser;
use ::regex::Regex;
use poise::serenity_prelude as serenity;
use poise::reply::CreateReply;
use serenity::Ready;
use serenity::EventHandler;
use serenity::CreateAttachment;
use thiserror::Error;

use url_cleaner_engine::prelude::*;

/// The introduction to the /help message.
const INTRO: &str = r#"URL Cleaner Discord App
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html"#;
/// The link to the source code of the bot.
const SOURCE_CODE_URL: &str = env!("CARGO_PKG_REPOSITORY");
/// The info to install the bot to your account/sever.
static INSTALL_INFO: OnceLock<String> = OnceLock::new();
/// The tutorial for using the bot.
const TUTORIAL: &str = r#"To clean the URLs in a message, right click/long press a message, go to the apps, and click any of the available \"Clean URLs\" actions"#;

/// Basic URL getting regex.
///
/// Does not account for code blocks, spoilers, etc.
static GET_URLS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[[^\]]+\]\((?<URL1>[^)]+)\)|(?<URL2>\w+:\/\/\S+)").expect("The URL parsing Regex to be valid."));

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
struct State {
    /// The [`Cleaner`] used to make the [`ProfiledCleaner`] as a string.
    cleaner_string: String,
    /// The [`ProfilesConfig`] used to make the [`ProfiledCleaner`] as a string.
    profiles_config_string: String,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    cache: Cache<'static>,
    /// The [`HttpClient`].
    #[cfg(feature = "http")]
    http_client: HttpClient
}

/// The error type.
type Error = Box<dyn std::error::Error + Send + Sync>;
/// The context type.
type Context<'a> = poise::Context<'a, State, Error>;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut commands = vec![help()];

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

    let profiled_cleaner = ProfiledCleanerConfig { cleaner, profiles_config }.make();

    for (name, cleaner) in profiled_cleaner.into_each() {
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

    let state = State {
        cleaner_string,
        profiles_config_string,
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

/// An [`EventHandler`] that prints license info and the app's authorization URL on [`EventHandler::ready`].
struct ReadyHandler;

#[serenity::async_trait]
impl EventHandler for ReadyHandler {
    async fn ready(&self, ctx: serenity::Context, data_about_bot: Ready) {
        let install_info = format!(
            r#"Install to your account: https://discord.com/oauth2/authorize?client_id={0}
Install to a server: https://discord.com/oauth2/authorize?client_id={0}&scope=bot"#,
            data_about_bot.application.id
        );

        println!("{INTRO}\n{SOURCE_CODE_URL}\n\n{install_info}\n\n{TUTORIAL}");

        INSTALL_INFO.set(install_info).expect("INSTALL_INFO to only be set once.");

        ctx.set_activity(Some(serenity::gateway::ActivityData::custom(SOURCE_CODE_URL)));
    }
}

#[poise::command(slash_command)]
async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let message = if ctx.http().get_current_application_info().await.expect("Getting the current application info to work.").bot_public {
        format!("{INTRO}\n{SOURCE_CODE_URL}\n\n{}\n\n{TUTORIAL}", INSTALL_INFO.get().expect("INSTALL_INFO to have been set."))
    } else {
        format!("{INTRO}\n{SOURCE_CODE_URL}\n\nThis specific instance of URL Cleaner Discord App is private.\n\n{TUTORIAL}")
    };

    ctx.send(CreateReply::default()
        .ephemeral(true)
        .content(message)
        .attachment(CreateAttachment::bytes(ctx.data().cleaner_string        .clone(), "cleaner.json" ).description("The Cleaner this bot is using."))
        .attachment(CreateAttachment::bytes(ctx.data().profiles_config_string.clone(), "profiles.json").description("The ProfilesConfig this bot is using."))
    ).await?;
    Ok(())
}

/// The enum of errors [`clean_urls`] can return.
#[derive(Debug, Error)]
pub enum CleanUrlsError {
    /// Returned when attempting to use an unknown profile.
    #[error("Unknown profile.")]
    UnknownProfile,
    /// Returned when a [`serenity::Error`] is encountered.
    #[error(transparent)]
    SerenityError(#[from] serenity::Error)
}

/// Clean a message's URLs with the specified [`Params`].
async fn clean_urls(ctx: Context<'_>, msg: serenity::Message, cleaner: &Cleaner<'_>) -> Result<(), CleanUrlsError> {
    let job = Job {
        config: JobConfig {
            context: &Default::default(),
            cleaner,
            unthreader: &Unthreader::default(),
            #[cfg(feature = "cache")]
            cache: ctx.data().cache,
            #[cfg(feature = "http")]
            http_client: &ctx.data().http_client
        },
        tasks: GET_URLS.captures_iter(&msg.content).map(|x| Ok(x.name("URL1").or(x.name("URL2")).expect("The regex to always match at least one.").as_str().into()))
    };

    let mut responses = Vec::new();
    for task in job {
        match task.expect("Making the LazyTask to work").make().expect("Making the Task to work").r#do() {
            Ok(url) => responses.push(url.into()),
            Err(e) => responses.push(format!("Error: {e:?}"))
        }
    }

    let content = if responses.is_empty() {
        "No URLs found".to_string()
    } else {
        responses.join("\n")
    };

    ctx.send(CreateReply::default().ephemeral(true).content(content)).await?;
    Ok(())
}
