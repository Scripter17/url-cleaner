use publicsuffix::List;
use std::include_str;
use std::sync::OnceLock;
use thiserror::Error;

/// The copy of [Mozilla's public suffix list](https://publicsuffix.org) included in URL Cleaner at compile time.
pub const SUFFIXES_STR: &str=include_str!("public_suffix_list.dat");
/// The cached result of parsing of [`SUFFIXES_STR`].
pub static SUFFIXES: OnceLock<List>=OnceLock::new();

/// Parses the included list of top-level domains from [publicsuffix.org](https://publicsuffix.org/) if not already cached, then returns the cached list.
pub fn get_suffixes() -> Result<&'static List, GetSuffixesError> {
    Ok(if let Some(suffixes) = SUFFIXES.get() {
        suffixes
    } else {
        let suffixes=SUFFIXES_STR.parse().map_err(|_| GetSuffixesError::ParseError)?;
        SUFFIXES.get_or_init(|| suffixes)
    })
}

/// An enum of the currently only problem that can occur when loading the list of TLDs compiled into URL Cleaner.
#[derive(Debug, Error)]
pub enum GetSuffixesError {
    /// The list of top level domains included in URL Cleaner at compile time could not be parsed.
    /// Currently does not contain [`publicsuffix::Error`] pending [thiserror#271](https://github.com/dtolnay/thiserror/issues/271).
    #[error("The list of suffixes included in URL Cleaner at compile time could not be parsed.")]
    ParseError
}
