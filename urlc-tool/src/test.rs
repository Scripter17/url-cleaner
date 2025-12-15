//! Do tests.

use serde::{Serialize, Deserialize};

use super::prelude::*;

/// Do tests.
#[derive(Debug, Parser)]
pub struct Args {
    /// The tests file.
    #[arg(long)]
    pub tests: Option<PathBuf>,
    /// The filter.
    #[arg(long)]
    pub filter: Option<String>,
    /// Assert the cleaner's suitability.
    #[arg(long)]
    pub assert_suitability: bool
}

/// The bundled tests.
const BUNDLED_TESTS: &str = include_str!("bundled-tests.json");

/// A tests file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tests {
    /// The [`Job`]s to test.
    pub jobs: Vec<Job>
}

/// A job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Job {
    /// The name of the job.
    pub name: String,
    /// The `--job-context`.
    pub job_context: Option<serde_json::Value>,
    /// The `--params-diff`.
    pub params_diff: Option<serde_json::Value>,
    /// The [`Test`]s.
    pub tests: Vec<Test>
}

/// A test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Test {
    /// The task.
    task: serde_json::Value,
    /// The expected result.
    expect: String
}

const TMP: &str = "urlc-tool/tmp/test/";

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        std::fs::create_dir_all(TMP).unwrap();

        if self.assert_suitability {
            assert_eq!(Command::new("target/debug/url-cleaner")
                .arg("--assert-suitability")
                .spawn().unwrap().wait().unwrap().code(), Some(0));
        }

        let tests_string = match self.tests {
            Some(path) => Cow::Owned(std::fs::read_to_string(path).unwrap()),
            None => Cow::Borrowed(BUNDLED_TESTS)
        };

        let tests: Tests = serde_json::from_str(&tests_string).unwrap();
        let filter = Regex::new(self.filter.as_deref().unwrap_or_default()).unwrap();

        for job in tests.jobs {
            if filter.find(&job.name).is_none() {
                continue;
            }

            let mut command = Command::new("target/debug/url-cleaner");

            command.args([
                "--output-buffer", "0",
                "--no-read-cache",
                "--no-write-cache",
                "--cleaner", "engine/src/cleaner/bundled-cleaner.json"
            ]);

            let mut tmp_file_handles = Vec::new();

            if let Some(job_context) = job.job_context {
                let tmp_file = tmp_file(&format!("{TMP}/job_context.json"));
                tmp_file.file().write_all(&serde_json::to_vec(&job_context).unwrap()).unwrap();
                command.args(["--job-context", tmp_file.path()]);
                tmp_file_handles.push(tmp_file);
            }

            if let Some(params_diff) = job.params_diff {
                let tmp_file = tmp_file(&format!("{TMP}/params_diff.json"));
                tmp_file.file().write_all(&serde_json::to_vec(&params_diff).unwrap()).unwrap();
                command.args(["--params-diff", tmp_file.path()]);
                tmp_file_handles.push(tmp_file);
            }

            let (stdin_read , mut stdin_write ) = std::io::pipe().unwrap();
            let (stdout_read,     stdout_write) = std::io::pipe().unwrap();

            command.stdin(stdin_read);
            command.stdout(stdout_write);

            let _child = TerminateOnDrop(command.spawn().unwrap());

            let mut results = BufReader::new(stdout_read).lines();

            for test in job.tests {
                writeln!(stdin_write, "{}", test.task).unwrap();
                assert_eq!(results.next().unwrap().unwrap(), test.expect, "Task failed: {}", test.task);
            }
        }
    }
}
