//! Test the setters.

use crate::prelude::*;

/// The test data.
const DATA: &str = include_str!("data/setters_tests.json");

#[test]
fn test_setters() {
    serde_json::from_str::<Tests>(DATA).expect("To parse the tests.").r#do()
}

/// The [`Test`]s.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Tests {
    /** The comment.        **/ comment : Vec<String>,
    /** The href tests.     **/ href    : Vec<Test>,
    /** The protocol tests. **/ protocol: Vec<Test>,
    /** The username tests. **/ username: Vec<Test>,
    /** The password tests. **/ password: Vec<Test>,
    /** The hostname tests. **/ hostname: Vec<Test>,
    /** The host tests.     **/ host    : Vec<Test>,
    /** The port tests.     **/ port    : Vec<Test>,
    /** The pathname tests. **/ pathname: Vec<Test>,
    /** The search tests.   **/ search  : Vec<Test>,
    /** The hash tests.     **/ hash    : Vec<Test>,
}

/// A test.
#[derive(Debug, Deserialize)]
struct Test {
    /// The input URL.
    href: BetterUrl,
    /// The new value.
    new_value: String,
    /// The expected output.
    expected: Expected,
}

/// A [`Test`]'s expected output.
#[derive(Debug, Deserialize)]
struct Expected {
    /// The href.
    href: BetterUrl,
}

impl Tests {
    /// Do the tests.
    /// # Panics
    /// Panics when a test fails.
    fn r#do(self) {
        println!("Protocol:");
        for mut test in self.protocol {
            println!("  {} + {} -> {}", test.href, test.new_value, test.expected.href);
            let _ = test.href.canon_set_protocol(test.new_value);
            assert_identical(&test.href, &test.expected.href);
        }
        println!();

        println!("Username:");
        for mut test in self.username {
            println!("  {} + {} -> {}", test.href, test.new_value, test.expected.href);
            let _ = test.href.canon_set_username(test.new_value);
            assert_identical(&test.href, &test.expected.href);
        }
        println!();

        println!("Password:");
        for mut test in self.password {
            println!("  {} + {} -> {}", test.href, test.new_value, test.expected.href);
            let _ = test.href.canon_set_password(test.new_value);
            assert_identical(&test.href, &test.expected.href);
        }
        println!();

        println!("Host:");
        for mut test in self.host {
            println!("  {} + {} -> {}", test.href, test.new_value, test.expected.href);
            let _ = test.href.canon_set_host(test.new_value);
            assert_identical(&test.href, &test.expected.href);
        }
        println!();

        println!("Hostname:");
        for mut test in self.hostname {
            println!("  {} + {} -> {}", test.href, test.new_value, test.expected.href);
            let _ = test.href.canon_set_hostname(test.new_value);
            assert_identical(&test.href, &test.expected.href);
        }
        println!();

        println!("Port:");
        for mut test in self.port {
            println!("  {} + {} -> {}", test.href, test.new_value, test.expected.href);
            let _ = test.href.canon_set_port(test.new_value);
            assert_identical(&test.href, &test.expected.href);
        }
        println!();

        println!("Pathname:");
        for mut test in self.pathname {
            println!("  {} + {} -> {}", test.href, test.new_value, test.expected.href);
            let _ = test.href.canon_set_pathname(test.new_value);
            assert_identical(&test.href, &test.expected.href);
        }
        println!();

        println!("Search:");
        for mut test in self.search {
            println!("  {} + {} -> {}", test.href, test.new_value, test.expected.href);
            let _ = test.href.canon_set_search(test.new_value);
            assert_identical(&test.href, &test.expected.href);
        }
        println!();

        println!("Hash:");
        for mut test in self.hash {
            println!("  {} + {} -> {}", test.href, test.new_value, test.expected.href);
            let _ = test.href.canon_set_hash(test.new_value);
            assert_identical(&test.href, &test.expected.href);
        }
        println!();
    }
}

/// Assert that all of the URL's parts are identical.
fn assert_identical(x: &BetterUrl, y: &BetterUrl) {
    assert_eq!(x.as_str              (), y.as_str              (), "as_str"              );
    assert_eq!(x.scheme              (), y.scheme              (), "scheme"              );
    assert_eq!(x.visible_username_str(), y.visible_username_str(), "visible_username_str");
    assert_eq!(x.username            (), y.username            (), "username"            );
    assert_eq!(x.visible_password_str(), y.visible_password_str(), "visible_password_str");
    assert_eq!(x.password            (), y.password            (), "password"            );
    assert_eq!(x.host                (), y.host                (), "host"                );
    assert_eq!(x.port_num            (), y.port_num            (), "port_num"            );
    assert_eq!(x.port_str            (), y.port_str            (), "port_str"            );
    assert_eq!(x.path                (), y.path                (), "path"                );
    assert_eq!(x.query               (), y.query               (), "query"               );
    assert_eq!(x.fragment            (), y.fragment            (), "fragment"            );

    assert_eq!(x.canon_get_href      (), y.canon_get_href      (), "canon_get_href"      );
    assert_eq!(x.canon_get_protocol  (), y.canon_get_protocol  (), "canon_get_protocol"  );
    assert_eq!(x.canon_get_username  (), y.canon_get_username  (), "canon_get_username"  );
    assert_eq!(x.canon_get_password  (), y.canon_get_password  (), "canon_get_password"  );
    assert_eq!(x.canon_get_hostname  (), y.canon_get_hostname  (), "canon_get_hostname"  );
    assert_eq!(x.canon_get_host      (), y.canon_get_host      (), "canon_get_host"      );
    assert_eq!(x.canon_get_port      (), y.canon_get_port      (), "canon_get_port"      );
    assert_eq!(x.canon_get_pathname  (), y.canon_get_pathname  (), "canon_get_pathname"  );
    assert_eq!(x.canon_get_search    (), y.canon_get_search    (), "canon_get_search"    );
    assert_eq!(x.canon_get_hash      (), y.canon_get_hash      (), "canon_get_hash"      );
}
