//! A discord app for URL Cleaner

use std::sync::LazyLock;
use std::path::PathBuf;
use std::borrow::Cow;

use clap::Parser;
use ::regex::Regex;
use poise::serenity_prelude as serenity;
use poise::reply::CreateReply;

use url_cleaner_engine::types::*;
use url_cleaner_engine::glue::*;

/// Basic URL getting regex.
///
/// Does not account for code blocks, spoilers, etc.
static GET_URLS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"https?://[^\])\s]+").expect("The URL parsing Regex to be valid."));

/// URL Cleaner Site Discord App
#[derive(Parser)]
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
    /// The cache to use.
    ///
    /// Defaults to "url-cleaner-discord-app-cache.sqlite"
    #[cfg(feature = "cache")]
    #[arg(long)]
    cache: Option<CachePath>,
    /// The ParamsDiff files to apply to the cleaner's Params.
    ///
    /// Applied before each --params-diff-profile.
    #[arg(long)]
    params_diff: Option<PathBuf>,
    /// Create an extra command for each ParamsDiff file.
    ///
    /// Profiles are applied on top of --params-diff.
    #[arg(long, num_args = 2)]
    params_diff_profile: Vec<Vec<String>>
}

/// The bot's state.
#[derive(Debug)]
struct State {
    /// The [`Cleaner`] to use.
    cleaner: Cleaner<'static>,
    /// The [`Cache`] to use.
    #[cfg(feature = "cache")]
    cache: Cache
}

/// The error type.
type Error = Box<dyn std::error::Error + Send + Sync>;
/// The context type.
type Context<'a> = poise::Context<'a, State, Error>;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut commands = vec![clean_urls()];

    #[cfg(feature = "default-cleaner")]
    let mut cleaner = Cleaner::load_or_get_default_no_cache(args.cleaner.as_deref()).expect("The cleaner to be valid.");
    #[cfg(not(feature = "default-cleaner"))]
    let mut cleaner = Cleaner::load_from_file(args.cleaner).expect("The cleaner to be valid.");

    if let Some(params_diff) = args.params_diff {
        serde_json::from_str::<ParamsDiff>(&std::fs::read_to_string(params_diff).expect("Reading the ParamsDiff file to a string to not error.")).expect("The read ParamsDiff file to be a valid ParamsDiff.").apply(cleaner.params.to_mut());
    }

    let state = State {
        cleaner,
        #[cfg(feature = "cache")]
        cache: args.cache.unwrap_or("url-cleaner-discord-app-cache.sqlite".into()).into()
    };

    for [name, params_diff] in args.params_diff_profile.into_iter().map(|args| <[String; 2]>::try_from(args).expect("The clap parser to work")) {
        let mut params = state.cleaner.params.clone().into_owned();

        serde_json::from_str::<ParamsDiff>(&std::fs::read_to_string(params_diff).expect("Reading the ParamsDiff file to a string to not error.")).expect("The read ParamsDiff file to be a valid ParamsDiff.").apply(&mut params);

        commands.push(poise::Command {
            name: Cow::Owned(format!("Clean URLs ({name})")),
            context_menu_action: Some(poise::structs::ContextMenuCommandAction::Message(move |ctx: poise::structs::ApplicationContext<'_, State, Error>, msg: serenity::Message| {
                Box::pin(async move {
                    clean_urls_with_params(
                        ctx.into(),
                        msg,
                        Some(ctx.command().custom_data.downcast_ref::<Params>().expect("The custom data to be a Params"))
                    ).await.map_err(|error| poise::FrameworkError::new_command(ctx.into(), error))
                })
            })),
            custom_data: Box::new(params),
            install_context: Some(vec![serenity::InstallationContext::User]),
            interaction_context: Some(vec![serenity::InteractionContext::Guild, serenity::InteractionContext::BotDm, serenity::InteractionContext::PrivateChannel]),
            ..Default::default()
        });
    }

    let token = std::env::var("URLCDA_KEY").expect("A discord bot token for URL Cleaner Discord App");
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
        .framework(framework)
        .await.expect("Building the client to work")
        .start().await.expect("The client to exit successfully");
}

#[poise::command(
    context_menu_command = "Clean URLs",
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel")]
async fn clean_urls(
    ctx: Context<'_>,
    msg: serenity::Message
) -> Result<(), Error> {
    clean_urls_with_params(ctx, msg, None).await
}

/// Clean a messages's URLs with the specified [`Params`].
async fn clean_urls_with_params(ctx: Context<'_>, msg: serenity::Message, params: Option<&Params>) -> Result<(), Error> {
    let mut cleaner = ctx.data().cleaner.borrowed();

    if let Some(params) = params {
        cleaner.params = Cow::Borrowed(params);
    }

    let job = Job {
        context: &Default::default(),
        cleaner: &cleaner,
        #[cfg(feature = "cache")]
        cache: &ctx.data().cache,
        #[cfg(feature = "cache")]
        cache_handle_config: Default::default(),
        lazy_task_configs: Box::new(GET_URLS.find_iter(&msg.content).map(|x| Ok(x.as_str().to_string().into())))
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
