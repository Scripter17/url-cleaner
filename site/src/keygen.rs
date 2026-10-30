//! Keygen.

use std::io::Write;
use std::path::PathBuf;

use crate::*;

/// Generates files to use URL Cleaner Site with TLS.
///
/// For maximum compatibility, uses 2048-bit RSA + SHA256 and limits both the Certificate Authority cert and Site cert to expire after 365 days.
///
/// - A Certificate Authority is used to please Rustls.
///
/// - Everything else is to please ios.
///
/// Creates 3 files in the directory specified by --out: urlcs-ca.crt, urlcs.key, and urlcs.crt.
///
/// The Certificate Authority key is never written to disk, preventing anyone who would've been able to read it to man-in-the-middle every TLS connection done by computers that trust it.
///
/// The Site key, urlcs.key, MUST BE KEPT SECRET to only you and your instance of URL Cleaner Site.
///
/// The Certificate Authority and Site certs, urlcs-ca.crt and urlcs.crt, are safe to share with anyone.
///
/// To use URL Cleaner Site with the generated TLS keypair:
///
/// 1. Add urlcs-ca.crt to your client's computer's trusted Certificate Authority certificate list.
///
/// 2. Configure clients to use HTTPS/WSS instead of HTTP/WS.
///
/// 3. Start URL Cleaner Site with --key and --cert set to urlcs.key and urlcs.crt.
#[derive(Debug, Parser)]
pub struct Args {
    /// The directory to put the output in.
    #[arg(long, default_value = "urlcs-tls-stuff")]
    pub out: PathBuf,
    /// The IPs to add to Site's certificate in addition to 127.0.0.1 and ::1.
    #[arg(long, num_args = 0..)]
    pub ips: Vec<std::net::IpAddr>,
    /// The domains to add to Site's certificate in addition to localhost.
    #[arg(long, num_args = 0..)]
    pub domains: Vec<rcgen::string::Ia5String>,
    /// Overwrite existing keys and certs.
    #[arg(long)]
    pub overwrite: bool,
}

/// [`Args::do`].
#[derive(Debug, Error)]
pub enum KeygenError {}

impl Args {
    /// Do the command.
    pub async fn r#do(self) -> Result<(), KeygenError> {
        if std::fs::exists(&self.out).expect("To be able to check if the output directory exists") && !self.overwrite {
            println!("Existing output dir found.");
            println!("If you want to overwrite the CA cert and Site key and cert, re-run with --overwrite.");
            return Ok(());
        }



        let ca_cert_file   = self.out.join("urlcs-ca.crt");
        let site_key_file  = self.out.join("urlcs.key"   );
        let site_cert_file = self.out.join("urlcs.crt"   );

        let not_before = time::OffsetDateTime::now_utc();
        let not_after  = not_before + time::Duration::DAY * 365;

        std::fs::create_dir_all(&self.out).expect("Creating output directory to work");



        // Generate Certificate Authority key.

        let ca_key = rcgen::KeyPair::generate_rsa_for(&rcgen::PKCS_RSA_SHA256, rcgen::RsaKeySize::_2048).expect("Generating Certificate Authority key to work");

        // Generate Certificate Authority params.

        let mut ca_params = rcgen::CertificateParams::default();

        ca_params.distinguished_name.push(rcgen::DnType::CommonName, "URL Cleaner Site Certificate Authority");

        ca_params.is_ca      = rcgen::IsCa::Ca(rcgen::BasicConstraints::Constrained(0));
        ca_params.not_before = not_before;
        ca_params.not_after  = not_after ;

        // Generate Certificate Authority cert.

        let ca_cert = ca_params.self_signed(&ca_key).expect("Signing Certificate Authority cert to work");



        // Generate Site key.

        let site_key = rcgen::KeyPair::generate_rsa_for(&rcgen::PKCS_RSA_SHA256, rcgen::RsaKeySize::_2048).expect("Generating Site key to work");

        // Generate Site params.

        let mut site_params = rcgen::CertificateParams::default();

        site_params.distinguished_name.push(rcgen::DnType::CommonName, "URL Cleaner Site");

        site_params.is_ca      = rcgen::IsCa::ExplicitNoCa;
        site_params.not_before = not_before;
        site_params.not_after  = not_after ;

        site_params.subject_alt_names.push(rcgen::SanType::DnsName  ("localhost".parse().expect("???")));
        site_params.subject_alt_names.push(rcgen::SanType::IpAddress("127.0.0.1".parse().expect("???")));
        site_params.subject_alt_names.push(rcgen::SanType::IpAddress("::1"      .parse().expect("???")));

        site_params.subject_alt_names.extend(self.domains.into_iter().map(rcgen::SanType::DnsName  ));
        site_params.subject_alt_names.extend(self.ips    .into_iter().map(rcgen::SanType::IpAddress));

        // Generate Site cert.

        let site_cert = site_params.signed_by(&site_key, &rcgen::Issuer::new(ca_params, &ca_key)).expect("Signing Site cert to work");



        // Write files.

        let mut opener = std::fs::OpenOptions::new();
        opener.read(true).write(true).create(true).truncate(true);

        opener.open(ca_cert_file  ).expect("Opening Certificate Authority cert file to work").write_all(ca_cert  .pem          ().as_bytes()).expect("Writing Certificate Authority cert to work");
        opener.open(site_key_file ).expect("Opening Site key file to work"                  ).write_all(site_key .serialize_pem().as_bytes()).expect("Writing Site key to work"                  );
        opener.open(site_cert_file).expect("Opening Site cert file to work"                 ).write_all(site_cert.pem          ().as_bytes()).expect("Writing Site cert to work"                 );



        Ok(())
    }
}
