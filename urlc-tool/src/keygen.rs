//! Keygen.

use crate::prelude::*;

/// Generate keys for Site.
#[derive(Debug, Parser)]
pub struct Args {
    /// The directory to pit the keys in.
    #[arg(long, default_value = "keys")]
    pub out: PathBuf
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        crate::bench::prelude::fresh_dir(self.out);

        assert_eq!(Command::new("openssl")
            .args(["req", "-x509", "-noenc"])
            .args(["-newkey", "rsa:2048"])
            .args(["-keyout", "keys/urlcs-ca.key"])
            .args(["-out", "keys/urlcs-ca.crt"])
            .args(["-days", "365"])
            .args(["-subj", "/CN=URL Cleaner Site CA"])
            .spawn().unwrap().wait().unwrap().code(), Some(0));

        assert_eq!(Command::new("openssl")
            .args(["req", "-noenc"])
            .args(["-newkey", "rsa:2048"])
            .args(["-keyout", "keys/urlcs.key"])
            .args(["-out", "keys/urlcs.csr"])
            .args(["-subj", "/CN=URL Cleaner Site"])
            .args(["-addext", "subjectAltName=DNS:localhost,IP:::1,IP:127.0.0.1"])
            .spawn().unwrap().wait().unwrap().code(), Some(0));

        assert_eq!(Command::new("openssl")
            .args(["x509", "-req"])
            .args(["-in", "keys/urlcs.csr"])
            .args(["-CA", "keys/urlcs-ca.crt"])
            .args(["-CAkey", "keys/urlcs-ca.key"])
            .args(["-out", "keys/urlcs.crt"])
            .args(["-days", "365"])
            .args(["-copy_extensions", "copy"])
            .spawn().unwrap().wait().unwrap().code(), Some(0));
    }
}
