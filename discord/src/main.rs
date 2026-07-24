//! A discord app for URL Cleaner

use std::path::PathBuf;
use std::borrow::Cow;
use std::sync::Arc;

use clap::Parser;
use bytes::Bytes;
use serenity::all::*;
use thiserror::Error;

use url_cleaner_engine::prelude::*;

mod help;
mod clean_url;
mod clean_urls;
mod parse;

/// The introduction to the /help message.
const INFO: &str = concat!(r#"URL Cleaner Discord
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html
"#, env!("CARGO_PKG_REPOSITORY"));

#[allow(rustdoc::bare_urls, reason = "It'd look bad in the console.")]
/// URL Cleaner Discord - Explicit non-consent to URL spytext.
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

    /// The secrets file to use.
    #[arg(long, value_name = "PATH")]
    secrets: Option<PathBuf>,

    /// The ProfilesConfig file.
    #[arg(long, verbatim_doc_comment, value_name = "PATH")]
    profiles: Option<PathBuf>,

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
pub struct Bot {
    /// The Cleaner as a file.
    cleaner_file: CreateAttachment<'static>,
    /// The ProfilesConfig as a file.
    profiles_file: CreateAttachment<'static>,
    /// The [`ProfiledCleaner`].
    profiled_cleaner: ProfiledCleaner<'static>,
    /// The [`Secrets`].
    secrets: Secrets,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    cache: url_cleaner_engine::prelude::Cache<'static>,
    /// The [`MaybeHttpClient`].
    #[cfg(feature = "http")]
    http_client: MaybeHttpClient,
}

/// [`main`].
#[derive(Debug, Error)]
enum DiscordError {
    /** [`LoadCleanerError`].        **/ #[error(transparent)] LoadCleanerError       (#[from] LoadCleanerError       ),
    /** [`LoadProfilesConfigError`]. **/ #[error(transparent)] LoadProfilesConfigError(#[from] LoadProfilesConfigError),
    /** [`LoadSecretsError`].        **/ #[error(transparent)] LoadSecretsError       (#[from] LoadSecretsError       ),
    /** [`TokenError`].              **/ #[error(transparent)] TokenError             (#[from] TokenError             ),
    /** [`serenity::Error`].         **/ #[error(transparent)] SerenityError          (#[from] serenity::Error        ),
}

#[tokio::main]
async fn main() -> Result<(), DiscordError> {
    let args = Args::parse();

    println!("{INFO}");

    let (cleaner_string , cleaner ) = cfg_select! {
        feature = "bundled-cleaner" => Cleaner::load_or_get_bundled(args.cleaner)?,
        _                           => {{let (x, y) = Cleaner::load(args.cleaner)?; (x.into(), y)}}
    };
    let (profiles_string, profiles) = ProfilesConfig::load_or_default(args.profiles)?;

    let cleaner_bytes = match cleaner_string {
        Cow::Owned   (x) => Bytes::from(x),
        Cow::Borrowed(x) => Bytes::from(x),
    };

    let profiles_bytes = match profiles_string {
        Cow::Owned   (x) => Bytes::from(x),
        Cow::Borrowed(x) => Bytes::from(x),
    };

    let bot = Bot {
        cleaner_file : CreateAttachment::bytes(cleaner_bytes , "cleaner.json" ).description("The Cleaner"       ),
        profiles_file: CreateAttachment::bytes(profiles_bytes, "profiles.json").description("The ProfilesConfig"),
        profiled_cleaner: profiles.make(Box::leak(Box::new(cleaner))),
        secrets: Secrets::load_or_default(args.secrets)?,
        #[cfg(feature = "cache")]
        cache: url_cleaner_engine::prelude::Cache {
            inner: Box::leak(Box::new(args.cache.into())),
            config: CacheConfig {
                read : !args.no_read_cache ,
                write: !args.no_write_cache,
                delay:  args.cache_delay   ,
            },
        },
        #[cfg(feature = "http")]
        http_client: MaybeHttpClient::new(Some(tokio::runtime::Handle::current())),
    };

    let intents = GatewayIntents::non_privileged();

    ClientBuilder::new(Token::from_env("URLCD_TOKEN")?, intents)
        .event_handler(Arc::new(bot))
        .activity(serenity::gateway::ActivityData::custom(env!("CARGO_PKG_REPOSITORY")))
        .await?.start().await?;

    Ok(())
}

#[serenity::async_trait]
impl EventHandler for Bot {
    async fn dispatch(&self, context: &Context, event: &FullEvent) {
        match event {
            FullEvent::Ready {data_about_bot, ..} => {
                println!();
                println!("Connected!");
                println!();
                println!("Install to your account: https://discord.com/oauth2/authorize?client_id={0}"          , data_about_bot.application.id);
                println!("Insrall to a server    : https://discord.com/oauth2/authorize?client_id={0}&scope=bot", data_about_bot.application.id);

                let mut profile = CreateCommandOption::new(CommandOptionType::String, "profile", "The profile");

                for name in self.profiled_cleaner.named.keys() {
                    profile = profile.add_string_choice(name, name);
                }

                context.http.create_global_command(&CreateCommand::new("clean_url")
                    .description("Clean a URL")
                    .kind(CommandType::ChatInput)
                    .add_option(CreateCommandOption::new(CommandOptionType::String, "url", "The URL to clean").required(true))
                    .add_option(profile)
                ).await.expect("Creating the clean_url command to work.");


                for name in self.profiled_cleaner.named.keys() {
                    context.http.create_global_command(&CreateCommand::new(format!("Clean URLs ({name})"))
                        .kind(CommandType::Message)
                    ).await.expect("Creating ever clean_urls command to work.");
                }

                context.http.create_global_command(&CreateCommand::new("help").description("Help").kind(CommandType::ChatInput)).await.expect("Creating the help command to work.");
            },
            FullEvent::InteractionCreate {interaction: Interaction::Command(command), ..} => match command.data.name.as_str() {
                "help"       => self.help      (context, command      ).await,
                "clean_url"  => self.clean_url (context, command      ).await,
                "Clean URLs" => self.clean_urls(context, command, None).await,
                x if let Some(y) = x.strip_prefix("Clean URLs (") && let Some(z) = y.strip_suffix(")") => self.clean_urls(context, command, Some(z)).await,
                _ => unreachable!()
            },
            _ => {}
        }
    }
}
