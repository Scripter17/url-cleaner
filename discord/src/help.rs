//! [`help`].

use crate::*;

impl Bot {
    /// The `help` slash command.
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub async fn help(&self, context: &Context, command: &CommandInteraction) {
        let mut message = format!("\
            {INFO}\n\n\
            Use either the `clean_url` slash command or the various \"Clean URLs\" context menu commands to clean URLs.\
        ");

        if let CurrentApplicationInfo {bot_public: true, id, ..} = context.http.get_current_application_info().await.expect("???") {
            message.push_str(&format!("\n\
                Install to your account: https://discord.com/oauth2/authorize?client_id={id}\
                Insrall to a server: https://discord.com/oauth2/authorize?client_id={id}&scope=bot\
            "));
        }

        let response = CreateInteractionResponseMessage::new()
            .content(message)
            .ephemeral(true)
            .add_files([self.cleaner_file.clone(), self.profiles_file.clone()]);

        command.create_response(&context.http, CreateInteractionResponse::Message(response)).await.expect("Sending response to work.");
    }
}
