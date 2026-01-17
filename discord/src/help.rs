//! [`help`].

use std::fmt::Write;

use poise::serenity_prelude::CreateAttachment;
use poise::reply::CreateReply;

use crate::prelude::*;

/// The help command.
#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let mut ret = crate::INFO.to_string();

    let bot_data = ctx.http().get_current_application_info().await.expect("Getting the current application info to work.");

    if bot_data.bot_public {
        writeln!(ret, "\n\nInstall to your account: https://discord.com/oauth2/authorize?client_id={0}"    , bot_data.id).expect("???");
        writeln!(ret, "Install to a server: https://discord.com/oauth2/authorize?client_id={0}&scope=bot", bot_data.id).expect("???");
    } else {
        writeln!(ret, "\n\nThis instance is private.").expect("???");
    }

    ret.push_str("\nTo clean the URLs in a message, right click/long press a message, go to the apps, and click any of the available \"Clean URLs\" actions");
    ret.push_str("\nAlternatively, you can use the /clean_url slash command.");

    ctx.send(CreateReply::default()
        .ephemeral(true)
        .content(ret)
        .attachment(CreateAttachment::bytes(ctx.data().cleaner_string        .clone(), "cleaner.json" ).description("The Cleaner this bot is using."))
        .attachment(CreateAttachment::bytes(ctx.data().profiles_config_string.clone(), "profiles.json").description("The ProfilesConfig this bot is using."))
    ).await?;
    Ok(())
}

