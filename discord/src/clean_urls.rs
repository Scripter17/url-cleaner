//! [`clean_urls`].

use std::fmt::Write;

use crate::*;

impl Bot {
    /// The `clean_urls` message context menu command.
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub async fn clean_urls(&self, context: &Context, command: &CommandInteraction, profile: Option<&str>) {
        if let Some(ResolvedTarget::Message(msg)) = command.data.target() {
            let job = Job {
                cleaner    : self.profiled_cleaner.get(profile).expect("To only be given valid profiles."),
                context    : Default::default(),
                secrets    : &self.secrets,
                unthreader : None,
                #[cfg(feature = "http")]
                http_client: self.http_client.as_ref(),
                #[cfg(feature = "cache")]
                cache_client: &self.cache_client,
                #[cfg(feature = "cache")]
                cache_config:  self.cache_config,
            };

            let mut ret = String::new();

            for url in crate::parse::parse(&msg.content) {
                match job.r#do(url) {
                    Ok ((_, x)) => writeln!(ret, "{x}"   ).expect("This to always work."),
                    Err(e     ) => writeln!(ret, "-{e:?}").expect("This to always work.")
                }
            }

            if ret.is_empty() {
                ret = "No URLs".into();
            }

            command.create_response(&context.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().ephemeral(true).content(ret))).await.expect("Sending the response to work.");
        }
    }
}
