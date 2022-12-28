use std::{
    fs::read_to_string,
    path::PathBuf,
    sync::{Mutex, MutexGuard},
};

use anyhow::{anyhow, bail, Result};
use etcd_client::{Certificate, Client, ConnectOptions, Identity, TlsOptions};
use serde::Deserialize;

use crate::config::get_config;

pub static mut ETCD_CLIENT: Option<Mutex<Client>> = None;

pub async fn init_etcd() -> Result<()> {
    let config = &get_config()?.etcd;
    let endpoints = config.endpoints.clone();
    let mut options = ConnectOptions::new();
    if let Some(auth) = config.auth.clone() {
        options = options.with_user(auth.user, auth.password);
    }
    if let Some(tls) = config.tls.clone() {
        let mut tls_config = TlsOptions::new();
        if let Some(domain) = tls.domain {
            tls_config = tls_config.domain_name(domain);
        }
        if let Some(file) = tls.ca_cert_file {
            tls_config = tls_config.ca_certificate(Certificate::from_pem(
                pem::parse(read_to_string(PathBuf::from(file)).unwrap())?.contents,
            ));
        }
        if let Some(cert_file) = tls.client_cert_file {
            if let Some(key_file) = tls.client_key_file {
                tls_config = tls_config.identity(Identity::from_pem(
                    pem::parse(read_to_string(PathBuf::from(cert_file)).unwrap())?.contents,
                    pem::parse(read_to_string(PathBuf::from(key_file)).unwrap())?.contents,
                ))
            } else {
                bail!("client cert file defined but no key file")
            }
        } else if let Some(_) = tls.client_key_file {
            bail!("client key file defined but no cert file")
        }
        options = options.with_tls(tls_config);
    }
    info!("connecting to etcd, endpoints: {:?}", endpoints);
    info!("etcd options: {:?}", options);
    unsafe {
        ETCD_CLIENT = Some(Mutex::new(Client::connect(endpoints, Some(options)).await?));
    }
    Ok(())
}

pub fn get_etcd_client() -> Result<MutexGuard<'static, Client>> {
    Ok(unsafe { ETCD_CLIENT.as_mut() }
        .ok_or(anyhow!("etcd client not initialized"))?
        .lock()
        .map_err(|e| anyhow!("failed to lock etcd client {}", e))?)
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EtcdConfig {
    pub endpoints: Vec<String>,
    pub auth: Option<EtcdAuthConfig>,
    pub tls: Option<EtcdTlsConfig>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EtcdAuthConfig {
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EtcdTlsConfig {
    pub domain: Option<String>,
    pub ca_cert_file: Option<String>,
    pub client_cert_file: Option<String>,
    pub client_key_file: Option<String>,
}
