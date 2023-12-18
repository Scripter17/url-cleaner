#[cfg(feature = "regex")]
mod enabled;
#[cfg(not(feature = "regex"))]
mod disabled;

#[cfg(feature = "regex")]
pub use enabled::Regex;
#[cfg(not(feature = "regex"))]
pub use disabled::Regex;
