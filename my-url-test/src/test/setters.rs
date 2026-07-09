//! Test the setters.

use serde::Deserialize;

use crate::prelude::*;

/// The test data.
const DATA: &str = include_str!("setters.json");

/// Test the setters.
#[derive(Debug, Default, Parser)]
pub struct Args {
    /// The location of the tests.
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

/// The [`Test`]s.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tests {
    /** The comment.        **/ pub comment : Vec<String>,
    /** The href tests.     **/ pub href    : Vec<Test>,
    /** The protocol tests. **/ pub protocol: Vec<Test>,
    /** The username tests. **/ pub username: Vec<Test>,
    /** The password tests. **/ pub password: Vec<Test>,
    /** The hostname tests. **/ pub hostname: Vec<Test>,
    /** The host tests.     **/ pub host    : Vec<Test>,
    /** The port tests.     **/ pub port    : Vec<Test>,
    /** The pathname tests. **/ pub pathname: Vec<Test>,
    /** The search tests.   **/ pub search  : Vec<Test>,
    /** The hash tests.     **/ pub hash    : Vec<Test>,
}

/// A test.
#[derive(Debug, Deserialize)]
pub struct Test {
    /// The input URL.
    href: String,
    /// The new value.
    new_value: String,
    /// The expected output.
    expected: Expected,
}

/// A [`Test`]'s expected output.
#[derive(Debug, Deserialize)]
pub struct Expected {
    /// The href.
    href: String,
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

impl Tests {
    /// Do the tests.
    /// # Panics
    /// Panics when a test fails.
    pub fn r#do(self) {
        println!("Protocol:");

        for test in self.protocol {
            println!("  {test:?}");

            let mut x = MyUrl::new(&test.href).expect("???");

            let _ = x.canon_set_protocol(test.new_value);

            assert_identical(&x, &MyUrl::new(&test.expected.href).expect("???"));
        }

        println!();

        println!("Username:");

        for test in self.username {
            println!("  {test:?}");

            let mut x = MyUrl::new(&test.href).expect("???");

            let _ = x.canon_set_username(test.new_value);

            assert_identical(&x, &MyUrl::new(&test.expected.href).expect("???"));
        }

        println!();

        println!("Password:");

        for test in self.password {
            println!("  {test:?}");

            let mut x = MyUrl::new(&test.href).expect("???");

            let _ = x.canon_set_password(test.new_value);

            assert_identical(&x, &MyUrl::new(&test.expected.href).expect("???"));
        }

        println!();

        println!("Host:");

        for test in self.host {
            println!("  {test:?}");

            let mut x = MyUrl::new(&test.href).expect("???");

            let _ = x.canon_set_host(test.new_value);

            assert_identical(&x, &MyUrl::new(&test.expected.href).expect("???"));
        }

        println!();

        println!("Hostname:");

        for test in self.hostname {
            println!("  {test:?}");

            let mut x = MyUrl::new(&test.href).expect("???");

            let _ = x.canon_set_hostname(test.new_value);

            assert_identical(&x, &MyUrl::new(&test.expected.href).expect("???"));
        }

        println!();

        println!("Port:");

        for test in self.port {
            println!("  {test:?}");

            let mut x = MyUrl::new(&test.href).expect("???");

            let _ = x.canon_set_port(test.new_value);

            assert_identical(&x, &MyUrl::new(&test.expected.href).expect("???"));
        }

        println!();

        println!("Pathname:");

        for test in self.pathname {
            println!("  {test:?}");

            let mut x = MyUrl::new(&test.href).expect("???");

            let _ = x.canon_set_pathname(test.new_value);

            assert_identical(&x, &MyUrl::new(&test.expected.href).expect("???"));
        }

        println!();

        println!("Search:");

        for test in self.search {
            println!("  {test:?}");

            let mut x = MyUrl::new(&test.href).expect("???");

            let _ = x.canon_set_search(test.new_value);

            assert_identical(&x, &MyUrl::new(&test.expected.href).expect("???"));
        }

        println!();

        println!("Hash:");

        for test in self.hash {
            println!("  {test:?}");

            let mut x = MyUrl::new(&test.href).expect("???");

            let _ = x.canon_set_hash(test.new_value);

            assert_identical(&x, &MyUrl::new(&test.expected.href).expect("???"));
        }
    }
}

/// Assert that all of the URL's parts are identical.
fn assert_identical(x: &MyUrl, y: &MyUrl) {
    assert_eq!(x.as_str        (), y.as_str        (), "as_str"        );
    assert_eq!(x.scheme        (), y.scheme        (), "scheme"        );
    assert_eq!(x.maybe_username(), y.maybe_username(), "maybe_username");
    assert_eq!(x.username      (), y.username      (), "username"      );
    assert_eq!(x.maybe_password(), y.maybe_password(), "maybe_password");
    assert_eq!(x.password      (), y.password      (), "password"      );
    assert_eq!(x.host          (), y.host          (), "host"          );
    assert_eq!(x.port_num      (), y.port_num      (), "port_num"      );
    assert_eq!(x.port_str      (), y.port_str      (), "port_str"      );
    assert_eq!(x.path          (), y.path          (), "path"          );
    assert_eq!(x.query         (), y.query         (), "query"         );
    assert_eq!(x.fragment      (), y.fragment      (), "fragment"      );

    assert_eq!(x.canon_get_href    (), y.canon_get_href    (), "canon_get_href"    );
    assert_eq!(x.canon_get_protocol(), y.canon_get_protocol(), "canon_get_protocol");
    assert_eq!(x.canon_get_username(), y.canon_get_username(), "canon_get_username");
    assert_eq!(x.canon_get_password(), y.canon_get_password(), "canon_get_password");
    assert_eq!(x.canon_get_hostname(), y.canon_get_hostname(), "canon_get_hostname");
    assert_eq!(x.canon_get_host    (), y.canon_get_host    (), "canon_get_host"    );
    assert_eq!(x.canon_get_port    (), y.canon_get_port    (), "canon_get_port"    );
    assert_eq!(x.canon_get_pathname(), y.canon_get_pathname(), "canon_get_pathname");
    assert_eq!(x.canon_get_search  (), y.canon_get_search  (), "canon_get_search"  );
    assert_eq!(x.canon_get_hash    (), y.canon_get_hash    (), "canon_get_hash"    );
}
