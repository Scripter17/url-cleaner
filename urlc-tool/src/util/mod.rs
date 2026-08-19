//! Util.

mod site;
mod fs;
mod process;

pub use site::*;
pub use fs::*;
pub use process::*;

use crate::prelude::*;

/// Format an integer.
pub fn format_int(x: u64) -> String {
    let mut ret = x.to_string();
    let mut i = ret.len();
    while i > 3 {
        i -= 3;
        ret.insert(i, ',');
    }
    ret
}

/// Returns [`true`] if `x` is the deault value.
pub fn is_default<T: Default + PartialEq>(x: &T) -> bool {
    x == &T::default()
}

/// The bin to run.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Bin {
    /** CLI.         **/ Cli       ,
    /** Site.        **/ Site      ,
    /** Site Client. **/ SiteClient,
    /** Discord.     **/ Discord   ,
    /** URLC Tool.   **/ UrlcTool  ,
    /** BURL Bench.  **/ BurlBench ,
}

impl Bin {
    /// The file name.
    pub fn file_name(self) -> &'static str {
        match self {
            Self::Cli        => "url-cleaner",
            Self::Site       => "url-cleaner-site",
            Self::SiteClient => "url-cleaner-site-client",
            Self::Discord    => "url-cleaner-discord",
            Self::UrlcTool   => "urlc-tool",
            Self::BurlBench  => "better-url-bench",
        }
    }

    /// The release path.
    pub fn release_path(self) -> &'static str {
        match self {
            Self::Cli        => "target/release/url-cleaner",
            Self::Site       => "target/release/url-cleaner-site",
            Self::SiteClient => "target/release/url-cleaner-site-client",
            Self::Discord    => "target/release/url-cleaner-discord",
            Self::UrlcTool   => "target/release/urlc-tool",
            Self::BurlBench  => "target/release/better-url-bench",
        }
    }

    /// The debug path.
    pub fn debug_path(self) -> &'static str {
        match self {
            Self::Cli        => "target/debug/url-cleaner",
            Self::Site       => "target/debug/url-cleaner-site",
            Self::SiteClient => "target/debug/url-cleaner-site-client",
            Self::Discord    => "target/debug/url-cleaner-discord",
            Self::UrlcTool   => "target/debug/urlc-tool",
            Self::BurlBench  => "target/debug/better-url-bench",
        }
    }
}
