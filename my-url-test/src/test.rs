//! Tests.

use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// `urltestdata.json`].
const DATA: &str = include_str!("urltestdata.json");

/// Test the implementation.
#[derive(Debug, Parser)]
pub struct Args {
    /// The location of the urltestdata.json.
    #[arg(long)]
    pub data: Option<PathBuf>,
}

impl Args {
    /// Do the command.
    /// # Panics
    /// If failing to load/parse the tests, panics.
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
        let href     = self.href;
        let scheme   = self.protocol.strip_suffix(":").expect("???");
        let username = self.username;
        let password = self.password;
        let host     = href.strip_prefix(scheme).expect("???").starts_with("://").then_some(&*self.hostname);
        let port_str = (!self.port.is_empty()).then_some(&*self.port);
        let port     = port_str.map(|x| x.parse().expect("???"));
        let path     = self.pathname;
        let query    = href.strip_suffix(&self.hash).expect("???").contains('?').then_some(self.search.strip_prefix('?').unwrap_or_default());
        let fragment = href                                       .contains('#').then_some(self.hash  .strip_prefix('#').unwrap_or_default());

        println!("Pass");
        println!("  base : {:?}", self.base );
        println!("  input: {:?}", self.input);
        println!("    href    : {href:?}"    );
        println!("    scheme  : {scheme:?}"  );
        println!("    username: {username:?}");
        println!("    password: {password:?}");
        println!("    host    : {host:?}"    );
        println!("    port_str: {port_str:?}");
        println!("    path    : {path:?}"    );
        println!("    query   : {query:?}"   );
        println!("    fragment: {fragment:?}");

        let url = match self.base {
            Some(base) => {
                let mut url = MyUrl::new(&base).expect("Base invalid");
                url.join(&self.input).expect("Join invalid");
                url
            },
            None => MyUrl::new(&self.input).expect("Baseless input invalid")
        };

        assert_eq!(url.as_str  (), href    , "href"    );
        assert_eq!(url.scheme  (), scheme  , "scheme"  );
        assert_eq!(url.username(), username, "username");
        assert_eq!(url.password(), password, "password");
        assert_eq!(url.host    (), host    , "host"    );
        assert_eq!(url.port_str(), port_str, "port_str");
        assert_eq!(url.port    (), port    , "port"    );
        assert_eq!(url.path    (), path    , "path"    );
        assert_eq!(url.query   (), query   , "query"   );
        assert_eq!(url.fragment(), fragment, "fragment");
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
