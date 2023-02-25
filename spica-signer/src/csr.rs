use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use openssl::{
    asn1::{Asn1Integer, Asn1Time},
    bn::{BigNum, MsbOption},
    pkey::PKey,
    x509::X509,
};
use serde::{Deserialize, Serialize};

use crate::{
    cert::CACert,
    openssl::{OpenSSLOpts, X509NameContainer},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CertReq {
    pub not_before: SystemTime,
    pub not_after: SystemTime,
    pub serial: Option<String>,
    pub subject_name: X509NameContainer,
    #[serde(rename = "openssl-opt", default)]
    pub openssl_opt: OpenSSLOpts,
    pub pubkey_pem: String,
}

impl CertReq {
    pub fn sign(&self, ca: &CACert) -> Result<String> {
        let mut builder = X509::builder()?;

        builder.set_version(2)?;
        builder.set_not_before(
            Asn1Time::from_unix(self.not_before.duration_since(UNIX_EPOCH)?.as_secs() as i64)?
                .as_ref(),
        )?;
        builder.set_not_after(
            Asn1Time::from_unix(self.not_after.duration_since(UNIX_EPOCH)?.as_secs() as i64)?
                .as_ref(),
        )?;
        let serial = match &self.serial {
            Some(serial) => BigNum::from_hex_str(&serial)?,
            None => Self::rand_serial()?,
        };
        builder.set_serial_number(Asn1Integer::from_bn(&serial)?.as_ref())?;
        builder.set_issuer_name(&ca.x509_name)?;
        builder.set_subject_name(&self.subject_name)?;
        let pubkey = PKey::public_key_from_pem(self.pubkey_pem.as_bytes())?;
        builder.set_pubkey(&pubkey)?;

        let cert = builder.build();
        Ok(String::from_utf8(cert.to_pem()?)?)
    }

    fn rand_serial() -> Result<BigNum> {
        let mut bn = BigNum::new()?;
        bn.rand(159, MsbOption::MAYBE_ZERO, false)?;
        Ok(bn)
    }
}
