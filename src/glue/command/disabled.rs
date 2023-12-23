use serde::{de::Error, Deserialize, Deserializer};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone)]
/// The disabled form of the wrapper around [`std::process::Command`].
/// This is the result of the default `command` feature being disabled during compile time.
/// This form cannot be deserialized, which may or may not be the best way to handle this.
pub struct CommandWrapper;

#[derive(Debug, Clone)]
pub enum OutputHandler {}

/// The disabled form of CommandError. As an empty enum it can't be created at all (in safe Rust).
#[derive(Debug, Error)]
pub enum CommandError {}

impl<'de> Deserialize<'de> for CommandWrapper {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(D::Error::custom("URL Cleaner was compiled without support for running commands."))
    }
}

impl<'de> Deserialize<'de> for OutputHandler {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(D::Error::custom("URL Cleaner was compiled without support for running commands."))
    }
}

impl CommandWrapper {
    pub fn exit_code(&self, _url: &Url) -> Result<i32, CommandError> {
        panic!()
    }

    pub fn get_url(&self, _url: &Url) -> Result<Url, CommandError> {
        panic!()
    }
}
