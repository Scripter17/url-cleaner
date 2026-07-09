//! Tests.

use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// The test data.
const DATA: &str = include_str!("parser.json");

/// Test the implementation.
#[derive(Debug, Default, Parser)]
pub struct Args {
    /// The location of the urltestdata.json.
    #[arg(long)]
    pub data: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        Args::default().r#do();
    }
}

impl Args {
    /// Do the command.
    /// # Panics
    /// Panics when failing to load/parse the tests.
    pub fn r#do(self) {
        let data = match self.data {
            Some(path) => Cow::Owned(std::fs::read_to_string(path).expect("To load the tests.")),
            None       => Cow::Borrowed(DATA)
        };

        serde_json::from_str::<Tests>(&data).expect("To parse the tests.").r#do()
    }
}

/// [`TestOrComment`]s.
#[repr(transparent)]
#[derive(Debug, Deserialize, Serialize)]
struct Tests(pub Vec<TestOrComment>);

/// A [`Test`] or a [`Comment`].
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TestOrComment {
    /** [`Test`].    **/ Test   (Box<Test>),
    /** [`Comment`]. **/ Comment(Comment  ),
}

/// A comment.
#[repr(transparent)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Comment(pub String);

/// Either a [`SuccessTest`] or a [`FailureTest`].
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Test {
    /** [`SuccessTest`]. **/ Success(Box<SuccessTest>),
    /** [`FailureTest`]. **/ Failure(    FailureTest ),
}

/// A test that succeeds.
#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessTest {
    /** The input.    **/ pub input   : String,
    /** The base.     **/ pub base    : Option<String>,
    /** The href.     **/ pub href    : String,
    /** The protocol. **/ pub protocol: String,
    /** The username. **/ pub username: String,
    /** The password. **/ pub password: String,
    /** The hostname. **/ pub hostname: String,
    /** The host.     **/ pub host    : String,
    /** The port.     **/ pub port    : String,
    /** The pathname. **/ pub pathname: String,
    /** The search.   **/ pub search  : String,
    /** The hash.     **/ pub hash    : String,
}

/// A test that returns failure.
#[derive(Debug, Deserialize, Serialize)]
pub struct FailureTest {
    /** The input. **/ pub input: String,
    /** The base.  **/ pub base : Option<String>,
}

impl Tests {
    /// Do the tests.
    pub fn r#do(self) {
        let len = self.0.len();

        for (i, x) in (1..).zip(self.0) {
            println!("{i}/{len}");

            x.r#do();
        }
    }
}

impl TestOrComment {
    /// Do the [`Test`] or print the [`Comment`].
    pub fn r#do(self) {
        match self {
            Self::Test(test) => test.r#do(),
            Self::Comment(x) => println!("Comment: {}", x.0),
        }
        println!();
    }
}

impl Test {
    /// Either [`SuccessTest::do`] or [`FailureTest::do`].
    pub fn r#do(self) {
        match self {
            Self::Success(test) => test.r#do(),
            Self::Failure(test) => test.r#do(),
        }
    }
}

impl SuccessTest {
    /// Do the test.
    /// # Panics
    /// If the test fails, panics.
    pub fn r#do(self) {

        println!("Pass");
        println!("  base : {:?}", self.base );
        println!("  input: {:?}", self.input);
        println!("    href    : {:?}", self.href    );
        println!("    protocol: {:?}", self.protocol);
        println!("    username: {:?}", self.username);
        println!("    password: {:?}", self.password);
        println!("    host    : {:?}", self.host    );
        println!("    hostname: {:?}", self.hostname);
        println!("    port    : {:?}", self.port    );
        println!("    pathname: {:?}", self.pathname);
        println!("    search  : {:?}", self.search  );
        println!("    hash    : {:?}", self.hash    );

        let url = match self.base {
            Some(base) => {
                let mut url = MyUrl::new(&base).expect("Base invalid");
                url.join(&self.input).expect("Join invalid");
                url
            },
            None => MyUrl::new(&self.input).expect("Baseless input invalid")
        };

        assert_eq!(url.canon_get_href    (), self.href    , "href"    );
        assert_eq!(url.canon_get_protocol(), self.protocol, "protocol");
        assert_eq!(url.canon_get_username(), self.username, "username");
        assert_eq!(url.canon_get_password(), self.password, "password");
        assert_eq!(url.canon_get_host    (), self.host    , "host"    );
        assert_eq!(url.canon_get_hostname(), self.hostname, "hostname");
        assert_eq!(url.canon_get_port    (), self.port    , "port"    );
        assert_eq!(url.canon_get_pathname(), self.pathname, "pathname");
        assert_eq!(url.canon_get_search  (), self.search  , "search"  );
        assert_eq!(url.canon_get_hash    (), self.hash    , "hash"    );
    }
}

impl FailureTest {
    /// Do the test.
    /// # Panics
    /// If the test fails, panics.
    pub fn r#do(self) {
        println!("Fail");
        println!("  base : {:?}", self.base);
        println!("  input: {:?}", self.input);

        match self.base {
            Some(base) => if let Ok(mut x) = MyUrl::new(&base) {
                x.join(&self.input).expect_err("Join valid.");
            },
            None => {
                MyUrl::new(&self.input).expect_err("Baseless input valid");
            }
        }
    }
}
