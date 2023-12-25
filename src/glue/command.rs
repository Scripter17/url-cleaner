#[cfg(feature = "commands")]
mod enabled;
#[cfg(not(feature = "commands"))]
mod disabled;

#[cfg(feature = "commands")]
pub use enabled::{CommandWrapper, OutputHandler, CommandError};
#[cfg(not(feature = "commands"))]
pub use disabled::{CommandWrapper, OutputHandler, CommandError};
