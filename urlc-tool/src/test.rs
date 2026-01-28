//! Do tests.

use serde::Deserialize;

use super::prelude::*;

/// Do tests.
#[derive(Debug, Parser)]
pub struct Args {
    /// The tests file.
    #[arg(long, default_value = "urlc-tool/tests.json")]
    pub tests: PathBuf,
    /// The filter to decide which jobs to do.
    #[arg(long)]
    pub filter: Option<String>,
    /// Assert the cleaner's suitability.
    #[arg(long)]
    pub assert_suitability: bool
}

/// A tests file.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Tests {
    /// The [`Job`]s to test.
    pub jobs: Vec<Job>
}

/// A job.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Job {
    /// The name of the job.
    pub name: String,
    /// The `--job-context`.
    #[serde(default = "empty_json_object")]
    pub job_context: serde_json::Value,
    /// The `--params-diff`.
    #[serde(default = "empty_json_object")]
    pub params_diff: serde_json::Value,
    /// The [`Test`]s.
    pub tests: Vec<Test>
}

/// Get an empty [`serde_json::Value::Object`].
fn empty_json_object() -> serde_json::Value {
    serde_json::Value::Object(Default::default())
}

/// A test.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Test {
    /// The task.
    task: serde_json::Value,
    /// The expected result.
    expect: String
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        if self.assert_suitability {
            assert_eq!(Command::new("target/debug/url-cleaner")
                .arg("--assert-suitability")
                .spawn().unwrap().wait().unwrap().code(), Some(0));
        }

        let tests: Tests = serde_json::from_str(&std::fs::read_to_string(self.tests).unwrap()).unwrap();
        let filter = Regex::new(&self.filter.unwrap_or_default()).unwrap();

        for job in tests.jobs {
            if filter.find(&job.name).is_none() {
                continue;
            }

            let job_context = serde_json::to_string(&job.job_context).unwrap();
            let params_diff = serde_json::to_string(&job.params_diff).unwrap();

            println!("Job:");
            println!("  Name: {}", job.name);
            println!("  Params diff: {params_diff}");
            println!("  Job context: {job_context}");
            println!("  Tests:");

            new_file("urlc-tool/tmp/test/job_context.json").write_all(job_context.as_bytes()).unwrap();
            new_file("urlc-tool/tmp/test/params_diff.json").write_all(params_diff.as_bytes()).unwrap();

            let (ir, mut iw) = std::io::pipe().unwrap();
            let (or,     ow) = std::io::pipe().unwrap();

            let mut lines = BufReader::new(or).lines();

            let mut child = Command::new("target/debug/url-cleaner");

            child.args([
                "--no-read-cache",
                "--no-write-cache",
                "--job-context", "urlc-tool/tmp/test/job_context.json",
                "--params-diff", "urlc-tool/tmp/test/params_diff.json",
            ]);

            child.stdin(ir);
            child.stdout(ow);

            let child = child.spawn().unwrap();

            for test in job.tests {
                let task = match test.task {
                    serde_json::Value::String(x) => x,
                    x => serde_json::to_string(&x).unwrap()
                };

                println!("    {task}");

                writeln!(iw, "{task}").unwrap();
                let result = lines.next().unwrap().unwrap();
                assert_eq!(result, test.expect);

                writeln!(iw, "{result}").unwrap();
                let reresult = lines.next().unwrap().unwrap();
                assert_eq!(result, reresult);
            }

            child.terminate();

            std::fs::remove_file("urlc-tool/tmp/test/job_context.json").unwrap();
            std::fs::remove_file("urlc-tool/tmp/test/params_diff.json").unwrap();
        }
    }
}
