use std::process::{Command, Output as CommandOutput, Stdio};
use std::io::Write;
use std::path::PathBuf;
use std::str::{from_utf8, FromStr};
use std::collections::HashMap;
use std::convert::Infallible;
use std::ffi::OsStr;
#[cfg(target_family = "unix")]
use std::os::unix::ffi::OsStrExt;
#[cfg(not(target_family = "unix"))]
use std::ffi::OsString;

use url::Url;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use which::which;

use crate::string_or_struct_magic;

/// The enabled form of the wrapper around [`Command`].
/// Only the necessary methods are exposed for the sake of simplicity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(remote= "Self")]
pub struct CommandConfig {
    /// The program to run.
    /// If the path is not found, looks for it in the PATH.
    pub program: String,
    /// The arguments to run [`Self::program`] with
    #[serde(default)]
    pub args: Vec<String>,
    /// The directory to run [`Self::program`] in.
    #[serde(default)]
    pub current_dir: Option<PathBuf>,
    /// The environment arguments to run [`Self::program`] with.
    #[serde(default)]
    pub envs: HashMap<String, String>,
    /// The rule for how the command's output is handled and returned in [`Self::get_url`].
    #[serde(default)]
    pub output_handler: OutputHandler
}

impl FromStr for CommandConfig {
    type Err = Infallible;

    /// Simply treats the string as the command to run.
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            program: x.to_string(),
            args: Vec::new(),
            current_dir: None,
            envs: HashMap::new(),
            output_handler: OutputHandler::default()
        })
    }
}

string_or_struct_magic!(CommandConfig);

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
    /// The command was terminated by a signal. See [`std::process::ExitStatus::code`] for details.
    #[error("The command was terminated by a signal. See std::process::ExitStatus::code for details.")]
    SignalTermination,
    /// The output handler was [`OutputHandler::Error`].
    #[error("The output handler was OutputHandler::Error.")]
    ExplicitError
}

// Serde helper functions
fn error_output_handler() -> Box<OutputHandler> {Box::new(OutputHandler::Error)}

impl CommandConfig {
    /// Creates a [`Command`] using [`Self`]
    pub fn make_command(&self, url: Option<&Url>) -> Command {
        let mut ret = Command::new(&self.program);
        match url {
            #[cfg(target_family = "unix")]
            Some(url) => {ret.args(self.args.iter().map(|arg| if &**arg=="{}" {OsStr::from_bytes(url.as_str().as_bytes())} else {OsStr::from_bytes(arg.as_bytes())}));},
            #[cfg(not(target_family = "unix"))]
            Some(url) => {ret.args(self.args.iter().map(|arg| if &**arg=="{}" {OsString::from(url.as_str())} else {OsString::from(arg)}));},
            #[cfg(target_family = "unix")]
            None => {ret.args(self.args.iter().map(|arg| OsStr::from_bytes(arg.as_bytes())));},
            #[cfg(not(target_family = "unix"))]
            None => {ret.args(self.args.iter().map(OsString::from));}
        }
        if let Some(current_dir) = &self.current_dir {
            ret.current_dir(current_dir);
        }
        ret.envs(self.envs.clone());
        ret
    }

    /// Checks if the command's [`std::process::Command::get_program`] exists. Checks the system's PATH.
    /// Uses [this StackOverflow post](https://stackoverflow.com/a/37499032/10720231) to check the PATH.
    #[must_use]
    pub fn exists(&self) -> bool {
        PathBuf::from(&self.program).exists() || which(&self.program).is_ok()
    }

    /// Runs the command and gets the exit code.
    /// # Errors
    /// If the command returns no exit code, returns the error [`Err(CommandError::SignalTermination)`].
    pub fn exit_code(&self, url: &Url) -> Result<i32, CommandError> {
        self.make_command(Some(url)).status()?.code().ok_or(CommandError::SignalTermination)
    }

    /// # Errors
    /// If `stdin` is `Some` and the calls to [`Command::spawn`], [`std::process::ChildStdin::write_all`], or [`std::process::Child::wait_with_output`] returns an error, that error is returned.
    /// If `stdin` is `None` and the call to [`Command::output`] returns an error, that error is returned.
    #[allow(clippy::missing_panics_doc)]
    pub fn output(&self, url: Option<&Url>, stdin: Option<&[u8]>) -> Result<String, CommandError> {
        Ok(match (url, stdin) {
            (url @ Some(_), Some(stdin)) => {
                // https://stackoverflow.com/a/49597789/10720231
                let mut command=self.make_command(url);
                let mut child=command
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;
                let child_stdin=child.stdin.as_mut().expect("The STDIN just set to be available."); // This never panics. If it ever does, so will I.
                child_stdin.write_all(stdin)?;
                self.output_handler.handle(url, child.wait_with_output()?)?
            },
            (url @ Some(_), None) => self.output_handler.handle(url, self.make_command(url).output()?)?,
            (None, Some(stdin)) => {
                // https://stackoverflow.com/a/49597789/10720231
                let mut command=self.make_command(url);
                let mut child=command
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;
                let child_stdin=child.stdin.as_mut().expect("The STDIN just set to be available."); // This never panics. If it ever does, so will I.
                child_stdin.write_all(stdin)?;
                self.output_handler.handle(url, child.wait_with_output()?)?
            },
            (None, None) => self.output_handler.handle(url, self.make_command(url).output()?)?
        })
    }

    /// Runs the command, does the [`OutputHandler`] stuff, removes trailing newlines and carriage returns form the output, then extracts the URL.
    /// # Errors
    /// If the call to [`Self::output`] returns an error, that error is returned.
    /// If the output cannot be parsed as a URL (give or take trailing newlines and carriage returns), returns the error [`CommandError::UrlParseError`].
    pub fn get_url(&self, url: Option<&Url>) -> Result<Url, CommandError> {
        Ok(Url::parse(self.output(url, None)?.trim_end_matches(&['\r', '\n']))?)
    }

}

/// The enabled form of `OutputHandler`.
/// Before a [`CommandConfig`] returns its output, it passes it through this to allow for piping and control flow.
/// For the sake of simplicity, [`OutputHandler::handle`] returns a [`Result<String, CommandError>`] instead of [`Result<Vec<u8>, CommandError>`].
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum OutputHandler {
    /// Return the STDOUT.
    #[default]
    ReturnStdout,
    /// Return the STDERR.
    ReturnStderr,
    /// Always return the error [`CommandError::ExplicitError`].
    Error,
    /// Pipes the STDOUT into the contained command's STDIN.
    PipeStdoutTo(Box<CommandConfig>),
    /// Pipes the STDERR into the contained command's STDIN.
    PipeStderrTo(Box<CommandConfig>),
    /// Extracts the URL from the STDOUT and applies it to the contained command's arguments.
    ApplyStdoutUrlTo(Box<CommandConfig>),
    /// Extracts the URL from the STDERR and applies it to the contained command's arguments.
    ApplyStderrUrlTo(Box<CommandConfig>),
    /// If the exit code equals `equals`, `then` is used as the handler. Otherwise `else` (Defaults to [`OutputHandler::Error`]) is used.
    /// Errors if the command is terminated without returning an exit code.
    IfExitCode {
        /// The expected exit code. Defaults to zero.
        #[serde(default)]
        expect: i32,
        /// The handler to use if the command's exit code equals `expect`.
        then: Box<OutputHandler>,
        /// The handler to use if the command's exit code does not equal expects`
        #[serde(default = "error_output_handler")]
        r#else: Box<OutputHandler>
    }
}

impl OutputHandler {
    /// Returns a string from the requested part of the command's output.
    /// When piping STDOUT/STDERR to another command's STDIN, no UTF-8 checks are done.
    /// # Errors
    /// If the command returns an error, that error is returned.
    /// If the command's STDOUT is not valid UTF-8 when using [`Self::ReturnStdout`] or [`Self::ApplyStdoutUrlTo`], returns the error [`CommandError::Utf8Error`].
    /// If the command's STDERR is not valid UTF-8 when using [`Self::ReturnStderr`] or [`Self::ApplyStderrUrlTo`], returns the error [`CommandError::Utf8Error`].
    pub fn handle(&self, url: Option<&Url>, output: CommandOutput) -> Result<String, CommandError> {
        match self {
            Self::ReturnStdout                     => Ok(from_utf8(&output.stdout)?.to_string()),
            Self::ReturnStderr                     => Ok(from_utf8(&output.stderr)?.to_string()),
            Self::Error                            => Err(CommandError::ExplicitError),
            Self::PipeStdoutTo(command)            => command.output(url, Some(&output.stdout)),
            Self::PipeStderrTo(command)            => command.output(url, Some(&output.stderr)),
            Self::ApplyStdoutUrlTo(command)        => command.output(Some(&Url::parse(from_utf8(&output.stdout)?)?), None),
            Self::ApplyStderrUrlTo(command)        => command.output(Some(&Url::parse(from_utf8(&output.stderr)?)?), None),
            Self::IfExitCode{expect, then, r#else} => if output.status.code().ok_or(CommandError::SignalTermination)?==*expect {then.handle(url, output)} else {r#else.handle(url, output)}
        }
    }
}
