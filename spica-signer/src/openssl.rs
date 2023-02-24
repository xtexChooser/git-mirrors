use std::collections::HashMap;

use anyhow::Result;
use lazy_static::lazy_static;
use openssl::{
    conf::{Conf, ConfMethod},
    x509::{X509Extension, X509v3Context},
};

lazy_static! {
    static ref CONF: Conf = Conf::new(ConfMethod::default()).unwrap();
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
