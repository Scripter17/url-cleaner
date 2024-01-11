#[cfg(all(feature = "regex", feature = "lazy-regex"))]
mod lazy;
#[cfg(all(feature = "regex", not(feature = "lazy-regex")))]
mod eager;
#[cfg(not(feature = "regex"))]
mod disabled;

#[cfg(all(feature = "regex", feature = "lazy-regex"))]
pub use lazy::*;
#[cfg(all(feature = "regex", not(feature = "lazy-regex")))]
pub use eager::*;
#[cfg(not(feature = "regex"))]
pub use disabled::*;

#[cfg(feature = "regex")]
mod regex_parts;
#[cfg(feature = "regex")]
pub use regex_parts::*;
