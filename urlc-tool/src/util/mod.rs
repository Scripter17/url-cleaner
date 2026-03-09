//! Util.

pub mod site;

/// Prelude module for importing everything here better.
pub mod prelude {
    pub use super::super::prelude::*;

    pub use super::site::*;

    pub use super::{TerminateOnDrop, Bin};

    pub use super::{new_file, is_default};
}

use prelude::*;

/// SIGTERMs the [`std::process::Child`] on [`Drop`].
#[derive(Debug)]
pub struct TerminateOnDrop(pub std::process::Child);

impl Drop for TerminateOnDrop {
    fn drop(&mut self) {
        if self.0.try_wait().unwrap().is_none() {
            unsafe {
                libc::kill(self.0.id() as _, libc::SIGTERM);
            }
            assert_eq!(self.0.wait().unwrap().code(), None);
        }
    }
}

/// Make a new writable [`File`] and its directory if needed.
pub fn new_file<P: AsRef<Path>>(path: P) -> File {
    if let Some(dir) = path.as_ref().parent() {
        std::fs::create_dir_all(dir).unwrap();
    }

    std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(path).unwrap()
}

/// Returns [`true`] if `x` is the deault value.
pub fn is_default<T: Default + PartialEq>(x: &T) -> bool {
    x == &T::default()
}

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
}

impl Bin {
    /// The binary's file name.
    pub fn file_name(self) -> &'static str {
        match self {
            Self::Cli        => "url-cleaner",
            Self::Site       => "url-cleaner-site",
            Self::SiteClient => "url-cleaner-site-client",
            Self::Discord    => "url-cleaner-discord,"
        }
    }

    /// The path of the release binary.
    pub fn release_path(self) -> &'static str {
        match self {
            Self::Cli        => "target/release/url-cleaner",
            Self::Site       => "target/release/url-cleaner-site",
            Self::SiteClient => "target/release/url-cleaner-site-client",
            Self::Discord    => "target/release/url-cleaner-discord,"
        }
    }

    /// The path of the debug binary.
    pub fn debug_path(self) -> &'static str {
        match self {
            Self::Cli        => "target/debug/url-cleaner",
            Self::Site       => "target/debug/url-cleaner-site",
            Self::SiteClient => "target/debug/url-cleaner-site-client",
            Self::Discord    => "target/debug/url-cleaner-discord,"
        }
    }
}
