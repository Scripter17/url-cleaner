//! [`clean_urls`].

use std::fmt::Write;

use poise::reply::CreateReply;
use comrak::nodes::NodeValue;

use url_cleaner_engine::prelude::*;

use crate::prelude::*;

/// Clean a message's URLs.
pub async fn clean_urls(ctx: Context<'_>, msg: serenity::Message, cleaner: Cleaner<'_>) -> Result<(), serenity::Error> {
    let job = &Job {
        context: Default::default(),
        cleaner,
        unthreader: &Unthreader::default(),
        #[cfg(feature = "cache")]
        cache: ctx.data().cache,
        #[cfg(feature = "http")]
        http_client: &ctx.data().http_client
    };

    let mut ret = String::with_capacity(64 * msg.content.len().checked_ilog2().unwrap_or(0).pow(2) as usize);

    {
        let arena = comrak::Arena::new();
        let root = comrak::parse_document(
            &arena,
            &msg.content,
            &comrak::Options {
                extension: comrak::options::Extension::builder().autolink(true).spoiler(true).strikethrough(true).underline(true).build(),
                ..Default::default()
            }
        );

        for node in root.descendants() {
            if let NodeValue::Link(ref link) = node.data.borrow().value {
                match job.r#do(&link.url) {
                    Ok (x) => writeln!(ret, "{x}"   ).expect("This to always work."),
                    Err(e) => writeln!(ret, "-{e:?}").expect("This to always work.")
                }
            }
        }
    }

    if ret.is_empty() {
        ret = "No URLs found".into();
    }

    ctx.send(CreateReply::default().ephemeral(true).content(ret)).await?;
    Ok(())
}
