//! [`clean_urls`].

use std::fmt::Write;

use crate::*;

impl Bot {
    /// The `clean_urls` message context menu command.
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub async fn clean_urls(&self, context: &Context, command: &CommandInteraction, profile: Option<&str>) {
        if let Some(ResolvedTarget::Message(msg)) = command.data.target() {
            let job = Job {
                cleaner     : self.profiled_cleaner.get(profile).expect("To only be given valid profiles."),
                context     : Default::default(),
                secrets     : &self.secrets,
                thread_hider: None,
                #[cfg(feature = "http")]
                http_client : self.http_client.as_ref(),
                #[cfg(feature = "cache")]
                cache_client: &self.cache_client,
                #[cfg(feature = "cache")]
                cache_config:  self.cache_config,
            };

            let mut ret = String::new();

            for url in crate::get_urls::get_urls(&msg.content) {
                match job.r#do(url) {
                    Ok ((_, x)) => writeln!(ret, "{x}"   ).expect("???"),
                    Err(e     ) => writeln!(ret, "-{e:?}").expect("???"),
                }
            }

            for (i, embed) in (1..).zip(&msg.embeds) {
                if let Some(description) = embed.description.as_deref() {
                    let mut temp = String::new();

                    for url in crate::get_urls::get_urls(description) {
                        match job.r#do(url) {
                            Ok ((_, x)) => writeln!(temp, "{x}"   ).expect("???"),
                            Err(e     ) => writeln!(temp, "-{e:?}").expect("???"),
                        }
                    }

                    if !temp.is_empty() {
                        writeln!(ret, "Embed {i}:\n{temp}").expect("???")
                    }
                }
            }

            if ret.is_empty() {
                ret = "No URLs found".into();
            } else if msg.content.contains("||") {
                ret = format!("Found \\|\\|; Assuming all URLs are spoilers\n||{ret}||");
            }

            command.create_response(&context.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().ephemeral(true).content(ret))).await.expect("Sending the response to work.");
        }
    }
}
