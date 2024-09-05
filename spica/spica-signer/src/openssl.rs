use std::{
    collections::HashMap,
    fmt,
    ops::{Deref, DerefMut},
};

use anyhow::Result;
use base64::Engine;
use lazy_static::lazy_static;
use openssl::{
    conf::{Conf, ConfMethod, ConfRef},
    ec::{EcGroup, EcKey},
    nid::Nid,
    pkey::Private,
    x509::{X509Extension, X509Name, X509v3Context},
};
use serde::{de::Visitor, Deserialize, Serialize};

lazy_static! {
    static ref CONF: Conf = Conf::new(ConfMethod::default()).unwrap();
}

pub fn get_ossl_conf() -> &'static ConfRef {
    &CONF
}

pub type OpenSSLOpts = HashMap<String, String>;

pub trait OpenSSLOptsExt {
    fn to_exts(&self, ctx: &X509v3Context) -> Result<Vec<X509Extension>>;
}

impl OpenSSLOptsExt for OpenSSLOpts {
    fn to_exts(&self, ctx: &X509v3Context) -> Result<Vec<X509Extension>> {
        let mut exts = Vec::<X509Extension>::new();
        for (k, v) in self.iter() {
            let ext = X509Extension::new(Some(&CONF), Some(ctx), k, v)?;
            exts.push(ext);
        }
        Ok(exts)
    }
}

pub struct X509NameContainer(pub X509Name);

impl X509NameContainer {
    pub fn from_der(der: &[u8]) -> Result<X509NameContainer> {
        Ok(X509NameContainer(X509Name::from_der(der)?))
    }

    pub fn from_picky(name: picky_asn1_x509::Name) -> Result<X509NameContainer> {
        Ok(X509NameContainer(X509Name::from_der(
            picky_asn1_der::to_vec(&name)?.as_slice(),
        )?))
    }
}

impl fmt::Debug for X509NameContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("X509Name").finish()
    }
}

impl fmt::Display for X509NameContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("X509Name")
    }
}

impl Deref for X509NameContainer {
    type Target = X509Name;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for X509NameContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<X509Name> for X509NameContainer {
    fn from(value: X509Name) -> Self {
        Self(value)
    }
}

impl Serialize for X509NameContainer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            base64::engine::general_purpose::STANDARD
                .encode(
                    self.0
                        .to_der()
                        .map_err(|e| serde::ser::Error::custom(e.to_string()))?,
                )
                .as_str(),
        )
    }
}

impl<'de> Deserialize<'de> for X509NameContainer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(X509NameVisitor)
    }
}

struct X509NameVisitor;

impl<'de> Visitor<'de> for X509NameVisitor {
    type Value = X509NameContainer;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a based64-encoded DER X.509 name")
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(X509NameContainer(
            X509Name::from_der(
                base64::engine::general_purpose::STANDARD
                    .decode(v)
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?
                    .as_slice(),
            )
            .map_err(|e| serde::de::Error::custom(e.to_string()))?,
        ))
    }
}

pub fn create_new_secp521r1_keypair() -> Result<EcKey<Private>> {
    Ok(EcKey::generate(
        EcGroup::from_curve_name(Nid::SECP521R1)?.as_ref(),
    )?)
}
