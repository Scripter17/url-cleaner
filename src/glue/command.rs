use std::process::{Command, Output as CommandOutput, Stdio};
use std::io::{Write, Error as IoError};
use std::path::{Path, PathBuf};
use url::{Url, ParseError};
use std::str::{from_utf8, FromStr, Utf8Error};
use thiserror::Error;
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::convert::Infallible;
use std::ffi::OsString;

use serde::{Serialize, Deserialize};

use crate::string_or_struct_magic;

/// The enabled form of the wrapper around [`Command`].
/// Only the necessary methods are exposed for the sake of simplicity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(remote= "Self")]
pub struct CommandWrapper {
    pub program: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub current_dir: Option<PathBuf>,
    #[serde(default)]
    pub envs: HashMap<String, String>,
    /// The rule for how the command's output is handled and returned in [`CommandWrapper::get_url`].
    #[serde(default)]
    pub output_handler: OutputHandler
}

impl FromStr for CommandWrapper {
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

string_or_struct_magic!{CommandWrapper}

/// The enabled form of `OutputHandler`.
/// Before a [`CommandWrapper`] returns its output, it passes it through this to allow for piping and control flow.
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
    PipeStdoutTo(Box<CommandWrapper>),
    /// Pipes the STDERR into the contained command's STDIN.
    PipeStderrTo(Box<CommandWrapper>),
    /// Extracts the URL from the STDOUT and applies it to the contained command's arguments.
    ApplyStdoutUrlTo(Box<CommandWrapper>),
    /// Extracts the URL from the STDERR and applies it to the contained command's arguments.
    ApplyStderrUrlTo(Box<CommandWrapper>),
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

// Serde helper functions
fn error_output_handler() -> Box<OutputHandler> {Box::new(OutputHandler::Error)}

/// The enabled form of the wrapper around all the errors a command condition/mapper can return.
#[derive(Debug, Error)]
pub enum CommandError {
    /// I/O error.
    #[error(transparent)]
    IoError(#[from] IoError),
    /// UTF-8 error.
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    /// URL parsing error.
    #[error(transparent)]
    ParseError(#[from] ParseError),
    /// The command was terminated by a signal. See [`std::process::ExitStatus::code`] for details.
    #[error("The command was terminated by a signal. See std::process::ExitStatus::code for details.")]
    SignalTermination,
    /// The output handler was [`OutputHandler::Error`].
    #[error("The output handler was OutputHandler::Error.")]
    ExplicitError
}

impl OutputHandler {
    /// Handles a command's output.
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

impl CommandWrapper {
    pub fn make_command(&self, url: Option<&Url>) -> Command {
        let mut ret = Command::new(&self.program);
        match url {
            Some(url) => {ret.args(self.args.iter().map(|arg| if &**arg=="{}" {Cow::Owned(OsString::from(url.as_str()))} else {Cow::Owned(OsString::from(arg))}));},
            None => {ret.args(self.args.iter().map(|arg| Cow::Owned(OsString::from(arg))));}
        }
        if let Some(current_dir) = &self.current_dir {
            ret.current_dir(current_dir);
        }
        ret.envs(self.envs.clone());
        ret
    }
    // fn apply_url(self, url: Option<&Url>) -> Self {
    //     if let Some(url) = url {
    //         // Why doesn't std::process::Command have a clear_args method?
    //         // More broadly why does the Command API suck?
    //         let mut ret=Command::new(&self.program);
    //         ret.args(self.args.iter().map(|arg| if Some(&**arg)==Some("{}") {Cow::Owned(OsString::from(url.as_str()))} else {Cow::Owned(OsString::from(arg))}));
    //         if let Some(dir) = &self.current_dir {
    //             ret.current_dir(dir);
    //         }
    //         ret.envs(self.envs.clone());
    //         Self {inner: ret, output_handler: self.output_handler.clone()}
    //     } else {
    //         self
    //     }
    // }
    
    /// Checks if the command's [`std::process::Command::get_program`] exists. Checks the system's PATH.
    /// Uses [this StackOverflow post](https://stackoverflow.com/a/37499032/10720231) to check the PATH.
    #[must_use]
    pub fn exists(&self) -> bool {
        PathBuf::from(&self.program).exists() || find_it(&self.program).is_some()
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
        match (url, stdin) {
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
                self.output_handler.handle(url, child.wait_with_output()?)
            },
            (url @ Some(_), None) => self.output_handler.handle(url, self.make_command(url).output()?),
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
                self.output_handler.handle(url, child.wait_with_output()?)
            },
            (None, None) => self.output_handler.handle(url, self.make_command(url).output()?) 
        }
    }

    /// Runs the command, does the [`OutputHandler`] stuff, removes trailing newlines and carriage returns form the output, then extracts the URL.
    /// # Errors
    /// If the call to [`Self::output`] returns an error, that error is returned.
    /// If the output cannot be parsed as a URL (give or take trailing newlines and carriage returns), returns the error [`CommandError::ParseError`].
    pub fn get_url(&self, url: Option<&Url>) -> Result<Url, CommandError> {
        Ok(Url::parse(self.output(url, None)?.trim_end_matches(&['\r', '\n']))?)
    }

}

impl PartialEq for CommandWrapper {
    /// Always returns `false` as commands are assumed to be non-deterministic.
    /// Yes that is a weird reason; This'll likely change at some point.
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

// https://stackoverflow.com/a/37499032/10720231
fn find_it<P: AsRef<Path>>(exe_name: P) -> Option<PathBuf> {
    let exe_name = enhance_exe_name(exe_name.as_ref());
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(&exe_name);
            full_path.is_file().then_some(full_path)
        })
    })
}

#[cfg(not(target_os = "windows"))]
fn enhance_exe_name(exe_name: &Path) -> Cow<Path> {
    exe_name.into()
}

#[cfg(target_os = "windows")]
fn enhance_exe_name(exe_name: &Path) -> Cow<Path> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let raw_input: Vec<_> = exe_name.as_os_str().encode_wide().collect();
    let raw_extension: Vec<_> = OsStr::new(".exe").encode_wide().collect();

    if raw_input.ends_with(&raw_extension) {
        exe_name.into()
    } else {
        let mut with_exe = exe_name.as_os_str().to_owned();
        with_exe.push(".exe");
        PathBuf::from(with_exe).into()
    }
}
