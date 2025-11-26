//! [`ReadyHandler`].

use serenity::{Context, Ready, EventHandler};

use crate::prelude::*;

/// An [`EventHandler`] that prints license info and the app's authorization URL on [`EventHandler::ready`].
pub struct ReadyHandler;

#[serenity::async_trait]
impl EventHandler for ReadyHandler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("\nInstall to your account: https://discord.com/oauth2/authorize?client_id={0}"    , data_about_bot.application.id);
        println!("Insrall to a server: https://discord.com/oauth2/authorize?client_id={0}&scope=bot", data_about_bot.application.id);

        ctx.set_activity(Some(serenity::gateway::ActivityData::custom(env!("CARGO_PKG_REPOSITORY"))));
    }
}
