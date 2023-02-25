use std::{net::SocketAddr, str::FromStr};

use anyhow::{Error, Result};
use spica_signer::{cert::get_certs, config::get_config, routes::make_router};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt().json().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("initializing openssl");
    openssl::init();

    get_certs();

    let listen_addr = SocketAddr::from_str(&get_config().listen_addr)?;
    info!(addr = listen_addr.to_string(), "listening");
    axum::Server::try_bind(&listen_addr)?
        .serve(make_router().await.into_make_service())
        .await
        .map_err(Error::from)?;

    Ok(())
}
