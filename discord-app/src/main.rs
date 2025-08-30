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
use thiserror::Error;

use url_cleaner_engine::types::*;
use url_cleaner_engine::glue::*;
use url_cleaner_engine::helpers::*;

/// The source code of this instance of the bot.
static SOURCE_CODE_URL: &str = "https://github.com/Scripter17/url-cleaner";

/// Basic URL getting regex.
///
/// Does not account for code blocks, spoilers, etc.
static GET_URLS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[[^\]]+\]\((?<URL1>[^)]+)\)|(?<URL2>\w+:\/\/\S+)").expect("The URL parsing Regex to be valid."));
/// The help message.
static HELP_MESSAGE: OnceLock<String> = OnceLock::new();

#[allow(rustdoc::bare_urls, reason = "It'd look bad in the console.")]
/// A discord app for URL Cleaner.
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
    #[arg(long)]
    #[cfg(feature = "default-cleaner")]
    cleaner: Option<PathBuf>,
    /// The config file to use.
    ///
    /// Omit to use the built in default cleaner.
    #[arg(long)]
    #[cfg(not(feature = "default-cleaner"))]
    cleaner: PathBuf,
    /// Export the cleaner after --params-diff, --flag, etc., if specified, are applied, then exit.
    #[arg(long)]
    export_cleaner: bool,
    /// The ParamsDiff profiles.
    #[arg(long)]
    profiles: Option<PathBuf>,
    /// The cache to use.
    ///
    /// Defaults to "url-cleaner-discord-app-cache.sqlite"
    #[cfg(feature = "cache")]
    #[arg(long)]
    cache: Option<CachePath>,
    /// Artificially delay cache reads about as long as the initial run to defend against cache detection.
    #[cfg(feature = "cache")]
    #[arg(long, default_missing_value = "true")]
    cache_delay: bool,
    /// Whether or not to read from the cache. If unspecified, defaults to true.
    #[cfg(feature = "cache")]
    #[arg(long, default_value = "true")]
    read_cache: Option<bool>,
    /// Whether or not to write to the cache. If unspecified, defaults to true.
    #[cfg(feature = "cache")]
    #[arg(long, default_value = "true")]
    write_cache: Option<bool>
}

/// The bot's state.
#[derive(Debug)]
struct State {
    /// The [`Cleaner`] to use.
    cleaner: ProfiledCleaner<'static>,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    cache: Cache,
    /// [`CacheHandleConfig`]
    #[cfg(feature = "cache")]
    cache_handle_config: CacheHandleConfig
}

/// The error type.
type Error = Box<dyn std::error::Error + Send + Sync>;
/// The context type.
type Context<'a> = poise::Context<'a, State, Error>;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut commands = vec![help(), clean_urls()];

    #[cfg(feature = "default-cleaner")]
    let cleaner = Cleaner::load_or_get_default_no_cache(args.cleaner.as_deref()).expect("The cleaner to be valid.");
    #[cfg(not(feature = "default-cleaner"))]
    let cleaner = Cleaner::load_from_file(args.cleaner).expect("The cleaner to be valid.");

    if args.export_cleaner {
        println!("{}", serde_json::to_string(&cleaner).expect("Cleaners to always serialize to JSON."));
        std::process::exit(0);
    }

    let profiles_config = match args.profiles {
        Some(file) => serde_json::from_str::<ProfilesConfig>(&std::fs::read_to_string(file).expect("Reading the ProfilesConfig file to a string to not error."))
            .expect("The read ProfilesConfig file to be a valid ParamsDiff."),
        None => Default::default()
    };
    let cleaner = cleaner.with_profiles(profiles_config);

    let mut profile_names = cleaner.profiles().names().map(String::from).collect::<Vec<_>>();
    profile_names.sort();

    for profile in profile_names {
        commands.push(poise::Command {
            name: Cow::Owned(format!("Clean URLs ({profile})")),
            context_menu_action: Some(poise::structs::ContextMenuCommandAction::Message(|ctx: poise::structs::ApplicationContext<'_, State, Error>, msg: serenity::Message| {
                Box::pin(async move {
                    clean_urls_with_profile(
                        ctx.into(),
                        msg,
                        Some(ctx.command().custom_data.downcast_ref::<String>().expect("The custom data to be a str"))
                    ).await.map_err(|error| poise::FrameworkError::new_command(ctx.into(), error.into()))
                })
            })),
            custom_data: Box::new(profile),
            install_context: Some(vec![serenity::InstallationContext::User, serenity::InstallationContext::Guild]),
            interaction_context: Some(vec![serenity::InteractionContext::Guild, serenity::InteractionContext::BotDm, serenity::InteractionContext::PrivateChannel]),
            ..Default::default()
        });
    }

    let state = State {
        cleaner,
        #[cfg(feature = "cache")]
        cache: args.cache.unwrap_or("url-cleaner-discord-app-cache.sqlite".into()).into(),
        #[cfg(feature = "cache")]
        cache_handle_config: CacheHandleConfig {
            delay: args.cache_delay,
            read : args.read_cache .unwrap_or(true),
            write: args.write_cache.unwrap_or(true)
        }
    };

    let token = std::env::var("URLCDA_KEY").expect("No discord app token found in the URLCDA_KEY environment variable.");
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
        .start().await.expect("Starting the app failed. Maybe the app token in the URLCDA_KEY environment variable was invalid?");
}

/// An [`EventHandler`] that prints license info and the app's authorization URL on [`EventHandler::ready`].
struct ReadyHandler;

#[serenity::async_trait]
impl EventHandler for ReadyHandler {
    async fn ready(&self, ctx: serenity::Context, data_about_bot: Ready) {
        let install_info = if ctx.http.get_current_application_info().await.expect("Getting the current application info to work").bot_public {
            format!(r#"Install to your account: https://discord.com/oauth2/authorize?client_id={0}
Install to a server: https://discord.com/oauth2/authorize?client_id={0}&scope=bot"#, data_about_bot.application.id)
        } else {
            "This instance is private, but you can host your own using the above link".to_string()
        };

        HELP_MESSAGE.set(format!(r#"URL Cleaner Discord App
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html
{SOURCE_CODE_URL}

{install_info}

To clean the URLs in a message, right click a message, go to the apps, and click any of the available \"Clean URLs\" actions"#)).expect("HELP_MESSAGE to only be set once.");

        ctx.set_activity(Some(serenity::gateway::ActivityData {
            name: SOURCE_CODE_URL.to_string(),
            kind: serenity::model::gateway::ActivityType::Custom,
            state: None,
            url: None
        }));

        println!(r#"URL Cleaner Discord App
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html
{SOURCE_CODE_URL}

Install to your account: https://discord.com/oauth2/authorize?client_id={0}
Install to a server    : https://discord.com/oauth2/authorize?client_id={0}&scope=bot

To clean the URLs in a message, right click a message, go to the apps, and click any of the available \"Clean URLs\" actions"#, data_about_bot.application.id);
    }
}

#[poise::command(slash_command)]
async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(HELP_MESSAGE.get().expect("HELP_MESSAGE to have been set.")).await?;
    Ok(())
}

#[poise::command(
    context_menu_command = "Clean URLs",
    install_context = "User|Guild",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn clean_urls(
    ctx: Context<'_>,
    msg: serenity::Message
) -> Result<(), Error> {
    clean_urls_with_profile(ctx, msg, None).await.map_err(Into::into)
}

/// The enum of errors [`clean_urls_with_profile`] can return.
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
async fn clean_urls_with_profile(ctx: Context<'_>, msg: serenity::Message, profile: Option<&str>) -> Result<(), CleanUrlsError> {
    let data = ctx.data();
    let cleaner = data.cleaner.with_profile(profile).ok_or(CleanUrlsError::UnknownProfile)?;

    let job = Job {
        context: &Default::default(),
        cleaner: &cleaner,
        #[cfg(feature = "cache")]
        cache: &ctx.data().cache,
        #[cfg(feature = "cache")]
        cache_handle_config: data.cache_handle_config,
        unthreader: &Default::default(),
        lazy_task_configs: Box::new(GET_URLS.captures_iter(&msg.content).map(|x| Ok(x.name("URL1").or(x.name("URL2")).expect("The regex to always match at least one.").as_str().into())))
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
