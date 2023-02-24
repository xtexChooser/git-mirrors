#![feature(let_chains)]

use actix_web::{App, HttpServer};
use anyhow::{Error, Result};
use spica_signer::config::get_config;
use tokio::fs;
use tracing::info;

#[actix_web::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt().json().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let mut server = HttpServer::new(|| App::new());

    if let Some(addr) = &get_config().listen_addr {
        info!(addr, "listening");
        server = server.bind(addr)?;
    }
    #[cfg(unix)]
    if let Some(path) = &get_config().listen_unix {
        info!(path, "listening UDS");
        server = server.bind_uds(path)?;
    }

    let server = server.run();

    #[cfg(unix)]
    if let Some(path) = &get_config().listen_unix && let Some(mode) = &get_config().listen_unix_mode {
        info!(path, mode, "updating UDS file mode");
        fs::set_permissions(path, std::os::unix::fs::PermissionsExt::from_mode(u32::from_str_radix(&mode, 8)?)).await?;
    }

    server.await.map_err(Error::from)?;
    Ok(())
}
