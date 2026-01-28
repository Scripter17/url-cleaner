//! [`clean_url`].

use poise::reply::CreateReply;

use url_cleaner_engine::prelude::*;

use crate::prelude::*;

/// Autocomplete for [`clean_url`]'s `profile` field.
async fn profile_autocomplete<'a>(ctx: Context<'a>, partial: &str) -> serenity::CreateAutocompleteResponse {
    serenity::CreateAutocompleteResponse::new()
        .set_choices(ctx.data().profiled_cleaner.names().filter(|name| name.to_lowercase().replace(" ", "").contains(&partial.to_lowercase().replace(" ", ""))).map(Into::into).collect())
}

/// Clean a single URL.
#[poise::command(slash_command, install_context = "Guild|User")]
pub async fn clean_url(
    ctx: Context<'_>,
    #[description = "The URL to clean."]
    url: String,
    #[description = "The name of a profile to use, if any are available."]
    #[autocomplete = "profile_autocomplete"]
    profile: Option<String>
) -> Result<(), Error> {
    match ctx.data().profiled_cleaner.get(profile.as_deref()) {
        Some(cleaner) => {
            let job = &Job {
                context: Default::default(),
                cleaner,
                unthreader: &Unthreader::default(),
                #[cfg(feature = "cache")]
                cache: ctx.data().cache,
                #[cfg(feature = "http")]
                http_client: &ctx.data().http_client
            };

            let ret = match job.r#do(url) {
                Ok (x) => String::from(x),
                Err(e) => format!("-{e:?}")
            };

            ctx.send(CreateReply::default().ephemeral(true).content(ret)).await?;
        },
        None => {ctx.send(CreateReply::default().ephemeral(true).content(format!("Unknown profile: {profile:?}"))).await?;}
    }

    Ok(())
}
