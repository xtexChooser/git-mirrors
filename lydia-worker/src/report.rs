use anyhow::{anyhow, Result};
use base64::Engine;
use serde::Serialize;
use tracing::info;

use crate::Bot;

pub async fn update_report(bot: &Bot, path: &str, message: &str, content: &str) -> Result<()> {
    let token = &bot.secrets.cb.token;
    let url = format!(
        "https://codeberg.org/api/v1/repos/Lydia/{}/contents/{}?token={}",
        if bot.is_dev() {
            "test-report"
        } else {
            "report"
        },
        path,
        token
    );
    info!(path, commit_msg = message, "upload report");
    let mut upload_body = serde_json::json!(
        {
          "branch": "pages",
          "content": base64::engine::general_purpose::STANDARD.encode(content),
          "message": format!("[{}] {}", bot.env, message),
          "sha": "string"
        }
    );
    let get_resp = bot.http.get(&url).send().await?;

    let resp = if get_resp.status().as_u16() == 404 {
        // new file
        info!(path, "create new report file");
        bot.http.post(&url).json(&upload_body).send().await?
    } else {
        // exists file
        upload_body["sha"] = get_resp
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?["sha"]
            .to_owned();
        bot.http.put(&url).json(&upload_body).send().await?
    }
    .error_for_status()?
    .json::<serde_json::Value>()
    .await?;

    let sha = resp["commit"]["sha"]
        .as_str()
        .ok_or_else(|| anyhow!("repo/contents .commit.sha is not str"))?;
    info!(path, sha, "report uploaded");
    Ok(())
}

#[inline]
pub async fn update_json_report<T>(bot: &Bot, path: &str, message: &str, content: &T) -> Result<()>
where
    T: ?Sized + Serialize,
{
    update_report(bot, path, message, &serde_json::to_string(content)?).await
}
