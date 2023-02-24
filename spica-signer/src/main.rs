use std::{net::SocketAddr, str::FromStr};

use anyhow::{Error, Result};
use spica_signer::{config::get_config, routes::make_router};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt().json().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = make_router().await;

    let listen_addr = SocketAddr::from_str(&get_config().listen_addr)?;
    info!(addr = listen_addr.to_string(), "listening");
    axum::Server::try_bind(&listen_addr)?
        .serve(app.into_make_service())
        .await
        .map_err(Error::from)?;

    Ok(())
}
