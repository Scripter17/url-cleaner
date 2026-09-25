//! Parsing.

use clap::Parser;

mod url;
mod scheme;
mod userinfo;
mod username;
mod password;
mod host;
mod port;
mod path;
mod query;
mod fragment;

/// Parsing.
#[expect(clippy::missing_docs_in_private_items, reason = "Makes clap inherit the docs.")]
#[derive(Debug, Parser)]
pub enum Args {
    Url(url::Args),
    Scheme(scheme::Args),
    Userinfo(userinfo::Args),
    Username(username::Args),
    Password(password::Args),
    Host(host::Args),
    Port(port::Args),
    #[command(subcommand)]
    Path(path::Args),
    #[command(subcommand)]
    Query(query::Args),
    Fragment(fragment::Args),
}
impl Args {
    /// Do the command.
    pub fn r#do(self) {
        match self {
            Self::Url     (args) => args.r#do(),
            Self::Scheme  (args) => args.r#do(),
            Self::Userinfo(args) => args.r#do(),
            Self::Username(args) => args.r#do(),
            Self::Password(args) => args.r#do(),
            Self::Host    (args) => args.r#do(),
            Self::Port    (args) => args.r#do(),
            Self::Path    (args) => args.r#do(),
            Self::Query   (args) => args.r#do(),
            Self::Fragment(args) => args.r#do(),
        }
    }
}

