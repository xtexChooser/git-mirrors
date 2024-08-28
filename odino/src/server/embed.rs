use std::{
    marker::PhantomData,
    sync::Arc,
    time::{Duration, SystemTime},
};

use actix_web::{
    body::BoxBody,
    dev::{
        AppService, HttpServiceFactory, ResourceDef, Service, ServiceFactory,
        ServiceRequest, ServiceResponse,
    },
    http::{
        header::{self, HttpDate},
        Method,
    },
    HttpMessage, HttpResponse,
};
use anyhow::Result;
use futures::future::LocalBoxFuture;
use mime::Mime;
use mime_guess::MimeGuess;

pub struct EmbedAssets<E>(pub String, pub PhantomData<E>)
where
    E: 'static + rust_embed::RustEmbed;

impl<E> HttpServiceFactory for EmbedAssets<E>
where
    E: 'static + rust_embed::RustEmbed,
{
    fn register(self, config: &mut AppService) {
        let resource_def = if config.is_root() {
            ResourceDef::root_prefix(&self.0)
        } else {
            ResourceDef::prefix(&self.0)
        };
        config.register_service(resource_def, None, self, None)
    }
}

impl<E> ServiceFactory<ServiceRequest> for EmbedAssets<E>
where
    E: 'static + rust_embed::RustEmbed,
{
    type Response = ServiceResponse;
    type Error = actix_web::Error;
    type Config = ();
    type Service = EmbedAssetsService<E>;
    type InitError = ();
    type Future =
        LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: ()) -> Self::Future {
        let path = format!("{}/", self.0);
        Box::pin(async move {
            Ok(EmbedAssetsService::<E>(Arc::new(path), PhantomData))
        })
    }
}

#[derive(Clone)]
pub struct EmbedAssetsService<E>(Arc<String>, PhantomData<E>)
where
    E: 'static + rust_embed::RustEmbed;

impl<E> Service<ServiceRequest> for EmbedAssetsService<E>
where
    E: 'static + rust_embed::RustEmbed,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::always_ready!();

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = self.0.clone();
        Box::pin(async move {
            if !matches!(*req.method(), Method::HEAD | Method::GET) {
                let mut resp = HttpResponse::MethodNotAllowed();
                resp.insert_header((
                    header::CONTENT_TYPE,
                    mime::TEXT_PLAIN_UTF_8,
                ));
                return Ok(req.into_response(resp));
            }
            let path = req.path().trim_start_matches(path.as_str());

            match E::get(path) {
                Some(f) => {
                    let hash = hex::encode(f.metadata.sha256_hash());

                    if req
                        .headers()
                        .get(header::IF_NONE_MATCH)
                        .and_then(|v| v.to_str().ok())
                        .map(|v| v.eq_ignore_ascii_case(&hash))
                        .unwrap_or(false)
                    {
                        return Ok(
                            req.into_response(HttpResponse::NotModified())
                        );
                    }

                    let mut resp = HttpResponse::Ok();
                    if let Some(lm) = f.metadata.last_modified() {
                        if let Some(header::IfUnmodifiedSince(since)) =
                            req.get_header()
                        {
                            let since: SystemTime = since.into();
                            if let Ok(since) =
                                since.duration_since(SystemTime::UNIX_EPOCH)
                            {
                                if lm <= since.as_secs() {
                                    return Ok(req.into_response(
                                        HttpResponse::NotModified(),
                                    ));
                                }
                            }
                        }

                        if let Some(lm) = SystemTime::UNIX_EPOCH
                            .checked_add(Duration::from_secs(lm))
                        {
                            resp.insert_header((
                                header::LAST_MODIFIED,
                                HttpDate::from(lm).to_string(),
                            ));
                        }
                    }

                    let mime = transform_to_utf8(
                        MimeGuess::from_path(path).first_or_octet_stream(),
                    );
                    let data = f.data.into_owned();
                    resp.content_type(mime.as_ref())
                        .insert_header((header::ETAG, hash));

                    Ok(req.into_response(resp.body(data)))
                }
                None => Ok(req.into_response(HttpResponse::NotFound())),
            }
        })
    }
}

fn transform_to_utf8(ct: Mime) -> Mime {
    if ct == mime::APPLICATION_JAVASCRIPT {
        mime::APPLICATION_JAVASCRIPT_UTF_8
    } else if ct == mime::TEXT_HTML {
        mime::TEXT_HTML_UTF_8
    } else if ct == mime::TEXT_CSS {
        mime::TEXT_CSS_UTF_8
    } else if ct == mime::TEXT_PLAIN {
        mime::TEXT_PLAIN_UTF_8
    } else if ct == mime::TEXT_CSV {
        mime::TEXT_CSV_UTF_8
    } else if ct == mime::TEXT_TAB_SEPARATED_VALUES {
        mime::TEXT_TAB_SEPARATED_VALUES_UTF_8
    } else {
        ct
    }
}
