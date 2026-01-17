//! Misc. utility stuff.

use super::prelude::*;

/// The bin to run.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Bin {
    /// CLI
    Cli,
    /// Site
    Site,
    /// Site Client
    SiteClient,
    /// Discord
    Discord,
    /// Urlc Tool
    UrlcTool
}

impl Bin {
    /// The file name of the binary.
    pub fn file_name(self) -> &'static str {
        match self {
            Self::Cli        => "url-cleaner",
            Self::Site       => "url-cleaner-site",
            Self::SiteClient => "url-cleaner-site-client",
            Self::Discord    => "url-cleaner-discord",
            Self::UrlcTool   => "urlc-tool"
        }
    }

    /// The release path of the binary.
    pub fn release_path(self) -> &'static str {
        match self {
            Self::Cli        => "target/release/url-cleaner",
            Self::Site       => "target/release/url-cleaner-site",
            Self::SiteClient => "target/release/url-cleaner-site-client",
            Self::Discord    => "target/release/url-cleaner-discord",
            Self::UrlcTool   => "target/release/urlc-tool"
        }
    }

    /// The debug path of the binary.
    pub fn debug_path(self) -> &'static str {
        match self {
            Self::Cli        => "target/debug/url-cleaner",
            Self::Site       => "target/debug/url-cleaner-site",
            Self::SiteClient => "target/debug/url-cleaner-site-client",
            Self::Discord    => "target/debug/url-cleaner-discord",
            Self::UrlcTool   => "target/debug/urlc-tool"
        }
    }
}
