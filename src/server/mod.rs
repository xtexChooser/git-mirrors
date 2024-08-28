use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    marker::PhantomData,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

use actix_web::{
    middleware::ErrorHandlers,
    web::{self, Data},
    App, HttpResponse, HttpServer, ResponseError,
};
use anyhow::{Context, Result};
use embed::EmbedAssets;
use handlebars::Handlebars;
use itertools::Itertools;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use register::RegisterConfig;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{error, info};

use crate::{
    database,
    idp::{IdProvider, IdProviderConfig, IdProviderFactory, IdProviderId},
};

pub mod embed;
pub mod frontend;
pub mod register;

pub struct IdServer {
    pub config: ServerConfig,
    pub database: PgPool,
    pub providers: HashMap<IdProviderId, IdProvider>,
    pub template: Arc<Handlebars<'static>>,
    pub csrng: Mutex<ChaChaRng>,
}

impl IdServer {
    pub async fn new(config_path: &Path, frontend_dev: bool) -> Result<Self> {
        let config =
            std::fs::read_to_string(config_path).with_context(|| {
                format!("Read configuration file from {:?}", config_path)
            })?;
        let config = toml::from_str::<ServerConfig>(&config)
            .with_context(|| "Parse TOML configuration")?;

        let database = database::connect(&config.database)
            .await
            .context("connect to database")?;

        let mut template = Handlebars::new();
        template.set_strict_mode(true);
        template.set_dev_mode(frontend_dev);
        frontend::register_frontend(
            &mut template,
            config.frontend.overlay.as_ref(),
        )?;
        let template = Arc::new(template);

        let providers = config
            .idp
            .iter()
            .map(|(key, value)| (IdProviderId::from(key.as_str()), value))
            .map(|(id, idp)| idp.to_idp(id).map(|idp| (id, idp)))
            .try_collect()?;

        let csrng = Mutex::new(ChaChaRng::from_entropy());

        Ok(Self {
            config,
            database,
            providers,
            template,
            csrng,
        })
    }

    pub async fn run(self) -> Result<()> {
        let server = Arc::new(self);
        _ = ID_SERVER.set(server.clone());
        database::schema::check(&server.database).await?;

        let mut http_server = {
            let server_data = Data::from(server.clone());
            let template_data = Data::from(server.template.clone());
            HttpServer::new(move || {
                let mut app = App::new()
                    .app_data(server_data.clone())
                    .app_data(template_data.clone())
                    .wrap(
                        ErrorHandlers::new()
                            .default_handler(frontend::handle_error),
                    )
                    .service(EmbedAssets::<frontend::BuiltinAssets>(
                        "/fec".to_string(),
                        PhantomData,
                    ))
                    .route("/", web::get().to(frontend::serve_index))
                    .service(
                        web::resource("/register")
                            .get(register::serve_get)
                            .post(register::serve_post),
                    );
                if let Some(overlay) =
                    server_data.config.frontend.overlay.as_ref()
                {
                    app = app.service(
                        actix_files::Files::new("/feo", overlay)
                            .prefer_utf8(true)
                            .use_etag(true)
                            .use_last_modified(true),
                    );
                }
                app
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

static ID_SERVER: OnceLock<Arc<IdServer>> = OnceLock::new();
impl IdServer {
    pub fn get() -> &'static Self {
        ID_SERVER
            .get()
            .expect("IdServer::get must be called after initialization")
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ServerConfig {
    pub database: database::Config,
    pub listen: ListenConfig,
    #[serde(default)]
    pub frontend: FrontendConfig,
    pub site: SiteConfig,
    pub idp: HashMap<String, IdProviderConfig>,
    pub register: RegisterConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ListenConfig {
    #[serde(default)]
    pub unix: Option<PathBuf>,
    #[serde(default)]
    pub addr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct FrontendConfig {
    #[serde(default)]
    pub overlay: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SiteConfig {
    pub name: String,
    #[serde(default)]
    pub path: String,
    pub description: String,
}

#[derive(Debug)]
pub enum HttpError {
    Unauthorized,
    Anyhow(anyhow::Error),
}

impl<E> From<E> for HttpError
where
    E: std::error::Error + Send + Sync + 'static,
{
    #[cold]
    fn from(error: E) -> Self {
        Self::Anyhow(error.into())
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpError::Anyhow(error) => Display::fmt(error, f),
            _ => Debug::fmt(self, f),
        }
    }
}

pub type HttpResult<R> = Result<R, HttpError>;

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HttpError::Unauthorized => HttpResponse::Unauthorized().finish(),
            HttpError::Anyhow(error) => {
                for cause in error.chain() {
                    if let Some(_) = cause.downcast_ref::<serde_json::Error>() {
                        return HttpResponse::BadRequest().finish();
                    }
                }
                error!(%error, "internal server error occurred");
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
