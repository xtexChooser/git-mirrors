use askama_axum::IntoResponse;
use axum::{extract::Host, routing::get, Json, Router};
use serde_json::json;

use crate::cert::get_certs;

pub async fn make_router() -> Router {
    Router::new()
        .route("/.well-known/nodeinfo", get(well_known))
        .route("/nodeinfo", get(nodeinfo))
}

async fn well_known(Host(host): Host) -> impl IntoResponse {
    (
        [(
            "content-type",
            "application/json; profile=\"http://nodeinfo.diaspora.software/ns/schema/2.1#\"",
        )],
        Json(json!(
        {
            "links": [
                {
                    "rel": "http://nodeinfo.diaspora.software/ns/schema/2.1",
                    "href": format!("https://{host}/nodeinfo")
                }
            ]
        }
        )),
    )
}

async fn nodeinfo() -> impl IntoResponse {
    (
        [(
            "content-type",
            "application/json; profile=\"http://nodeinfo.diaspora.software/ns/schema/2.1#\"",
        )],
        Json(json!(
        {
            "version": "2.1",
            "software": {
                "name": "SPICA",
                "version": env!("CARGO_PKG_VERSION"),
                "repository": "https://codeberg.org/XTEX-VNET/spica",
            },
            "protocols": [
                "x-spica-signer"
            ],
            "services": {
                "inbound": [],
                "outbound": [],
            },
            "metadata": {
                "supported_certs": serde_json::Value::Array(get_certs().keys().to_owned()
                        .map(|s| serde_json::Value::String(s.to_string())).collect())
            }
        }
        )),
    )
}
