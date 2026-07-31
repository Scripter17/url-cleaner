//! [`clean_url`].

use crate::*;

impl Bot {
    /// The `clean_url` slash command.
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub async fn clean_url(&self, context: &Context, command: &CommandInteraction) {
        let options = command.data.options();

        let url = options.iter().find_map(|x| {
            match x {
                ResolvedOption {name: "url", value: ResolvedValue::String(url), ..} => Some(*url),
                _ => None
            }
        }).expect("A URL");

        let profile = options.iter().find_map(|x| {
            match x {
                ResolvedOption {name: "profile", value: ResolvedValue::String(profile), ..} => Some(*profile),
                _ => None
            }
        });

        let cleaner = self.profiled_cleaner.get(profile).expect("Only valid profiles to be accepted");

        let response = CreateInteractionResponseMessage::new().ephemeral(true);

        let job = Job {
            cleaner,
            context    : Default::default(),
            unthreader : None,
            secrets    : &self.secrets,
            #[cfg(feature = "http")]
            http_client: self.http_client.as_ref(),
            #[cfg(feature = "cache")]
            cache_client: &self.cache_client,
            #[cfg(feature = "cache")]
            cache_config:  self.cache_config,
        };

        let response = match job.r#do(url) {
            Ok ((_, url)) => response.content(String::from(url)),
            Err(e       ) => response.content(format!("-{e:?}")),
        };

        command.create_response(&context.http, CreateInteractionResponse::Message(response)).await.expect("Sending the response to work.");
    }
}
