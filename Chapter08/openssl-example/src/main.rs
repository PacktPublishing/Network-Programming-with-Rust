extern crate openssl;

use std::env;
use std::fs::File;
use std::io::Write;

use openssl::x509::{X509, X509Name};
use openssl::nid::Nid;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::error::ErrorStack;
use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MsbOption};
use openssl::hash::MessageDigest;

fn create_cert() -> Result<(X509, PKey<Private>), ErrorStack> {
    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(2)?;

    let serial_number = {
        let mut serial = BigNum::new()?;
        serial.rand(160, MsbOption::MAYBE_ZERO, false)?;
        serial.to_asn1_integer()?
    };
    cert_builder.set_serial_number(&serial_number)?;

    let mut name = X509Name::builder()?;
    name.append_entry_by_text("C", "UK")?;
    name.append_entry_by_text("CN", "Our common name")?;
    let cert_name = name.build();
    cert_builder.set_issuer_name(&cert_name)?;

    let not_before = Asn1Time::days_from_now(0)?;
    cert_builder.set_not_before(&not_before)?;

    let not_after = Asn1Time::days_from_now(365)?;
    cert_builder.set_not_after(&not_after)?;

    cert_builder.set_subject_name(&cert_name)?;

    let private_key = PKey::from_rsa(Rsa::generate(3072)?)?;
    cert_builder.set_pubkey(&private_key)?;

    cert_builder.sign(&private_key, MessageDigest::sha512())?;
    let cert = cert_builder.build();

    Ok((cert, private_key))
}

fn main() {
    if let Some(arg) = env::args().nth(1) {
        let (cert, _key) = create_cert().expect("could not create cert");
        let cert_data = cert.to_pem().expect("could not convert cert to pem");

        let mut cert_file = File::create(arg).expect("could not create cert file");
        cert_file
            .write_all(&cert_data)
            .expect("failed to write cert");

        let subject = cert.subject_name();
        let cn = subject
            .entries_by_nid(Nid::COMMONNAME)
            .next()
            .expect("failed to get subject");
        println!(
            "{}",
            String::from_utf8(cn.data().as_slice().to_vec()).unwrap()
        );
    } else {
        eprintln!("Expected at least one argument");
        std::process::exit(1);
    }
}
