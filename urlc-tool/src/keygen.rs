//! Keygen.

use crate::prelude::*;

/// Generate keys for Site.
#[derive(Debug, Parser)]
pub struct Args {
    /// The directory to put the keys in.
    #[arg(long, default_value = "keys")]
    pub out: PathBuf,
    /// Make a new CA key/cert pair.
    #[arg(long)]
    pub new_ca: bool,
    /// Make a new Site key/cert pair.
    #[arg(long)]
    pub new_site: bool,
    /// The IPs to add to the certificate.
    #[arg(long)]
    pub ips: Vec<std::net::IpAddr>,
    /// The domains to add to the certificate.
    #[arg(long)]
    pub domains: Vec<DomainHost<'static>>,
    /// Make a new CA key.
    #[arg(long)]
    pub new_ca_key: bool,
    /// Make a new CA cert.
    #[arg(long)]
    pub new_ca_crt: bool,
    /// Make a new Site key.
    #[arg(long)]
    pub new_site_key: bool,
    /// Make a new Site CSR.
    #[arg(long)]
    pub new_site_csr: bool,
    /// Make a new Site cert.
    #[arg(long)]
    pub new_site_crt: bool,
}

/// Turn string literals and expressions into OsStrs.
macro_rules! thing {
    ($($x:expr),*$(,)?) => {[$(std::ffi::OsStr::new($x)),*]};
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        std::fs::create_dir_all(&self.out).unwrap();

        let ca_key   = self.out.join("urlcs-ca.key");
        let ca_crt   = self.out.join("urlcs-ca.crt");
        let site_key = self.out.join("urlcs.key"   );
        let site_csr = self.out.join("urlcs.csr"   );
        let site_crt = self.out.join("urlcs.crt"   );

        let ca_key   = ca_key  .as_os_str();
        let ca_crt   = ca_crt  .as_os_str();
        let site_key = site_key.as_os_str();
        let site_csr = site_csr.as_os_str();
        let site_crt = site_crt.as_os_str();

        if !std::fs::exists(ca_key).unwrap() || self.new_ca_key || self.new_ca {
            assert_eq!(Command::new("openssl")
                .args(thing![
                    "genrsa",
                    "-out", ca_key, "2048",
                ]).spawn().unwrap().wait().unwrap().code(), Some(0));
        }

        if !std::fs::exists(ca_crt).unwrap() || self.new_ca_crt || self.new_ca {
            assert_eq!(Command::new("openssl")
                .args(thing![
                    "req", "-x509", "-new", "-noenc",
                    "-key", ca_key,
                    "-days", "365",
                    "-subj", "/CN=URL Cleaner Site CA",
                    "-out", ca_crt,
                ]).spawn().unwrap().wait().unwrap().code(), Some(0));
        }

        if !std::fs::exists(site_key).unwrap() || self.new_site_key || self.new_site {
            assert_eq!(Command::new("openssl")
                .args(thing![
                    "genrsa",
                    "-out", site_key, "2048"
                ]).spawn().unwrap().wait().unwrap().code(), Some(0));
        }

        if !std::fs::exists(site_csr).unwrap() || self.new_site_csr || self.new_site {
            let mut san = "subjectAltName=DNS:localhost,IP:::1,IP:127.0.0.1".to_string();

            for ip in self.ips {
                san.push_str(",IP:");
                san.push_str(&ip.to_string());
            }

            for domain in self.domains {
                san.push_str(",DNS:");
                san.push_str(domain.as_str());
            }

            assert_eq!(Command::new("openssl")
                .args(thing![
                    "req", "-new", "-noenc",
                    "-subj", "/CN=URL Cleaner Site",
                    "-addext", &san,
                    "-key", site_key,
                    "-out", site_csr,
                ]).spawn().unwrap().wait().unwrap().code(), Some(0));
        }

        if !std::fs::exists(site_crt).unwrap() || self.new_site_crt || self.new_site {
            assert_eq!(Command::new("openssl")
                .args(thing![
                    "x509", "-req",
                    "-CAkey", ca_key,
                    "-CA"   , ca_crt,
                    "-in"   , site_csr,
                    "-out"  , site_crt,
                    "-days" , "365",
                    "-copy_extensions", "copy",
                ]).spawn().unwrap().wait().unwrap().code(), Some(0));
        }
    }
}
