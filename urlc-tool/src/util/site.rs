//! Site.

use super::prelude::*;

/// Start Site, waiting for it to be ready.
pub fn start_site(tls: bool) -> TerminateOnDrop {
    assert_no_site();

    let mut cmd = Command::new("target/release/url-cleaner-site");

    cmd.args(["--port", "9148"]);

    if tls {
        cmd.args([
            "--key", "urlc-tool/src/bench/urlcs-bench.key",
            "--cert", "urlc-tool/src/bench/urlcs-bench.crt"
        ]);
    }

    cmd.stdout(std::process::Stdio::null());
    cmd.stderr(std::process::Stdio::null());

    let mut child = cmd.spawn().unwrap();

    await_site(&mut child);

    TerminateOnDrop(child)
}

/// Assert no Site.
pub fn assert_no_site() {
    if std::net::TcpStream::connect("127.0.0.1:9148").is_ok() {
        panic!("Site is already started.");
    }
}

/// Await Site.
pub fn await_site(child: &mut std::process::Child) {
    while std::net::TcpStream::connect("127.0.0.1:9148").is_err() {
        if child.try_wait().unwrap().is_some() {
            panic!("Site failed to start.");
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
