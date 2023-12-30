use serde::{
    Serialize, Deserialize,
    ser::{Error as _, Serializer},
    de::{Error as _, Deserializer}
};
use thiserror::Error;
use url::Url;

/// The disabled version of the wrapper around [`std::process::Command`].
/// This is the result of the default `command` feature being at during compile time.
/// This version cannot be deserialized, which may or may not be the best way to handle this.
/// Calling any provided method on this will panic.
#[derive(Debug, Clone)]
pub struct CommandWrapper;

impl PartialEq for CommandWrapper {
    fn eq(&self, _: &Self) -> bool {false}
}

/// The disabled version of `OutputHandler`. As an empty enum it can't be created at all.
/// Calling any provided method on this will panic.
#[derive(Debug, Clone, PartialEq)]
pub enum OutputHandler {}

/// The disabled version of `CommandError`. As an empty enum it can't be created at all.
/// Calling any provided method on this will panic.
#[derive(Debug, Error)]
pub enum CommandError {}

impl<'de> Deserialize<'de> for CommandWrapper {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(D::Error::custom("URL Cleaner was compiled without the `commands` feature."))
    }
}

impl Serialize for CommandWrapper {
    fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
        Err(S::Error::custom("URL Cleaner was compiled without the `commands` feature."))
    }
}

impl<'de> Deserialize<'de> for OutputHandler {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(D::Error::custom("URL Cleaner was compiled without the `commands` feature."))
    }
}

impl Serialize for OutputHandler {
    fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
        Err(S::Error::custom("URL Cleaner was compiled without the `commands` feature."))
    }
}

impl CommandWrapper {
    /// The disabled version of the function that checks if a path exists.
    /// # Panics
    /// This version will always panic.
    pub fn exists(&self) -> bool {
        panic!("URL Cleaner was compiled without the `commands` feature.")
    }

    /// The disabled version of the function that gets the command's exit code.
    /// # Panics
    /// This version will always panic.
    pub fn exit_code(&self, _url: &Url) -> Result<i32, CommandError> {
        panic!("URL Cleaner was compiled without the `commands` feature.")
    }

    /// The disabled version of the function that gets the URL from the command's output.
    /// # Panics
    /// This version will always panic.
    pub fn get_url(&self, _url: &Url) -> Result<Url, CommandError> {
        panic!("URL Cleaner was compiled without the `commands` feature.")
    }
}
