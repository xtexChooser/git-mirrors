use std::{cell::OnceCell, fs::read_to_string, path::PathBuf, sync::Mutex};

use anyhow::{bail, Result};
use etcd_client::{Certificate, Client, ConnectOptions, Identity, TlsOptions};

use crate::config::CONFIG;

pub static ETCD_CLIENT: Mutex<OnceCell<Client>> = Mutex::new(OnceCell::new());

pub async fn init_etcd() -> Result<()> {
    let config_lock = CONFIG.lock().unwrap();
    let config = &config_lock.get().unwrap().etcd;
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
                bail!("Client cert file defined but no key file")
            }
        } else if let Some(_) = tls.client_key_file {
            bail!("Client key file defined but no cert file")
        }
        options = options.with_tls(tls_config);
    }
    info!("connecting to etcd, endpoints: {:?}", endpoints);
    info!("etcd options: {:?}", options);
    ETCD_CLIENT
        .lock()
        .unwrap()
        .set(Client::connect(endpoints, Some(options)).await?)
        .ok()
        .unwrap();
    Ok(())
}
