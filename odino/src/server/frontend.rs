use std::path::PathBuf;

use actix_web::{
    dev::ServiceResponse,
    http::{
        header::{
            self, CacheControl, CacheDirective, ContentType, TryIntoHeaderValue,
        },
        StatusCode,
    },
    middleware::ErrorHandlerResponse,
    web::{Data, Html},
    Responder,
};
use anyhow::Result;
use handlebars::{handlebars_helper, Handlebars};
use rust_embed::Embed;
use serde_json::json;
use tracing::error;

use super::{HttpResult, IdServer, ID_SERVER};

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/frontend"]
#[exclude = "assets/*"]
pub struct BuiltinTemplates;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/frontend/assets"]
pub struct BuiltinAssets;

pub fn register_frontend(
    registry: &mut Handlebars,
    overlay: Option<&PathBuf>,
) -> Result<()> {
    registry.register_helper("site_name", Box::new(helper_site_name));
    registry.register_helper("site_path", Box::new(helper_site_path));
    registry.register_helper("site", Box::new(helper_site));
    registry.register_helper("odino_version", Box::new(helper_odino_version));

    registry
        .register_embed_templates_with_extension::<BuiltinTemplates>(".hbs")?;
    if let Some(overlay) = overlay {
        registry.register_templates_directory(overlay, Default::default())?;
    }
    Ok(())
}

handlebars_helper!(helper_site_name: |*_args| ID_SERVER.get().unwrap().config.site.name.as_str());
handlebars_helper!(helper_site_path: |*_args| ID_SERVER.get().unwrap().config.site.path.as_str());
handlebars_helper!(helper_site: |*_args| serde_json::to_value(&ID_SERVER.get().unwrap().config.site).unwrap());
handlebars_helper!(helper_odino_version: |*_args| env!("CARGO_PKG_VERSION"));

pub async fn serve_index(server: Data<IdServer>) -> HttpResult<impl Responder> {
    Ok(Html::new(server.template.render("page/index", &json!({}))?))
}

pub fn handle_error<B>(
    res: ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (req, mut res) = res.into_parts();
    let status = res.status();

    if status != StatusCode::NOT_FOUND {
        res.headers_mut().insert(
            header::CACHE_CONTROL,
            CacheControl(vec![CacheDirective::NoCache])
                .try_into_value()
                .unwrap(),
        );
    }

    if req
        .headers()
        .get(header::ACCEPT)
        .and_then(|s| s.to_str().ok())
        .map(|s| s.contains("text/html"))
        .unwrap_or(false)
    {
        let server = ID_SERVER.get().unwrap();
        let mut template = format!("error/{}", status.as_u16());
        if !server.template.has_template(&template) {
            template = "error/default".to_string();
        }

        let error = res.error();
        let html = server.template.render(
            &template,
            &json!({
                "status": status.as_u16(),
                "status-msg": status.canonical_reason(),
                "error": error.map(|e|format!("{e}")).unwrap_or_default(),
            }),
        );

        match html {
            Ok(html) => {
                let mut res = res.set_body(html);
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    ContentType::html().try_into_value().unwrap(),
                );
                let res = ServiceResponse::new(req, res)
                    .map_into_boxed_body()
                    .map_into_right_body();
                return Ok(ErrorHandlerResponse::Response(res));
            }
            Err(error) => {
                error!(?error, %status, "failed to render error page");
            }
        }
    }

    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        req,
        res.map_into_left_body(),
    )))
}
