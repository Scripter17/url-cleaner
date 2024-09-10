//! Provides [`CommandConfig`] to allow usage of external commands.
//! 
//! Enabled by the `commands` feature flag.

use std::process::{Command, Stdio};
use std::io::Write;
use std::path::PathBuf;
use std::str::{from_utf8, FromStr};
use std::collections::HashMap;
use std::convert::Infallible;
use std::ffi::OsString;

use url::Url;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use which::which;

#[allow(unused_imports, reason = "Used in a doc comment.")]
use crate::types::*;
use crate::util::*;

/// Instructions on how to make and run a [`Command`] object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(remote= "Self")]
pub struct CommandConfig {
    /// The program to run.
    pub program: String,
    /// The arguments to run [`Self::program`] with
    #[serde(default, skip_serializing_if = "is_default")]
    pub args: Vec<StringSource>,
    /// The directory to run [`Self::program`] in.
    /// 
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub current_dir: Option<PathBuf>,
    /// The environment arguments to run [`Self::program`] with.
    /// 
    /// If a call to [`StringSource::get`] returns [`None`], that environment variable is omitted from the request. For an environment variable with an empty value, use [`StringSource::NoneToEmptyString`].
    /// 
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub envs: HashMap<String, Option<StringSource>>,
    /// The STDIN to feed into the command.
    /// 
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub stdin: Option<StringSource>
}

impl FromStr for CommandConfig {
    type Err = Infallible;

    /// Simply treats the string as the command to run.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<&str> for CommandConfig {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for CommandConfig {
    fn from(value: String) -> Self {
        Self {
            program: value,
            args: Default::default(),
            current_dir: Default::default(),
            envs: Default::default(),
            stdin: Default::default()
        }
    }
}

crate::util::string_or_struct_magic!(CommandConfig);

/// The enum of all possible errors [`CommandConfig::exit_code`], [`CommandConfig::output`], and [`CommandConfig::get_url`] can return.
#[derive(Debug, Error)]
pub enum CommandError {
    /// Returned when a [`std::io::Error`] is encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Returned when a [`std::str::Utf8Error`] is encountered.
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Returned when a command is terminated by a signal. See [`std::process::ExitStatus::code`] for details.
    #[error("The command was terminated by a signal. See std::process::ExitStatus::code for details.")]
    SignalTermination,
    /// Returned when a call to [`StringSource::get`] returns `None` where it has to be `Some`.
    #[error("The specified StringSource returned None where it had to be Some.")]
    StringSourceIsNone,
    /// Returned when a [`StringSourceError`] is encountered.
    #[error(transparent)]
    StringSourceError(#[from] StringSourceError)
}

impl CommandConfig {
    /// Creates a [`Command`] using [`Self`].
    /// 
    /// DOES NOT APPLY STDIN.
    /// # Errors
    /// If a call to [`StringSource::get`] returns an error, that error is returned.
    pub fn make_command(&self, job_state: &JobState) -> Result<Command, CommandError> {
        let mut ret = Command::new(&self.program);
        for arg in self.args.iter() {
            ret.arg(OsString::from(get_string!(arg, job_state, CommandError)));
        }
        if let Some(current_dir) = &self.current_dir {
            ret.current_dir(current_dir);
        }
        ret.envs(self.envs.iter().map(|(k, v)| Ok(get_option_string!(v, job_state).map(|v| (k, v)))).filter_map(|x| x.transpose()).collect::<Result<HashMap<_, _>, StringSourceError>>()?);
        Ok(ret)
    }

    /// Checks if the path at [`Self::program`] exists.
    /// 
    /// Currently does not do any permissions or executability checks.
    /// 
    /// Uses [`which::which`] to emulate PATH handling.
    #[must_use]
    pub fn exists(&self) -> bool {
        PathBuf::from(&self.program).exists() || which(&self.program).is_ok()
    }

    /// Runs the command and gets the exit code.
    /// # Errors
    /// If the command returns no exit code, returns the error [`CommandError::SignalTermination`].
    pub fn exit_code(&self, job_state: &JobState) -> Result<i32, CommandError> {
        self.make_command(job_state)?.status()?.code().ok_or(CommandError::SignalTermination)
    }

    /// Run the command from [`Self::make_command`] and returns the STDOUT.
    /// # Errors
    /// If `stdin` is `Some` and the calls to [`Command::spawn`], [`std::process::ChildStdin::write_all`], or [`std::process::Child::wait_with_output`] returns an error, that error is returned.
    /// 
    /// If `stdin` is `None` and the call to [`Command::output`] returns an error, that error is returned.
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't ever panic.")]
    pub fn output(&self, job_state: &JobState) -> Result<String, CommandError> {
        // https://stackoverflow.com/a/49597789/10720231
        let mut command = self.make_command(job_state)?;
        command.stdout(Stdio::piped());
        command.stderr(Stdio::null());
        let child = if let Some(stdin) = &self.stdin {
            command.stdin(Stdio::piped());
            let mut child=command.spawn()?;
            let child_stdin=child.stdin.as_mut().expect("The STDIN just set to be available."); // This never panics.
            child_stdin.write_all(get_string!(stdin, job_state, CommandError).as_bytes())?;
            child
        } else {
            command.spawn()?
        };
        Ok(from_utf8(&child.wait_with_output()?.stdout)?.to_string())
    }

    /// Runs the command, gets the STDOUT, trims trailing newlines and carriage returns form the output using [`str::trim_end_matches`], then extracts the URL.
    /// # Errors
    /// If the call to [`Self::output`] returns an error, that error is returned.
    /// 
    /// If the call to [`Url::parse`] returns an error, that error is returned.
    pub fn get_url(&self, job_state: &JobState) -> Result<Url, CommandError> {
        Ok(Url::parse(self.output(job_state)?.trim_end_matches(&['\r', '\n']))?)
    }
}
