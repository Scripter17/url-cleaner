use publicsuffix::List;
// pub use publicsuffix::Error as ParseError;
use std::include_str;
use std::sync::OnceLock;
use thiserror::Error;

const TLDS_STR: &str=include_str!("tlds.dat");
static TLDS: OnceLock<List>=OnceLock::new();

/// Parses the included list of top-level domains from [publicsuffix.org](https://publicsuffix.org/) if not already cached, then returns the cached list.
pub fn get_tlds() -> Result<&'static List, GetTldsError> {
    Ok(if let Some(tlds) = TLDS.get() {
        tlds
    } else {
        let tlds=TLDS_STR.parse().map_err(|_| GetTldsError::ParseError)?;
        TLDS.get_or_init(|| tlds)
    })
}

/// An enum of the currently only problem that can occur when loading the list of TLDs compiled into URL Cleaner.
#[derive(Debug, Error)]
pub enum GetTldsError {
    /// The list of top level domains included in URL Cleaner at compile time could not be parsed.
    /// Currently does not contain [`publicSuffix::Error`] pending [thiserror#271](https://github.com/dtolnay/thiserror/issues/271).
    #[error("The list of top level domains included in URL Cleaner at compile time could not be parsed.")]
    ParseError
}
