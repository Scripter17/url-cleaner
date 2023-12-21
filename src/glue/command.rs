#[cfg(feature = "command")]
mod enabled;
#[cfg(not(feature = "command"))]
mod disabled;

#[cfg(feature = "command")]
pub use enabled::{CommandWrapper as Command, CommandError};
#[cfg(not(feature = "command"))]
pub use disabled::{CommandWrapper as Command, CommandError};
