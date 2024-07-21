use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
};

use actix_web::{web::Data, App, HttpServer};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;

use crate::database;

pub struct IdServer {
    pub config: ServerConfig,
    pub database: PgPool,
}

impl IdServer {
    pub async fn new(config_path: &Path) -> Result<Self> {
        let config =
            std::fs::read_to_string(config_path).with_context(|| {
                format!("Read configuration file from {:?}", config_path)
            })?;
        let config = toml::from_str::<ServerConfig>(&config)
            .with_context(|| "Parse TOML configuration")?;

        let database = database::connect(&config.database)
            .await
            .context("connect to database")?;

        Ok(Self { config, database })
    }

    pub async fn run(self) -> Result<()> {
        let server = Arc::new(self);
        database::schema::check(&server.database).await?;

        let mut http_server = {
            let server_data = Data::from(server.clone());
            HttpServer::new(move || {
                App::new().app_data(server_data.clone())
                // .route("/", web::get().to(index))
            })
        };
        if let Some(path) = &server.config.listen.unix {
            info!(?path, "listening unix socket");
            http_server = http_server.bind_uds(path)?;
        }
        if let Some(addr) = &server.config.listen.addr {
            info!(addr, "listening h2c socket");
            http_server =
                http_server.bind_auto_h2c(addr.parse::<SocketAddr>()?)?;
        }
        http_server.run().await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ServerConfig {
    pub database: database::Config,
    pub listen: ListenConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ListenConfig {
    #[serde(default)]
    pub unix: Option<PathBuf>,
    #[serde(default)]
    pub addr: Option<String>,
}
