//! Tests.

use crate::prelude::*;

/// The test data.
const DATA: &str = include_str!("parser.json");

#[test]
fn test_parser() {
    serde_json::from_str::<Tests>(DATA).expect("To parse the tests.").r#do()
}

/// [`TestOrComment`]s.
#[repr(transparent)]
#[derive(Debug, Deserialize)]
struct Tests(Vec<TestOrComment>);

/// A [`Test`] or a [`Comment`].
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TestOrComment {
    /** [`Test`].    **/ Test   (Box<Test>),
    /** [`Comment`]. **/ Comment(Comment  ),
}

/// A comment.
#[repr(transparent)]
#[derive(Debug, Deserialize)]
struct Comment(String);

/// Either a [`SuccessTest`] or a [`FailureTest`].
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Test {
    /** [`SuccessTest`]. **/ Success(Box<SuccessTest>),
    /** [`FailureTest`]. **/ Failure(    FailureTest ),
}

/// A test that succeeds.
#[derive(Debug, Deserialize)]
struct SuccessTest {
    /** The input.    **/ input   : String,
    /** The base.     **/ base    : Option<BetterUrl>,
    /** The href.     **/ href    : String,
    /** The protocol. **/ protocol: String,
    /** The username. **/ username: String,
    /** The password. **/ password: String,
    /** The hostname. **/ hostname: String,
    /** The host.     **/ host    : String,
    /** The port.     **/ port    : String,
    /** The pathname. **/ pathname: String,
    /** The search.   **/ search  : String,
    /** The hash.     **/ hash    : String,
}

/// A test that returns failure.
#[derive(Debug, Deserialize)]
struct FailureTest {
    /** The input. **/ input: String,
    /** The base.  **/ base : Option<BetterUrl>,
}

impl Tests {
    /// Do the tests.
    fn r#do(self) {
        let len = self.0.len();

        for (i, x) in (1..).zip(self.0) {
            println!("{i}/{len}");

            x.r#do();
        }
    }
}

impl TestOrComment {
    /// Do the [`Test`] or print the [`Comment`].
    fn r#do(self) {
        match self {
            Self::Test   (x) => x.r#do(),
            Self::Comment(x) => println!("Comment: {}", x.0),
        }
        println!();
    }
}

impl Test {
    /// Either [`SuccessTest::do`] or [`FailureTest::do`].
    fn r#do(self) {
        match self {
            Self::Success(x) => x.r#do(),
            Self::Failure(x) => x.r#do(),
        }
    }
}

impl SuccessTest {
    /// Do the test.
    /// # Panics
    /// If the test fails, panics.
    fn r#do(self) {
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
            Some(mut url) => {
                url.join(&self.input).expect("Input to be a valid join.");
                url
            },
            None => BetterUrl::new(self.input).expect("Input to be a valid URL.")
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
    fn r#do(self) {
        println!("Fail");
        println!("  base : {:?}", self.base);
        println!("  input: {:?}", self.input);

        match self.base {
            Some(mut url) => {url      .join(&self.input).expect_err("Input shouldn't be a valid join.");},
            None          => {BetterUrl::new(&self.input).expect_err("Input shouldn't be a valid URL." );},
        }
    }
}

