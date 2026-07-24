//! Keygen.

use std::fmt::Write;
use std::path::PathBuf;
use std::process::Command;

use better_url::prelude::*;

use crate::*;

/// Generate TLS keys for Site using OpenSSL.
#[derive(Debug, Parser)]
pub struct Args {
    /// The directory to put the keys in.
    #[arg(long, default_value = "keys")]
    pub out: PathBuf,
    /// The IPs to add to the certificate.
    #[arg(long)]
    pub ips: Vec<std::net::IpAddr>,
    /// The domains to add to the certificate.
    #[arg(long)]
    pub domains: Vec<DomainHost<'static>>,

    /** OVerwrite the existing CA key.    **/ #[arg(long)] pub overwrite_ca_key   : bool,
    /** OVerwrite the existing CA cert.   **/ #[arg(long)] pub overwrite_ca_cert  : bool,
    /** Overwrite the existing Site key.  **/ #[arg(long)] pub overwrite_site_key : bool,
    /** Overwrite the existing Site csr.  **/ #[arg(long)] pub overwrite_site_csr : bool,
    /** Overwrite the existing Site cert. **/ #[arg(long)] pub overwrite_site_cert: bool,
}

/// [`Args::do`].
#[derive(Debug, Error)]
pub enum KeygenError {}

/// Basic [`Command`] generation.
macro_rules! cmd {
    ($cmd:expr$(, $arg:expr)*) => {
        assert_eq!(
            Command::new($cmd)$(.arg($arg))*.spawn().expect("???").wait().expect("???").code(),
            Some(0),
        )
    }
}

impl Args {
    /// Do the command.
    pub async fn r#do(self) -> Result<(), KeygenError> {
        let mut san = "subjectAltName=DNS:localhost,IP:::1,IP:127.0.0.1".to_string();

        for ip     in self.ips     {write!(san, ",IP:{ip}"     ).expect("???");}
        for domain in self.domains {write!(san, ",DNS:{domain}").expect("???");}

        let ca_key    = &self.out.join("urlcs-ca.key");
        let ca_cert   = &self.out.join("urlcs-ca.crt");
        let site_key  = &self.out.join("urlcs.key"   );
        let site_csr  = &self.out.join("urlcs.csr"   );
        let site_cert = &self.out.join("urlcs.crt"   );

        std::fs::create_dir_all(self.out).expect("???");



        if !std::fs::exists(ca_key).expect("???") || self.overwrite_ca_key {
            println!("Generating CA key...");

            cmd!("openssl", "genrsa", "-out", ca_key, "2048");
        } else {
            println!("Not overwriting CA key.");
        }



        if !std::fs::exists(ca_cert).expect("???") || self.overwrite_ca_cert {
            println!("Generating CA cert...");

            cmd!(
                "openssl", "req", "-x509", "-new", "-noenc",
                    "-key" , ca_key,
                    "-days", "365",
                    "-subj", "/CN=URL Cleaner Site CA",
                    "-out" , ca_cert
            );
        } else {
            println!("Not overwriting CA cert.")
        }



        if !std::fs::exists(site_key).expect("???") || self.overwrite_site_key {
            println!("Generating Site key...");

            cmd!("openssl", "genrsa", "-out", site_key, "2048");
        } else {
            println!("Not overwriting Site key.");
        }



        if !std::fs::exists(site_csr).expect("???") || self.overwrite_site_csr {
            println!("Generating Site CSR...");

            cmd!(
                "openssl", "req", "-new", "-noenc",
                    "-subj"  , "/CN=URL Cleaner Site",
                    "-addext", &san,
                    "-key"   , site_key,
                    "-out"   , site_csr
            );
        } else {
            println!("Not overwriting Site CSR.");
        }



        if !std::fs::exists(site_cert).expect("???") || self.overwrite_site_cert {
            println!("Generating Site cert...");

            cmd!(
                "openssl", "x509", "-req",
                    "-CAkey"          , ca_key,
                    "-CA"             , ca_cert,
                    "-in"             , site_csr,
                    "-out"            , site_cert,
                    "-days"           , "365",
                    "-copy_extensions", "copy"
            );
        } else {
            println!("Not overwriting Site cert.")
        }



        Ok(())
    }
}
