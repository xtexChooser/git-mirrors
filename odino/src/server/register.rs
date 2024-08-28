use actix_web::{
    web::{self, Data, Html},
    Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::user::group::UserGroup;

use super::{HttpResult, IdServer};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct RegisterConfig {
    #[serde(default)]
    pub requires_group: UserGroup,
    #[serde(default)]
    pub allow_invitation: bool,
}

pub async fn serve_get(server: Data<IdServer>) -> HttpResult<impl Responder> {
    Ok(Html::new(server.template.render("page/register", &json!({}))?))
}

pub async fn serve_post(server: Data<IdServer>) -> HttpResult<impl Responder> {
    Ok(Html::new(server.template.render("page/register", &json!({}))?))
}
