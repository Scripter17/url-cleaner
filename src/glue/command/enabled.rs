use std::ffi::OsString;
use std::process::Command;
use serde::{Deserialize, Deserializer};
use std::io::Error as IoError;
use std::path::PathBuf;
use url::{Url, ParseError};
use std::str::{from_utf8, FromStr, Utf8Error};
use thiserror::Error;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct CommandWrapper {
    #[serde(flatten, deserialize_with = "deserialize_command")]
    pub inner: Command
}

#[derive(Debug, Deserialize)]
struct CommandParts {
    #[serde(deserialize_with = "deserialize_os_string")]
    program: OsString,
    #[serde(deserialize_with = "deserialize_os_string_vec", default)]
    args: Vec<OsString>,
    #[serde(default)]
    current_dir: Option<PathBuf>
}

fn deserialize_os_string<'de, D>(deserializer: D) -> Result<OsString, D::Error>
where
    D: Deserializer<'de>
{
    let temp: String = Deserialize::deserialize(deserializer)?;
    Ok(temp.into())
}

fn deserialize_os_string_vec<'de, D>(deserializer: D) -> Result<Vec<OsString>, D::Error>
where
    D: Deserializer<'de>
{
    let temp: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(temp.into_iter().map(|x| x.into()).collect::<Vec<_>>())
}

fn deserialize_command<'de, D>(deserializer: D) -> Result<Command, D::Error>
where
    D: Deserializer<'de>
{
    let command_parts: CommandParts = Deserialize::deserialize(deserializer)?;
    let mut ret=Command::new(command_parts.program);
    ret.args(command_parts.args);
    match command_parts.current_dir {
        Some(dir) => {ret.current_dir(dir);},
        None => {}
    }
    Ok(ret)
}

/// A wrapper around all the errors a command condition/mapper can return.
#[derive(Debug, Error)]
pub enum CommandError {
    /// I/O error.
    #[error("I/O error.")]
    IoError(#[from] IoError),
    /// UTF-8 error.
    #[error("UTF-8 error.")]
    Utf8Error(#[from] Utf8Error),
    /// URL parsing error.
    #[error("URL parsing error.")]
    ParseError(#[from] ParseError),
    /// The command was terminated by a signal. See [`std::process::ExitStatus::code`] for details.
    #[error("The command was terminated by a signal. See std::process::ExitStatus::code for details.")]
    SignalTermination
}

impl CommandWrapper {
    /// Runs the command and gets the exit code. Returns [`Err(CommandError::SignalTerminatio)`] if the command returns no exit code.
    pub fn exit_code(&self, url: &Url) -> Result<i32, CommandError> {
        self.clone().apply_url(url).inner.status()?.code().ok_or(CommandError::SignalTermination)
    }

    /// Run the command and get the resulting URL from the STDOUT.
    /// First calls [`str::trim_end_matches`] on the STDOUT to get rid of all trailing carriage returns and newlines, then passes what's left to [`Url::parse`].
    pub fn get_url(&self, url: &Url) -> Result<Url, CommandError> {
        Ok(Url::parse(from_utf8(&self.clone().apply_url(url).inner.output()?.stdout)?.trim_end_matches(&['\r', '\n']))?)
    }

    /// A very messy function that puts the URL in the command arguments.
    fn apply_url(self, url: &Url) -> Self {
        // Why doesn't std::process::Command have a clear_args method?
        let mut ret=Command::new(self.inner.get_program());
        match self.inner.get_current_dir() {
            Some(dir) => {ret.current_dir(dir);},
            None => {}
        }
        ret.args(self.inner.get_args().map(|arg| if arg.to_str()==Some("{}") {Cow::Owned(OsString::from_str(url.as_str()).unwrap())} else {Cow::Borrowed(arg)}));
        Self { inner: ret }
    }
}

impl Clone for CommandWrapper {
    fn clone(&self) -> Self {
        let mut ret=Command::new(self.inner.get_program());
        ret.args(self.inner.get_args());
        match self.inner.get_current_dir() {
            Some(dir) => {ret.current_dir(dir);},
            None => {}
        }
        Self {inner: ret}
    }
}
